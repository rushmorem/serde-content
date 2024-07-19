#![cfg(test)]

use crate::Content;
use crate::Error;
use core::mem::size_of;

#[test]
fn content_size() {
    assert!(size_of::<Content>() <= 32);
}

#[test]
fn error_size() {
    assert!(size_of::<Error>() <= 32);
}
