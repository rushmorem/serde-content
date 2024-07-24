use crate::{DataType, Number};
use alloc::boxed::Box;
use alloc::string::String;
#[cfg(feature = "serde")]
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt;

/// Alias for [core::result::Result] with [crate::Error] as the error type.
pub type Result<T> = core::result::Result<T, Error>;

/// The error type returned by this crate.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "derive", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive] // In case we add new error variants in future.
pub struct Error {
    kind: Box<ErrorKind>,
}

impl Error {
    /// Creates a new unexpected error
    pub fn unexpected(found: Found, expected: Expected) -> Self {
        Self {
            kind: Box::new(ErrorKind::Unexpected { found, expected }),
        }
    }

    /// Borrows the underlying error kind
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Consumes the error and returns the error kind
    pub fn into_kind(self) -> ErrorKind {
        *self.kind
    }
}

/// The kind of error returned by [`Error::kind`]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "derive", derive(serde::Serialize, serde::Deserialize))]
pub enum ErrorKind {
    /// Found an unexpected type when deserialising.
    Unexpected {
        /// The type we found (and data where applicable).
        found: Found,
        /// The type we expected.
        expected: Expected,
    },
    /// A custom error message from `serde`.
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.kind {
            ErrorKind::Custom(msg) => write!(f, "{msg}"),
            ErrorKind::Unexpected { found, expected } => write!(
                f,
                "failed to deserialise; expected {expected}, found {found}"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "serde")]
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self {
            kind: Box::new(ErrorKind::Custom(msg.to_string())),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self {
            kind: Box::new(ErrorKind::Custom(msg.to_string())),
        }
    }
}

/// The type that was expected.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "derive", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive] // In case we add new number variants in future.
pub enum Expected {
    /// Expected the Rust unit type, `()`.
    Unit,
    /// Expected a Rust boolean.
    Bool,
    /// Expected an 8-bit signed integer type.
    I8,
    /// Expected an 8-bit unsigned integer type.
    U8,
    /// Expected a 16-bit signed integer type.
    I16,
    /// Expected a 16-bit unsigned integer type.
    U16,
    /// Expected a 32-bit signed integer type.
    I32,
    /// Expected a 32-bit unsigned integer type.
    U32,
    /// Expected a 32-bit floating point type.
    F32,
    /// Expected a 64-bit signed integer type.
    I64,
    /// Expected a 64-bit unsigned integer type.
    U64,
    /// Expected a 32-bit floating point type.
    F64,
    /// Expected a 128-bit signed integer type.
    I128,
    /// Expected a 128-bit unsigned integer type.
    U128,
    /// Expected a Rust character.
    Char,
    /// Expected a Rust string.
    String,
    /// Expected a Rust byte array.
    Bytes,
    /// Expected an array of Rust values.
    Seq,
    /// Expected a map of Rust values.
    Map,
    /// Expected optional Rust values.
    Option,
    /// Expected a Rust struct.
    Struct {
        /// The name of the struct.
        name: String,
        /// The type of the struct
        typ: DataType,
    },
    /// Expected a Rust enum.
    Enum {
        /// The name of the enum.
        name: String,
        /// The type of the enum.
        typ: DataType,
    },
    /// Expected a Rust tuple.
    Tuple(usize),
    /// Expected a struct field or an enum variant.
    Identifier,
}

#[doc(hidden)] // Not public API.
impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expected::Unit => write!(f, "a unit"),
            Expected::Bool => write!(f, "a boolean"),
            Expected::I8 => write!(f, "an 8-bit signed integer"),
            Expected::U8 => write!(f, "an 8-bit unsigned integer"),
            Expected::I16 => write!(f, "a 16-bit signed integer"),
            Expected::U16 => write!(f, "a 16-bit unsigned integer"),
            Expected::I32 => write!(f, "a 32-bit signed integer"),
            Expected::U32 => write!(f, "a 32-bit unsigned integer"),
            Expected::F32 => write!(f, "a 32-bit floating point"),
            Expected::I64 => write!(f, "an 8-bit signed integer"),
            Expected::U64 => write!(f, "a 64-bit signed integer"),
            Expected::F64 => write!(f, "a 64-bit floating point"),
            Expected::I128 => write!(f, "a 128-bit signed integer"),
            Expected::U128 => write!(f, "a 128-bit unsigned integer"),
            Expected::Char => write!(f, "a single character"),
            Expected::String => write!(f, "a string"),
            Expected::Bytes => write!(f, "a byte array"),
            Expected::Seq => write!(f, "a sequence"),
            Expected::Map => write!(f, "a map"),
            Expected::Option => write!(f, "an option"),
            Expected::Struct { name, typ } => match name.as_str() {
                crate::UNKNOWN_TYPE_NAME => write!(f, "{typ} struct"),
                name => write!(f, "{typ} struct named {name}"),
            },
            Expected::Enum { name, typ } => match name.as_str() {
                crate::UNKNOWN_TYPE_NAME => write!(f, "{typ} enum variant"),
                name => write!(f, "{typ} enum variant of {name}"),
            },
            Expected::Tuple(len) => write!(f, "a tuple with {len} elements"),
            Expected::Identifier => write!(f, "a struct field name or an enum variant"),
        }
    }
}

