//! A container for the Serde data model.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs, unused_imports)]

extern crate alloc;

mod de;
mod error;
mod ser;
mod tests;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub use error::Data as FoundData;
pub use error::Error;
pub use error::ErrorKind;
pub use error::Expected;
pub use error::Found;
pub use error::Result;
#[cfg(feature = "serde")]
pub use {de::from_content, de::Deserializer, de::Unexpected, ser::to_content, ser::Serializer};

/// A containter for all Rust number types.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[non_exhaustive] // In case Rust introduces new number types.
pub enum Number {
    /// Holds an 8-bit signed integer type.
    I8(i8),
    /// Holds an 8-bit unsigned integer type.
    U8(u8),

    /// Holds a 16-bit signed integer type.
    I16(i16),
    /// Holds a 16-bit unsigned integer type.
    U16(u16),

    /// Holds a 32-bit signed integer type.
    I32(i32),
    /// Holds a 32-bit unsigned integer type.
    U32(u32),
    /// Holds a 32-bit floating point type.
    F32(f32),

    /// Holds a 64-bit signed integer type.
    I64(i64),
    /// Holds a 64-bit unsigned integer type.
    U64(u64),
    /// Holds a 32-bit floating point type.
    F64(f64),

    /// Holds a 128-bit signed integer type.
    I128(i128),
    /// Holds a 128-bit unsigned integer type.
    U128(u128),
}

/// Represents struct and enum data.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Data<'a> {
    /// Represents unit structs and unit enum variants.
    Unit,
    /// Represents newtype structs and enum variants.
    NewType {
        /// The value of the newtype struct or enum variant.
        value: Content<'a>,
    },
    /// Represents tuple structs and enum variants.
    Tuple {
        /// The values of the tuple struct or enum variant.
        values: Vec<Content<'a>>,
    },
    /// Represents object-like structs and enum variants.
    Struct {
        /// A vector of field names and their values.
        fields: Vec<(&'static str, Content<'a>)>,
    },
}

impl Data<'_> {
    /// Moves data where possible or otherwise clones it into an owned object.
    pub fn into_owned(self) -> Data<'static> {
        match self {
            Data::Unit => Data::Unit,
            Data::NewType { value } => Data::NewType {
                value: value.into_owned(),
            },
            Data::Tuple { values } => Data::Tuple {
                values: values.into_iter().map(Content::into_owned).collect(),
            },
            Data::Struct { fields } => Data::Struct {
                fields: fields
                    .into_iter()
                    .map(|(key, value)| (key, value.into_owned()))
                    .collect(),
            },
        }
    }

    #[cfg(feature = "serde")]
    const fn typ(&self) -> DataType {
        match self {
            Data::Unit => DataType::Unit,
            Data::NewType { .. } => DataType::NewType,
            Data::Tuple { .. } => DataType::Tuple,
            Data::Struct { .. } => DataType::Struct,
        }
    }
}

/// Struct and enum data type.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "derive", derive(serde::Serialize, serde::Deserialize))]
pub enum DataType {
    /// Unit struct or unit enum variant.
    Unit,
    /// Newtype struct or enum variant.
    NewType,
    /// Tuple struct or enum variant.
    Tuple,
    /// Object-like struct or enum variant.
    Struct,
}

/// Represents a Rust struct.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Struct<'a> {
    /// The name of the struct.
    pub name: &'static str,
    /// The data of the struct.
    pub data: Data<'a>,
}

impl Struct<'_> {
    /// Moves data where possible or otherwise clones it into an owned object.
    pub fn into_owned(self) -> Struct<'static> {
        Struct {
            data: self.data.into_owned(),
            ..self
        }
    }
}

/// Represents a Rust enum.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Enum<'a> {
    /// The name of the enum.
    pub name: &'static str,
    /// The index of the enum variant.
    pub variant_index: u32,
    /// The name of the enum variant.
    pub variant: &'static str,
    /// The data of the enum.
    pub data: Data<'a>,
}

impl Enum<'_> {
    /// Moves data where possible or otherwise clones it into an owned object.
    pub fn into_owned(self) -> Enum<'static> {
        Enum {
            data: self.data.into_owned(),
            ..self
        }
    }
}

/// Represents any valid Rust value.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Content<'a> {
    /// Represents the Rust unit type, `()`.
    Unit,
    /// Represents a Rust boolean.
    Bool(bool),
    /// Represents any Rust number.
    Number(Number),
    /// Represents a Rust character.
    Char(char),
    /// Represents a Rust string.
    String(Cow<'a, str>),
    /// Represents a Rust byte array.
    Bytes(Cow<'a, [u8]>),
    /// Represents an array of Rust values.
    Seq(Vec<Content<'a>>),
    /// Represents a map of Rust values.
    Map(Vec<(Content<'a>, Content<'a>)>),
    /// Represents optional Rust values.
    Option(Option<Box<Content<'a>>>),
    /// Represents a Rust struct.
    Struct(Box<Struct<'a>>),
    /// Represents a Rust enum.
    Enum(Box<Enum<'a>>),
    /// Represents a Rust tuple.
    Tuple(Vec<Content<'a>>),
}

impl Content<'_> {
    /// Moves data where possible or otherwise clones it into an owned object.
    pub fn into_owned(self) -> Content<'static> {
        match self {
            Content::Unit => Content::Unit,
            Content::Bool(v) => Content::Bool(v),
            Content::Number(v) => Content::Number(v),
            Content::Char(v) => Content::Char(v),
            Content::String(v) => Content::String(Cow::Owned(v.into_owned())),
            Content::Bytes(v) => Content::Bytes(Cow::Owned(v.into_owned())),
            Content::Seq(v) => Content::Seq(v.into_iter().map(Self::into_owned).collect()),
            Content::Map(v) => Content::Map(
                v.into_iter()
                    .map(|(key, value)| (key.into_owned(), value.into_owned()))
                    .collect(),
            ),
            Content::Option(v) => match v {
                Some(v) => Content::Option(Some(Box::new(v.into_owned()))),
                None => Content::Option(None),
            },
            Content::Struct(v) => Content::Struct(Box::new(v.into_owned())),
            Content::Enum(v) => Content::Enum(Box::new(v.into_owned())),
            Content::Tuple(v) => Content::Tuple(v.into_iter().map(Self::into_owned).collect()),
        }
    }
}
