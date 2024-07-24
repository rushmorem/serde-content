#![cfg(feature = "serde")]

mod r#enum;
mod identifier;
mod map;
mod number;
mod seq;
mod r#struct;
mod tests;

use crate::Content;
use crate::Data;
use crate::Error;
use crate::Number;
use crate::HUMAN_READABLE;
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use identifier::Identifier;
use map::Map;
use seq::Seq;
use serde::de;
mod error;
use map::Key;
use serde::de::EnumAccess;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::Visitor;

const UNKNOWN_TYPE_NAME: &str = "<unknown>";

/// Deserializes a value `T` from [`Content`]
pub fn from_content<'de, T>(content: Content<'de>) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    let deserializer = Deserializer::new(content, HUMAN_READABLE);
    T::deserialize(deserializer)
}

/// A structure that deserializes Rust values into [Content].
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Deserializer<'de> {
    content: Content<'de>,
    human_readable: bool,
}

impl<'de> Deserializer<'de> {
    /// Creates a deserializer
    pub const fn new(content: Content<'de>, human_readable: bool) -> Self {
        Self {
            content,
            human_readable,
        }
    }
}

#[cfg(feature = "std")]
impl<'de> serde::de::IntoDeserializer<'de, Error> for Deserializer<'de> {
    type Deserializer = Deserializer<'de>;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