#[doc(hidden)] // Not public API.
impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Unit => write!(f, "a unit"),
            DataType::NewType => write!(f, "a newtype"),
            DataType::Tuple => write!(f, "a tuple"),
            DataType::Struct => write!(f, "an object-like"),
        }
    }
}

/// The type that was found.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "derive", derive(serde::Serialize, serde::Deserialize))]
pub enum Found {
    /// Found the Rust unit type, `()`.
    Unit,
    /// Found a Rust boolean.
    Bool(bool),
    /// Found any Rust number.
    Number(Number),
    /// Found a Rust character.
    Char(char),
    /// Found a Rust string.
    String(String),
    /// Found a Rust byte array.
    Bytes(Vec<u8>),
    /// Found an array of Rust values.
    Seq,
    /// Found a map of Rust values.
    Map,
    /// Found optional Rust values.
    Option,
    /// Found a Rust struct.
    Struct {
        /// The name of the struct.
        name: String,
        /// The type of the struct
        typ: DataType,
    },
    /// Found a Rust enum.
    Enum {
        /// The name of the enum.
        name: String,
        /// The variant of the enum.
        variant: String,
        /// The type of the enum.
        typ: DataType,
    },
    /// Found a Rust tuple.
    Tuple,
    /// Found a struct field or an enum variant.
    Identifier(String),
}

#[doc(hidden)] // Not public API.
impl fmt::Display for Found {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Found::Unit => write!(f, "a unit"),
            Found::Bool(v) => write!(f, "a boolean {v}"),
            Found::Number(v) => match v {
                Number::I8(v) => write!(f, "an 8-bit signed integer {v}"),
                Number::U8(v) => write!(f, "an 8-bit unsigned integer {v}"),
                Number::I16(v) => write!(f, "a 16-bit signed integer {v}"),
                Number::U16(v) => write!(f, "a 16-bit unsigned integer {v}"),
                Number::I32(v) => write!(f, "a 32-bit signed integer {v}"),
                Number::U32(v) => write!(f, "a 32-bit unsigned integer {v}"),
                Number::F32(v) => write!(f, "a 32-bit floating point {v}"),
                Number::I64(v) => write!(f, "a 64-bit signed integer {v}"),
                Number::U64(v) => write!(f, "a 64-bit unsigned integer {v}"),
                Number::F64(v) => write!(f, "a 64-bit floating point {v}"),
                Number::I128(v) => write!(f, "a 128-bit signed integer {v}"),
                Number::U128(v) => write!(f, "a 128-bit unsigned integer {v}"),
            },
            Found::Char(v) => write!(f, "a character {v}"),
            Found::String(v) => write!(f, "a string `{v}`"),
            Found::Bytes(_) => write!(f, "a byte array"),
            Found::Seq => write!(f, "a sequence"),
            Found::Map => write!(f, "a map"),
            Found::Option => write!(f, "an option"),
            Found::Struct { name, typ } => match name.as_str() {
                crate::UNKNOWN_TYPE_NAME => write!(f, "{typ} struct"),
                name => write!(f, "{typ} struct named {name}"),
            },
            Found::Enum { name, variant, typ } => match name.as_str() {
                crate::UNKNOWN_TYPE_NAME => write!(f, "{typ} enum variant named {variant}"),
                name => write!(f, "{typ} enum variant of {name} named {variant}"),
            },
            Found::Tuple => write!(f, "a tuple"),
            Found::Identifier(v) => write!(f, "found a struct field or enum variant {v}"),
        }
    }
}
