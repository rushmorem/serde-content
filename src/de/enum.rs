use crate::de::identifier::Identifier;
use crate::de::Map;
use crate::de::Seq;
use crate::Content;
use crate::Data;
use crate::Enum;
use crate::Error;
use crate::HUMAN_READABLE;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use serde::de;
use serde::de::IntoDeserializer;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::VariantAccess;
use serde::Deserialize;

impl<'de> IntoDeserializer<'de, Error> for Enum<'de> {
    type Deserializer = crate::Deserializer<'de>;

    fn into_deserializer(self) -> Self::Deserializer {
        crate::Deserializer::new(Content::Enum(Box::new(self)), HUMAN_READABLE)
    }
}

pub(super) struct Deserializer<'de> {
    enum_box: Box<Enum<'de>>,
    human_readable: bool,
}

impl<'de> Deserializer<'de> {
    pub(super) const fn new(enum_box: Box<Enum<'de>>, human_readable: bool) -> Self {
        Self {
            enum_box,
            human_readable,
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
        let variant = Identifier::new(self.enum_box.variant, self.enum_box.variant_index as u64);
        seed.deserialize(variant).map(|v| (v, self))
    }
}

impl<'de> de::VariantAccess<'de> for Deserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.enum_box.data {
            Data::Unit => Ok(()),
            _ => Err(self.enum_box.data.invalid_enum_type(&"unit variant")),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.enum_box.data {
            Data::NewType { value } => {
                let deserializer = crate::Deserializer::new(value, self.human_readable);
                seed.deserialize(deserializer)
            }
            _ => Err(self.enum_box.data.invalid_enum_type(&"newtype variant")),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.enum_box.data {
            Data::Tuple { values } => visitor.visit_seq(Seq::new(values, self.human_readable)),
            _ => Err(self.enum_box.data.invalid_enum_type(&"tuple variant")),
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
            Data::Struct { fields } => visitor.visit_map(Map::from((fields, self.human_readable))),
            _ => Err(self.enum_box.data.invalid_enum_type(&"struct variant")),
        }
    }
}

enum DataType {
    Unit,
    NewType,
    Tuple,
    Struct,
}

impl Data<'_> {
    const fn typ(&self) -> DataType {
        match self {
            Data::Unit => DataType::Unit,
            Data::NewType { .. } => DataType::NewType,
            Data::Tuple { .. } => DataType::Tuple,
            Data::Struct { .. } => DataType::Struct,
        }
    }

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
                vec.push(*key);
            }
            return vec;
        }

        Vec::new()
    }
}

pub(super) fn visit_enum<'de, V>(
    v: Box<Enum<'de>>,
    human_readable: bool,
    visitor: V,
) -> Result<V::Value, Error>
where
    V: de::Visitor<'de>,
{
    let name = v.name;
    let variant_index = v.variant_index;
    let variant = v.variant;
    let typ = v.data.typ();
    let len = v.data.len();
    let fields = v.data.field_names();
    let data = Deserializer::new(v, human_readable);
    match typ {
        DataType::Unit => visitor.visit_unit_variant(name, variant_index, variant, data),
        DataType::NewType => visitor.visit_newtype_variant(name, variant_index, variant, data),
        DataType::Tuple => visitor.visit_tuple_variant(name, variant_index, variant, len, data),
        DataType::Struct => {
            visitor.visit_struct_variant(name, variant_index, variant, &fields, data)
        }
    }
}

impl<'de> Deserialize<'de> for Enum<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(Visitor)
    }
}

pub(super) struct Visitor;

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
        let variant_access = data.variant::<Content>()?.1;
        variant_access.unit_variant()?;
        Ok(Enum {
            name,
            variant_index,
            variant,
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
        let variant_access = data.variant::<Content>()?.1;
        Ok(Enum {
            name,
            variant_index,
            variant,
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
        struct SeqVisitor;

        impl<'de> de::Visitor<'de> for SeqVisitor {
            type Value = Vec<Content<'de>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Vec<Content>")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let len = visitor.size_hint().unwrap_or_default();
                let mut vec = Vec::with_capacity(len);
                while let Some(content) = visitor.next_element()? {
                    vec.push(content);
                }
                Ok(vec)
            }
        }

        let variant_access = data.variant::<Content>()?.1;
        Ok(Enum {
            name,
            variant_index,
            variant,
            data: Data::Tuple {
                values: variant_access.tuple_variant(len, SeqVisitor)?,
            },
        })
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
        A: de::EnumAccess<'de>,
    {
        struct MapVisitor<'a>(&'a [&'static str]);

        impl<'de> de::Visitor<'de> for MapVisitor<'_> {
            type Value = Vec<(&'static str, Content<'de>)>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut vec = Vec::with_capacity(self.0.len());
                while let Some(key) = map.next_key::<Cow<str>>()? {
                    let value = map.next_value()?;
                    for field_name in self.0 {
                        if *field_name == key.as_ref() {
                            vec.push((*field_name, value));
                            break;
                        }
                    }
                }
                Ok(vec)
            }
        }

        let variant_access = data.variant::<Content>()?.1;
        let visitor = MapVisitor(fields);
        Ok(Enum {
            name,
            variant_index,
            variant,
            data: Data::Struct {
                fields: variant_access.struct_variant(&[], visitor)?,
            },
        })
    }
}
