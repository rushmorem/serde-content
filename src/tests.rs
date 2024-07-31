#![cfg(test)]

use crate::Error;
use crate::Value;
use core::mem::size_of;

#[cfg(feature = "derive")]
#[derive(serde::Serialize)]
struct _Bar<'a>(Value<'a>);

#[test]
fn value_size() {
    assert!(size_of::<Value>() <= 32);
}

#[test]
fn error_size() {
    assert!(size_of::<Error>() <= 8);
}
