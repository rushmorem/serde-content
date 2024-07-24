use alloc::string::String;
#[cfg(feature = "serde")]
use alloc::string::ToString;
use core::fmt;

/// Alias for [core::result::Result] with [crate::Error] as the error type.
pub type Result<T> = core::result::Result<T, Error>;

/// The error type returned by this crate.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "derive", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive] // In case we add new error variants in future.
pub enum Error {
    /// A custom error message from `serde`.
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{msg}"),
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
        Self::Custom(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}