#[cfg(feature = "std")]
impl<'de> serde::de::IntoDeserializer<'de, Error> for Content<'de> {
    type Deserializer = Deserializer<'de>;

    fn into_deserializer(self) -> Self::Deserializer {
        Deserializer::new(self, HUMAN_READABLE)
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Unit => visitor.visit_unit(),
            Content::Bool(v) => visitor.visit_bool(v),
            Content::Number(n) => match n {
                Number::I8(v) => visitor.visit_i8(v),
                Number::U8(v) => visitor.visit_u8(v),
                Number::I16(v) => visitor.visit_i16(v),
                Number::U16(v) => visitor.visit_u16(v),
                Number::I32(v) => visitor.visit_i32(v),
                Number::U32(v) => visitor.visit_u32(v),
                Number::F32(v) => visitor.visit_f32(v),
                Number::I64(v) => visitor.visit_i64(v),
                Number::U64(v) => visitor.visit_u64(v),
                Number::F64(v) => visitor.visit_f64(v),
                Number::I128(v) => visitor.visit_i128(v),
                Number::U128(v) => visitor.visit_u128(v),
            },
            Content::Char(v) => visitor.visit_char(v),
            Content::String(v) => match v {
                Cow::Borrowed(v) => visitor.visit_borrowed_str(v),
                Cow::Owned(v) => visitor.visit_string(v),
            },
            Content::Bytes(v) => match v {
                Cow::Borrowed(v) => visitor.visit_borrowed_bytes(v),
                Cow::Owned(v) => visitor.visit_byte_buf(v),
            },
            Content::Seq(v) => visitor.visit_seq(Seq::new(v, self.human_readable)),
            Content::Map(v) => visitor.visit_map(Map::from((v, self.human_readable))),
            Content::Option(v) => match v {
                Some(v) => {
                    let deserializer = Deserializer::new(*v, self.human_readable);
                    visitor.visit_some(deserializer)
                }
                None => visitor.visit_none(),
            },
            Content::Struct(v) => match v.data {
                Data::Unit => visitor.visit_unit_struct(v.name),
                Data::NewType { value } => {
                    let deserializer = Deserializer::new(value, self.human_readable);
                    visitor.visit_newtype_struct_with_name(v.name, deserializer)
                }
                Data::Tuple { values } => {
                    visitor.visit_tuple_struct(v.name, Seq::new(values, self.human_readable))
                }
                Data::Struct { fields } => {
                    let len = fields.len();
                    let mut field_names = Vec::with_capacity(len);
                    let mut vec = Vec::with_capacity(len);
                    for (index, (key, value)) in fields.into_iter().enumerate() {
                        field_names.push(key);
                        let key = Key::Identifier(Identifier::new(key, index as u64));
                        vec.push((key, value));
                    }
                    let data = Map::new(vec, self.human_readable);
                    visitor.visit_struct(v.name, &field_names, data)
                }
            },
            Content::Enum(v) => r#enum::visit_enum(v, self.human_readable, visitor),
            Content::Tuple(v) => visitor.visit_tuple(Seq::new(v, self.human_readable)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Bool(v) => visitor.visit_bool(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::I8(v)) => visitor.visit_i8(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::I16(v)) => visitor.visit_i16(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::I32(v)) => visitor.visit_i32(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::I64(v)) => visitor.visit_i64(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::I128(v)) => visitor.visit_i128(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::U8(v)) => visitor.visit_u8(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::U16(v)) => visitor.visit_u16(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::U32(v)) => visitor.visit_u32(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::U64(v)) => visitor.visit_u64(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::U128(v)) => visitor.visit_u128(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::F32(v)) => visitor.visit_f32(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Number(Number::F64(v)) => visitor.visit_f64(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Char(v) => visitor.visit_char(v),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::String(v) => match v {
                Cow::Borrowed(v) => visitor.visit_borrowed_str(v),
                Cow::Owned(v) => visitor.visit_string(v),
            },
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Bytes(v) => match v {
                Cow::Borrowed(v) => visitor.visit_borrowed_bytes(v),
                Cow::Owned(v) => visitor.visit_byte_buf(v),
            },
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Option(v) => match v {
                Some(v) => {
                    let deserializer = Deserializer::new(*v, self.human_readable);
                    visitor.visit_some(deserializer)
                }
                None => visitor.visit_none(),
            },
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Unit => visitor.visit_unit(),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Struct(v) => match v.data {
                Data::Unit => visitor.visit_unit_struct(name),
                data => Err(data.invalid_struct_type(&visitor)),
            },
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Struct(v) => match v.data {
                Data::NewType { value } => {
                    let deserializer = Deserializer::new(value, self.human_readable);
                    visitor.visit_newtype_struct_with_name(v.name, deserializer)
                }
                data => Err(data.invalid_struct_type(&visitor)),
            },
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Seq(v) => visitor.visit_seq(Seq::new(v, self.human_readable)),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Tuple(v) => visitor.visit_tuple(Seq::new(v, self.human_readable)),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Struct(v) => match v.data {
                Data::Tuple { values } => {
                    visitor.visit_tuple_struct(name, Seq::new(values, self.human_readable))
                }
                data => Err(data.invalid_struct_type(&visitor)),
            },
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Map(v) => visitor.visit_map(Map::from((v, self.human_readable))),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let field_names = fields;
        match self.content {
            Content::Struct(v) => match v.data {
                Data::Struct { fields } => visitor.visit_struct(
                    name,
                    field_names,
                    Map::from((fields, self.human_readable)),
                ),
                data => Err(data.invalid_struct_type(&visitor)),
            },
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Enum(v) => r#enum::visit_enum(v, self.human_readable, visitor),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::String(v) => match v {
                Cow::Borrowed(v) => visitor.visit_borrowed_str(v),
                Cow::Owned(v) => visitor.visit_string(v),
            },
            Content::Enum(v) => visitor.visit_borrowed_str(v.variant),
            _ => Err(self.content.invalid_type(&visitor)),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn is_human_readable(&self) -> bool {
        self.human_readable
    }
}

impl<'de> de::Deserialize<'de> for Content<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ContentVisitor)
    }
}

struct ContentVisitor;

