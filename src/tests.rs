#![cfg(test)]

use crate::Error;
use crate::Value;
use core::mem::size_of;

// Ensure the value works well with derive macros.
#[cfg(feature = "derive")]
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(dead_code)] // This is only used for size_of
struct Bar(Value<'static>);

// Ensure the value works well with derive macros.
#[cfg(feature = "derive")]
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(dead_code)] // This is only used for size_of
struct Foo(Bar);

#[test]
fn value_size() {
    assert!(size_of::<Value>() <= 32);
}

#[test]
fn error_size() {
    assert!(size_of::<Error>() <= 8);
}
