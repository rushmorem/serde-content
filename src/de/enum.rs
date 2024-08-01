use crate::de::error::Unexpected;
use crate::de::identifier::Identifier;
use crate::de::Map;
use crate::de::Seq;
use crate::Data;
use crate::DataType;
use crate::Enum;
use crate::Error;
use crate::Expected;
use crate::Value;
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use core::marker::PhantomData;
use serde::de;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::VariantAccess as _;
use serde::Deserialize;
use serde::Deserializer as _;

#[cfg(feature = "std")]
impl<'de> serde::de::IntoDeserializer<'de, Error> for Enum<'de> {
    type Deserializer = crate::Deserializer<'de>;

    fn into_deserializer(self) -> Self::Deserializer {
        use crate::Deserializer;

        Deserializer::new(Value::Enum(Box::new(self)))
    }
}

pub(super) struct Deserializer<'de> {
    // The name of the enum we are expecting
    expected: Cow<'static, str>,
    enum_box: Box<Enum<'de>>,
    human_readable: bool,
    coerce_numbers: bool,
}

impl<'de> Deserializer<'de> {
    pub(super) const fn new(
        expected: Cow<'static, str>,
        enum_box: Box<Enum<'de>>,
        human_readable: bool,
        coerce_numbers: bool,
    ) -> Self {
        Self {
            expected,
            enum_box,
            human_readable,
            coerce_numbers,
        }
    }
}

impl<'de> de::EnumAccess<'de> for Deserializer<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = Identifier::new(
            self.enum_box.variant.clone(),
            self.enum_box.variant_index as u64,
        );
        seed.deserialize(variant).map(|v| (v, self))
    }
}

impl<'de> de::VariantAccess<'de> for Deserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.enum_box.data {
            Data::Unit => Ok(()),
            _ => Err(self.enum_box.unexpected(Expected::Enum {
                name: Some(self.expected.into_owned()),
                typ: Some(DataType::Unit),
            })),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.enum_box.data {
            Data::NewType { value } => {
                let deserializer = crate::Deserializer {
                    value,
                    human_readable: self.human_readable,
                    coerce_numbers: self.coerce_numbers,
                };
                seed.deserialize(deserializer)
            }
            _ => Err(self.enum_box.unexpected(Expected::Enum {
                name: Some(self.expected.into_owned()),
                typ: Some(DataType::NewType),
            })),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.enum_box.data {
            Data::Tuple { values } => {
                visitor.visit_seq(Seq::new(values, self.human_readable, self.coerce_numbers))
            }
            _ => Err(self.enum_box.unexpected(Expected::Enum {
                name: Some(self.expected.into_owned()),
                typ: Some(DataType::Tuple),
            })),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.enum_box.data {
            Data::Struct { fields } => visitor.visit_map(Map::from((
                fields,
                self.human_readable,
                self.coerce_numbers,
            ))),
            _ => Err(self.enum_box.unexpected(Expected::Enum {
                name: Some(self.expected.into_owned()),
                typ: Some(DataType::Struct),
            })),
        }
    }
}

impl Data<'_> {
    fn len(&self) -> usize {
        match self {
            Data::Unit => 0,
            Data::NewType { .. } => 1,
            Data::Tuple { values } => values.len(),
            Data::Struct { fields } => fields.len(),
        }
    }

    fn field_names(&self) -> Vec<&'static str> {
        if let Data::Struct { fields } = self {
            let mut vec = Vec::with_capacity(fields.len());
            for (key, _) in fields {
                if let Cow::Borrowed(key) = key {
                    vec.push(*key);
                }
            }
            return vec;
        }

        Vec::new()
    }
}

pub(super) fn visit_enum<'de, V>(
    expected: Cow<'static, str>,
    v: Box<Enum<'de>>,
    human_readable: bool,
    coerce_numbers: bool,
    visitor: V,
) -> Result<V::Value, Error>
where
    V: de::Visitor<'de>,
{
    let variant_index = v.variant_index;
    let typ = v.data.typ();
    let len = v.data.len();
    let fields = v.data.field_names();
    match (v.name.clone(), v.variant.clone()) {
        (Cow::Borrowed(name), Cow::Borrowed(variant)) => {
            let data = Deserializer::new(expected, v, human_readable, coerce_numbers);
            match typ {
                DataType::Unit => visitor.visit_unit_variant(name, variant_index, variant, data),
                DataType::NewType => {
                    visitor.visit_newtype_variant(name, variant_index, variant, data)
                }
                DataType::Tuple => {
                    visitor.visit_tuple_variant(name, variant_index, variant, len, data)
                }
                DataType::Struct => {
                    visitor.visit_struct_variant(name, variant_index, variant, &fields, data)
                }
            }
        }
        _ => match v.data {
            Data::Unit => match v.variant {
                Cow::Borrowed(v) => visitor.visit_borrowed_str(v),
                Cow::Owned(v) => visitor.visit_string(v),
            },
            Data::NewType { value } => visitor.visit_map(Map::from((
                vec![(Value::String(v.variant), value)],
                human_readable,
                coerce_numbers,
            ))),
            Data::Tuple { values } => visitor.visit_map(Map::from((
                vec![(Value::String(v.variant), Value::Seq(values))],
                human_readable,
                coerce_numbers,
            ))),
            Data::Struct { fields } => visitor.visit_map(Map::from((
                vec![(
                    Value::String(v.variant),
                    Value::Map(fields.into_iter().map(|(k, v)| (k.into(), v)).collect()),
                )],
                human_readable,
                coerce_numbers,
            ))),
        },
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Enum<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(Visitor)
    }
}