impl<'de> Visitor<'de> for ContentVisitor {
    type Value = Content<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any value")
    }

    fn visit_bool<F>(self, value: bool) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Bool(value))
    }

    fn visit_i8<F>(self, value: i8) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Number(Number::I8(value)))
    }

    fn visit_i16<F>(self, value: i16) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Number(Number::I16(value)))
    }

    fn visit_i32<F>(self, value: i32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Number(Number::I32(value)))
    }

    fn visit_i64<F>(self, value: i64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Number(Number::I64(value)))
    }

    fn visit_u8<F>(self, value: u8) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Number(Number::U8(value)))
    }

    fn visit_u16<F>(self, value: u16) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Number(Number::U16(value)))
    }

    fn visit_u32<F>(self, value: u32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Number(Number::U32(value)))
    }

    fn visit_u64<F>(self, value: u64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Number(Number::U64(value)))
    }

    fn visit_f32<F>(self, value: f32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Number(Number::F32(value)))
    }

    fn visit_f64<F>(self, value: f64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Number(Number::F64(value)))
    }

    fn visit_char<F>(self, value: char) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Char(value))
    }

    fn visit_str<F>(self, value: &str) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::String(Cow::Owned(value.to_owned())))
    }

    fn visit_borrowed_str<F>(self, value: &'de str) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::String(Cow::Borrowed(value)))
    }

    fn visit_string<F>(self, value: String) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::String(Cow::Owned(value)))
    }

    fn visit_bytes<F>(self, value: &[u8]) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Bytes(Cow::Owned(value.to_owned())))
    }

    fn visit_borrowed_bytes<F>(self, value: &'de [u8]) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Bytes(Cow::Borrowed(value)))
    }

    fn visit_byte_buf<F>(self, value: Vec<u8>) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Bytes(Cow::Owned(value)))
    }

    fn visit_unit<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Unit)
    }

    fn visit_none<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Option(None))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(deserializer).map(|v| Content::Option(Some(Box::new(v))))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let data = r#struct::Visitor.visit_newtype_struct(deserializer)?;
        Ok(Content::Struct(Box::new(data)))
    }

    fn visit_newtype_struct_with_name<D>(
        self,
        name: &'static str,
        deserializer: D,
    ) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let data = r#struct::Visitor.visit_newtype_struct_with_name(name, deserializer)?;
        Ok(Content::Struct(Box::new(data)))
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let len = visitor.size_hint().unwrap_or_default();
        let mut vec = Vec::with_capacity(len);
        while let Some(e) = visitor.next_element()? {
            vec.push(e);
        }
        Ok(Content::Seq(vec))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let len = visitor.size_hint().unwrap_or_default();
        let mut vec = Vec::with_capacity(len);
        while let Some(kv) = visitor.next_entry()? {
            vec.push(kv);
        }
        Ok(Content::Map(vec))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Content::Number(Number::I128(v)))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Content::Number(Number::U128(v)))
    }

    fn visit_unit_struct<E>(self, name: &'static str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let data = r#struct::Visitor.visit_unit_struct(name)?;
        Ok(Content::Struct(Box::new(data)))
    }

    fn visit_tuple<A>(self, mut visitor: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let len = visitor.size_hint().unwrap_or_default();
        let mut vec = Vec::with_capacity(len);
        while let Some(e) = visitor.next_element()? {
            vec.push(e);
        }
        Ok(Content::Tuple(vec))
    }

    fn visit_tuple_struct<A>(self, name: &'static str, data: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let data = r#struct::Visitor.visit_tuple_struct(name, data)?;
        Ok(Content::Struct(Box::new(data)))
    }

    fn visit_struct<A>(
        self,
        name: &'static str,
        fields: &[&'static str],
        data: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let data = r#struct::Visitor.visit_struct(name, fields, data)?;
        Ok(Content::Struct(Box::new(data)))
    }

    fn visit_unit_variant<A>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        data: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        let data = r#enum::Visitor.visit_unit_variant(name, variant_index, variant, data)?;
        Ok(Content::Enum(Box::new(data)))
    }

    fn visit_newtype_variant<A>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        data: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        let data = r#enum::Visitor.visit_newtype_variant(name, variant_index, variant, data)?;
        Ok(Content::Enum(Box::new(data)))
    }

    fn visit_tuple_variant<A>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
        data: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let data = r#enum::Visitor.visit_tuple_variant(name, variant_index, variant, len, data)?;
        Ok(Content::Enum(Box::new(data)))
    }

    fn visit_struct_variant<A>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        fields: &[&'static str],
        data: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let data =
            r#enum::Visitor.visit_struct_variant(name, variant_index, variant, fields, data)?;
        Ok(Content::Enum(Box::new(data)))
    }
}
