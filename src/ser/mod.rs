#![cfg(feature = "serde")]

mod r#enum;
mod map;
mod number;
mod seq;
mod r#struct;
mod tests;
mod tuple;

use crate::Data;
use crate::Error;
use crate::Number;
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt;
use map::Map;
use r#enum::Enum;
use r#struct::Struct;
use seq::Seq;
use serde::ser;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::ser::SerializeTuple;
use tuple::Tuple;

type Content = super::Content<'static>;

/// Convert a `T` into `Content` which is an enum that can represent any valid Rust data.
pub fn to_content<T>(value: T) -> Result<Content, Error>
where
    T: ser::Serialize,
{
    value.serialize(Serializer::new(false))
}

/// A structure for serialising Rust values into [crate::Content].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Serializer {
    human_readable: bool,
}

impl Serializer {
    /// Creates a serializer
    pub const fn new(human_readable: bool) -> Self {
        Self { human_readable }
    }
}

impl ser::Serializer for Serializer {
    type Ok = Content;
    type Error = Error;

    type SerializeSeq = Seq;
    type SerializeTuple = Tuple;
    type SerializeTupleStruct = Struct;
    type SerializeTupleVariant = Enum;
    type SerializeMap = Map;
    type SerializeStruct = Struct;
    type SerializeStructVariant = Enum;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Error> {
        Ok(Content::Bool(value))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::I8(value)))
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::I16(value)))
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::I32(value)))
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::I64(value)))
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::I128(value)))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::U8(value)))
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::U16(value)))
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::U32(value)))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::U64(value)))
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::U128(value)))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::F32(value)))
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Error> {
        Ok(Content::Number(Number::F64(value)))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Error> {
        Ok(Content::Char(value))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Error> {
        Ok(Content::String(Cow::Owned(value.to_owned())))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Error> {
        Ok(Content::Bytes(Cow::Owned(value.to_owned())))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Error> {
        Ok(Content::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Error> {
        Ok(Content::Struct(Box::new(super::Struct {
            name,
            data: Data::Unit,
        })))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Error> {
        Ok(Content::Enum(Box::new(super::Enum {
            name,
            variant_index,
            variant,
            data: Data::Unit,
        })))
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok, Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Ok(Content::Struct(Box::new(super::Struct {
            name,
            data: Data::NewType {
                value: value.serialize(self)?,
            },
        })))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Ok(Content::Enum(Box::new(super::Enum {
            name,
            variant_index,
            variant,
            data: Data::NewType {
                value: value.serialize(self)?,
            },
        })))
    }

    fn serialize_none(self) -> Result<Self::Ok, Error> {
        Ok(Content::Option(None))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Error>
    where
        T: ?Sized + ser::Serialize,
    {
        let content = value.serialize(self)?;
        Ok(Content::Option(Some(Box::new(content))))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Ok(Seq::new(
            Vec::with_capacity(len.unwrap_or_default()),
            self.human_readable,
        ))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
        Ok(Tuple::new(Vec::with_capacity(len), self.human_readable))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        let st = super::Struct {
            name,
            data: Data::Tuple {
                values: Vec::with_capacity(len),
            },
        };
        Ok(Struct::new(st, self.human_readable))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        let en = super::Enum {
            name,
            variant_index,
            variant,
            data: Data::Tuple {
                values: Vec::with_capacity(len),
            },
        };
        Ok(Enum::new(en, self.human_readable))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Ok(Map::new(
            Vec::with_capacity(len.unwrap_or_default()),
            self.human_readable,
        ))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        let st = super::Struct {
            name,
            data: Data::Struct {
                fields: Vec::with_capacity(len),
            },
        };
        Ok(Struct::new(st, self.human_readable))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        let en = super::Enum {
            name,
            variant_index,
            variant,
            data: Data::Struct {
                fields: Vec::with_capacity(len),
            },
        };
        Ok(Enum::new(en, self.human_readable))
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + fmt::Display,
    {
        Ok(Content::String(Cow::Owned(value.to_string())))
    }

    fn is_human_readable(&self) -> bool {
        self.human_readable
    }
}

impl ser::Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Content::Unit => serializer.serialize_unit(),
            Content::Bool(v) => serializer.serialize_bool(*v),
            Content::Number(v) => v.serialize(serializer),
            Content::Char(v) => serializer.serialize_char(*v),
            Content::String(v) => serializer.serialize_str(v.as_ref()),
            Content::Bytes(v) => serializer.serialize_bytes(v.as_ref()),
            Content::Seq(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for value in v {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            Content::Map(v) => {
                let mut map = serializer.serialize_map(Some(v.len()))?;
                for (key, value) in v {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
            Content::Option(v) => match v {
                Some(v) => serializer.serialize_some(v),
                None => serializer.serialize_none(),
            },
            Content::Struct(v) => v.serialize(serializer),
            Content::Enum(v) => v.serialize(serializer),
            Content::Tuple(v) => {
                let mut tup = serializer.serialize_tuple(v.len())?;
                for value in v {
                    tup.serialize_element(value)?;
                }
                tup.end()
            }
        }
    }
}