pub(super) struct Visitor;

impl Visitor {
    pub(super) const fn new() -> Self {
        Self
    }
}

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Enum<'de>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("an enum")
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
        let variant_access = data.variant::<Value>()?.1;
        variant_access.unit_variant()?;
        Ok(Enum {
            name: Cow::Borrowed(name),
            variant_index,
            variant: Cow::Borrowed(variant),
            data: Data::Unit,
        })
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
        let variant_access = data.variant::<Value>()?.1;
        Ok(Enum {
            name: Cow::Borrowed(name),
            variant_index,
            variant: Cow::Borrowed(variant),
            data: Data::NewType {
                value: variant_access.newtype_variant()?,
            },
        })
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
        A: de::EnumAccess<'de>,
    {
        struct SeqVisitor<'a> {
            marker: PhantomData<Value<'a>>,
        }

        impl<'de> de::Visitor<'de> for SeqVisitor<'de> {
            type Value = Vec<Value<'de>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Vec<Value>")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let len = visitor.size_hint().unwrap_or_default();
                let mut vec = Vec::with_capacity(len);
                while let Some(value) = visitor.next_element()? {
                    vec.push(value);
                }
                Ok(vec)
            }
        }

        let variant_access = data.variant::<Value>()?.1;
        Ok(Enum {
            name: Cow::Borrowed(name),
            variant_index,
            variant: Cow::Borrowed(variant),
            data: Data::Tuple {
                values: variant_access.tuple_variant(
                    len,
                    SeqVisitor {
                        marker: PhantomData,
                    },
                )?,
            },
        })
    }

    fn visit_struct_variant<A>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _fields: &[&'static str],
        data: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        struct MapVisitor;

        impl<'de> de::Visitor<'de> for MapVisitor {
            type Value = Vec<(Cow<'static, str>, Value<'de>)>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let len = map.size_hint().unwrap_or_default();
                let mut vec = Vec::with_capacity(len);
                while let Some((key, value)) = map.next_entry()? {
                    vec.push((key, value));
                }
                Ok(vec)
            }
        }

        let variant_access = data.variant::<Value>()?.1;
        let visitor = MapVisitor;
        Ok(Enum {
            name: Cow::Borrowed(name),
            variant_index,
            variant: Cow::Borrowed(variant),
            data: Data::Struct {
                fields: variant_access.struct_variant(&[], visitor)?,
            },
        })
    }
}

pub(super) struct Access<'de> {
    // The name of the enum we are expecting
    pub(super) expected: &'static str,
    pub(super) name: Value<'de>,
    pub(super) data: Option<Value<'de>>,
    pub(super) human_readable: bool,
    pub(super) coerce_numbers: bool,
}

impl<'de> de::EnumAccess<'de> for Access<'de> {
    type Error = Error;
    type Variant = VariantAccess<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let deserializer = crate::Deserializer {
            value: self.name,
            human_readable: self.human_readable,
            coerce_numbers: self.coerce_numbers,
        };
        seed.deserialize(deserializer).map(|v| {
            (
                v,
                VariantAccess {
                    expected: self.expected,
                    data: self.data,
                    human_readable: self.human_readable,
                    coerce_numbers: self.coerce_numbers,
                },
            )
        })
    }
}

pub(super) struct VariantAccess<'de> {
    // The name of the enum we are expecting
    expected: &'static str,
    data: Option<Value<'de>>,
    human_readable: bool,
    coerce_numbers: bool,
}

impl<'de> de::VariantAccess<'de> for VariantAccess<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.data {
            None => Ok(()),
            Some(v) => Err(v.unexpected(Expected::Enum {
                name: Some(self.expected.to_owned()),
                typ: Some(DataType::Unit),
            })),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.data {
            Some(value) => {
                let deserializer = crate::Deserializer {
                    value,
                    human_readable: self.human_readable,
                    coerce_numbers: self.coerce_numbers,
                };
                seed.deserialize(deserializer)
            }
            None => Err(Value::Unit.unexpected(Expected::Enum {
                name: Some(self.expected.to_owned()),
                typ: Some(DataType::NewType),
            })),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.data {
            Some(Value::Seq(seq)) => {
                let deserializer = crate::Deserializer {
                    value: Value::Seq(seq),
                    human_readable: self.human_readable,
                    coerce_numbers: self.coerce_numbers,
                };
                deserializer.deserialize_seq(visitor)
            }
            Some(v) => Err(v.unexpected(Expected::Enum {
                name: Some(self.expected.to_owned()),
                typ: Some(DataType::Tuple),
            })),
            None => Err(Value::Unit.unexpected(Expected::Enum {
                name: Some(self.expected.to_owned()),
                typ: Some(DataType::NewType),
            })),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.data {
            Some(Value::Map(map)) => {
                let deserializer = crate::Deserializer {
                    value: Value::Map(map),
                    human_readable: self.human_readable,
                    coerce_numbers: self.coerce_numbers,
                };
                deserializer.deserialize_map(visitor)
            }
            Some(v) => Err(v.unexpected(Expected::Enum {
                name: Some(self.expected.to_owned()),
                typ: Some(DataType::Struct),
            })),
            None => Err(Value::Unit.unexpected(Expected::Enum {
                name: Some(self.expected.to_owned()),
                typ: Some(DataType::NewType),
            })),
        }
    }
}
