use serde::de::{self, Expected, Unexpected};

use crate::{Content, Data, Number};

impl Content<'_> {
    #[cold]
    pub(super) fn invalid_type<E>(&self, exp: &dyn Expected) -> E
    where
        E: de::Error,
    {
        de::Error::invalid_type(self.unexpected(), exp)
    }

    #[cold]
    fn unexpected(&self) -> Unexpected {
        match self {
            Content::Unit => Unexpected::Unit,
            Content::Bool(v) => Unexpected::Bool(*v),
            Content::Number(n) => match *n {
                Number::I8(n) => Unexpected::Signed(n as i64),
                Number::U8(n) => Unexpected::Unsigned(n as u64),
                Number::I16(n) => Unexpected::Signed(n as i64),
                Number::U16(n) => Unexpected::Unsigned(n as u64),
                Number::I32(n) => Unexpected::Signed(n as i64),
                Number::U32(n) => Unexpected::Unsigned(n as u64),
                Number::F32(n) => Unexpected::Float(n as f64),
                Number::I64(n) => Unexpected::Signed(n),
                Number::U64(n) => Unexpected::Unsigned(n),
                Number::F64(n) => Unexpected::Float(n),
                Number::I128(n) => match i64::try_from(n) {
                    Ok(n) => Unexpected::Signed(n),
                    Err(_) => Unexpected::Other("128-bit signed integer"),
                },
                Number::U128(n) => match u64::try_from(n) {
                    Ok(n) => Unexpected::Unsigned(n),
                    Err(_) => Unexpected::Other("128-bit unsigned integer"),
                },
            },
            Content::Char(v) => Unexpected::Char(*v),
            Content::String(v) => Unexpected::Str(v.as_ref()),
            Content::Bytes(v) => Unexpected::Bytes(v.as_ref()),
            Content::Seq(_) => Unexpected::Seq,
            Content::Map(_) => Unexpected::Map,
            Content::Option(_) => Unexpected::Option,
            Content::Struct(v) => v.data.unexpected_struct(),
            Content::Enum(v) => v.data.unexpected_enum(),
            Content::Tuple(_) => Unexpected::Other("tuple"),
        }
    }
}

impl Data<'_> {
    #[cold]
    pub(super) fn invalid_struct_type<E>(&self, exp: &dyn Expected) -> E
    where
        E: de::Error,
    {
        de::Error::invalid_type(self.unexpected_struct(), exp)
    }

    #[cold]
    fn unexpected_struct(&self) -> Unexpected {
        match self {
            Data::Unit => Unexpected::Other("unit struct"),
            Data::NewType { .. } => Unexpected::NewtypeStruct,
            Data::Tuple { .. } => Unexpected::Other("tuple struct"),
            Data::Struct { .. } => Unexpected::Other("struct"),
        }
    }

    #[cold]
    pub(super) fn invalid_enum_type<E>(&self, exp: &dyn Expected) -> E
    where
        E: de::Error,
    {
        de::Error::invalid_type(self.unexpected_enum(), exp)
    }

    #[cold]
    fn unexpected_enum(&self) -> Unexpected {
        match self {
            Data::Unit => Unexpected::UnitVariant,
            Data::NewType { .. } => Unexpected::NewtypeVariant,
            Data::Tuple { .. } => Unexpected::TupleVariant,
            Data::Struct { .. } => Unexpected::StructVariant,
        }
    }
}
