use crate::{Content, Enum, Error, Expected, Found, Number, Struct};

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

impl Unexpected for Content<'_> {
    fn unexpected(self, expected: Expected) -> Error {
        let found = match self {
            Content::Unit => Found::Unit,
            Content::Bool(v) => Found::Bool(v),
            Content::Number(v) => Found::Number(v),
            Content::Char(v) => Found::Char(v),
            Content::String(v) => Found::String(v.into_owned()),
            Content::Bytes(v) => Found::Bytes(v.into_owned()),
            Content::Seq(_) => Found::Seq,
            Content::Map(_) => Found::Map,
            Content::Option(_) => Found::Option,
            Content::Struct(v) => Found::Struct {
                name: v.name.to_owned(),
                typ: v.data.typ(),
            },
            Content::Enum(v) => Found::Enum {
                name: v.name.to_owned(),
                variant: v.variant.to_owned(),
                typ: v.data.typ(),
            },
            Content::Tuple(_) => Found::Tuple,
        };
        Error::unexpected(found, expected)
    }
}

impl Unexpected for Box<Struct<'_>> {
    fn unexpected(self, expected: Expected) -> Error {
        let found = Found::Struct {
            name: self.name.to_owned(),
            typ: self.data.typ(),
        };
        Error::unexpected(found, expected)
    }
}

impl Unexpected for Box<Enum<'_>> {
    fn unexpected(self, expected: Expected) -> Error {
        let found = Found::Enum {
            name: self.name.to_owned(),
            variant: self.variant.to_owned(),
            typ: self.data.typ(),
        };
        Error::unexpected(found, expected)
    }
}
