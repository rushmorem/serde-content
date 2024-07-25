use crate::{Content, Data, Enum, Error, Expected, Found, FoundData, Number, Struct};
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// A convenience wrapper for constructing [crate::Found] and returning an error.
pub trait Unexpected {
    /// Consumes the type and returns an error.
    fn unexpected(self, expected: Expected) -> Error;
}

impl Unexpected for Number {
    fn unexpected(self, expected: Expected) -> Error {
        let found = Found::Number(self);
        Error::unexpected(found, expected)
    }
}

impl Content<'_> {
    fn into_found(self) -> Found {
        match self {
            Content::Unit => Found::Unit,
            Content::Bool(v) => Found::Bool(v),
            Content::Number(v) => Found::Number(v),
            Content::Char(v) => Found::Char(v),
            Content::String(v) => Found::String(v.into_owned()),
            Content::Bytes(v) => Found::Bytes(v.into_owned()),
            Content::Seq(v) => {
                let mut vec = Vec::with_capacity(v.len());
                for content in v {
                    vec.push(content.into_found());
                }
                Found::Seq(vec)
            }
            Content::Map(v) => {
                let mut vec = Vec::with_capacity(v.len());
                for (key, value) in v {
                    vec.push((key.into_found(), value.into_found()));
                }
                Found::Map(vec)
            }
            Content::Option(v) => Found::Option(v.map(|x| Box::new(x.into_found()))),
            Content::Struct(v) => Found::Struct {
                name: v.name.to_owned(),
                data: Box::new(v.data.into_found()),
            },
            Content::Enum(v) => Found::Enum {
                name: v.name.to_owned(),
                variant: v.variant.to_owned(),
                data: Box::new(v.data.into_found()),
            },
            Content::Tuple(v) => {
                let mut vec = Vec::with_capacity(v.len());
                for content in v {
                    vec.push(content.into_found());
                }
                Found::Tuple(vec)
            }
        }
    }
}

impl Data<'_> {
    fn into_found(self) -> FoundData {
        match self {
            Data::Unit => FoundData::Unit,
            Data::NewType { value } => FoundData::NewType(value.into_found()),
            Data::Tuple { values } => {
                FoundData::Tuple(values.into_iter().map(Content::into_found).collect())
            }
            Data::Struct { fields } => FoundData::Struct(
                fields
                    .into_iter()
                    .map(|(k, v)| (k.to_owned(), v.into_found()))
                    .collect(),
            ),
        }
    }
}

impl Unexpected for Content<'_> {
    fn unexpected(self, expected: Expected) -> Error {
        let found = self.into_found();
        Error::unexpected(found, expected)
    }
}

impl Unexpected for Box<Struct<'_>> {
    fn unexpected(self, expected: Expected) -> Error {
        let found = Found::Struct {
            name: self.name.to_owned(),
            data: Box::new(self.data.into_found()),
        };
        Error::unexpected(found, expected)
    }
}

impl Unexpected for Box<Enum<'_>> {
    fn unexpected(self, expected: Expected) -> Error {
        let found = Found::Enum {
            name: self.name.to_owned(),
            variant: self.variant.to_owned(),
            data: Box::new(self.data.into_found()),
        };
        Error::unexpected(found, expected)
    }
}
