#![cfg(feature = "derive")]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde_content::from_content;
use serde_content::to_content;
use serde_content::Error;
use serde_content::Expected;
use serde_content::Found;
use serde_content::FoundData;
use serde_content::Number;

fn error_message(message: &str) -> String {
    format!("failed to deserialise; {message}")
}

#[test]
fn bool_errors() {
    let v = true;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Bool(v), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found true")
    );
}

#[test]
fn i8_errors() {
    let v = 1i8;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::I8(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1i8")
    );
}

#[test]
fn i16_errors() {
    let v = 1i16;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::I16(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1i16")
    );
}

#[test]
fn i32_errors() {
    let v = 1i32;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::I32(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1i32")
    );
}

#[test]
fn i64_errors() {
    let v = 1i64;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::I64(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1i64")
    );
}

#[test]
fn i128_errors() {
    let v = 1i128;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::I128(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1i128")
    );
}

#[test]
fn u8_errors() {
    let v = 1u8;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::U8(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1u8")
    );
}

#[test]
fn u16_errors() {
    let v = 1u16;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::U16(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1u16")
    );
}

#[test]
fn u32_errors() {
    let v = 1u32;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::U32(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1u32")
    );
}

#[test]
fn u64_errors() {
    let v = 1u64;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::U64(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1u64")
    );
}

#[test]
fn u128_errors() {
    let v = 1u128;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::U128(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1u128")
    );
}

#[test]
fn f32_errors() {
    let v = 1f32;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::F32(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1f32")
    );
}

#[test]
fn f64_errors() {
    let v = 1f64;
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Number(Number::F64(v)), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 1f64")
    );
}

#[test]
fn char_errors() {
    let v = 'a';
    let content = to_content(v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Char(v), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found 'a'")
    );
}

#[test]
fn string_errors() {
    let v = "foo".to_owned();
    let content = to_content(&v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::String(v), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message(r#"expected a unit, found "foo""#)
    );
}

#[test]
fn bytes_errors() {
    #[derive(Debug, Clone, PartialEq)]
    struct Bytes(Vec<u8>);
    impl Serialize for Bytes {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_bytes(&self.0)
        }
    }
    impl<'de> Deserialize<'de> for Bytes {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct BytesVisitor;
            impl<'de> Visitor<'de> for BytesVisitor {
                type Value = Bytes;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("bytes")
                }

                fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Bytes(v))
                }
            }

            deserializer.deserialize_byte_buf(BytesVisitor)
        }
    }
    let v = Bytes("foo".as_bytes().to_vec());
    let content = to_content(&v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Bytes(v.0), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found &[102, 111, 111]")
    );
}

#[test]
fn option_errors() {
    let content = to_content(None::<&str>).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(Found::Option(None), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found None")
    );
    //
    let v = "foo".to_owned();
    let content = to_content(Some(&v)).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(
        Found::Option(Some(Box::new(Found::String(v.into())))),
        Expected::Unit,
    );
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message(r#"expected a unit, found Some("foo")"#)
    );
}

#[test]
fn unit_errors() {
    let v = ();
    let content = to_content(v).unwrap();
    let error = from_content::<bool>(content).unwrap_err();
    let expected = Error::unexpected(Found::Unit, Expected::Bool);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a boolean, found ()")
    );
}

#[test]
fn unit_struct_errors() {
    #[derive(Serialize, Deserialize)]
    struct Foo;
    let content = to_content(Foo).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(
        Found::Struct {
            name: "Foo".to_owned(),
            data: Box::new(FoundData::Unit),
        },
        Expected::Unit,
    );
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found Foo")
    );
}

#[test]
fn unit_variant_errors() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Foo {
        Bar,
    }
    let v = Foo::Bar;
    let content = to_content(&v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(
        Found::Enum {
            name: "Foo".to_owned(),
            variant: "Bar".to_owned(),
            data: Box::new(FoundData::Unit),
        },
        Expected::Unit,
    );
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found Foo::Bar")
    );
}

#[test]
fn newtype_struct_errors() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo(bool);
    let v = true;
    let content = to_content(Foo(v)).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(
        Found::Struct {
            name: "Foo".to_owned(),
            data: Box::new(FoundData::NewType(Found::Bool(v))),
        },
        Expected::Unit,
    );
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found Foo(true)")
    );
}

#[test]
fn newtype_variant_errors() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Foo {
        Bar(bool),
    }
    let v = true;
    let content = to_content(Foo::Bar(v)).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(
        Found::Enum {
            name: "Foo".to_owned(),
            variant: "Bar".to_owned(),
            data: Box::new(FoundData::NewType(Found::Bool(v))),
        },
        Expected::Unit,
    );
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found Foo::Bar(true)")
    );
}

#[test]
fn seq_errors() {
    let v = vec![true, false];
    let content = to_content(&v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(
        Found::Seq(v.into_iter().map(Found::Bool).collect()),
        Expected::Unit,
    );
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found [true, false]")
    );
}

#[test]
fn tuple_errors() {
    let v = (true, 'a', "foo".to_owned());
    let content = to_content(&v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let tup = vec![
        Found::Bool(v.0),
        Found::Char(v.1),
        Found::String(v.2.into()),
    ];
    let expected = Error::unexpected(Found::Tuple(tup), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message(r#"expected a unit, found (true, 'a', "foo")"#)
    );
}

#[test]
fn tuple_struct_errors() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo(bool, char);
    let v = Foo(true, 'a');
    let content = to_content(&v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(
        Found::Struct {
            name: "Foo".to_owned(),
            data: Box::new(FoundData::Tuple(vec![Found::Bool(true), Found::Char('a')])),
        },
        Expected::Unit,
    );
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found Foo(true, 'a')")
    );
}

#[test]
fn tuple_variant_errors() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Foo {
        Bar(bool, char),
    }
    let v = Foo::Bar(true, 'a');
    let content = to_content(&v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(
        Found::Enum {
            name: "Foo".to_owned(),
            variant: "Bar".to_owned(),
            data: Box::new(FoundData::Tuple(vec![Found::Bool(true), Found::Char('a')])),
        },
        Expected::Unit,
    );
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found Foo::Bar(true, 'a')")
    );
}

#[test]
fn map_errors() {
    let mut v = BTreeMap::new();
    v.insert('f', false);
    v.insert('t', true);
    let content = to_content(&v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let map = v
        .into_iter()
        .map(|(k, v)| (Found::Char(k), Found::Bool(v)))
        .collect();
    let expected = Error::unexpected(Found::Map(map), Expected::Unit);
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found { 'f': false, 't': true }")
    );
}

#[test]
fn struct_errors() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo {
        bar: bool,
        baz: char,
    }
    let v = Foo {
        bar: true,
        baz: 'a',
    };
    let content = to_content(&v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(
        Found::Struct {
            name: "Foo".to_owned(),
            data: Box::new(FoundData::Struct(vec![
                ("bar".to_owned(), Found::Bool(true)),
                ("baz".to_owned(), Found::Char('a')),
            ])),
        },
        Expected::Unit,
    );
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found Foo { bar: true, baz: 'a' }")
    );
}

#[test]
fn struct_variant_errors() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Foo {
        Bar { bar: bool, baz: char },
    }
    let v = Foo::Bar {
        bar: true,
        baz: 'a',
    };
    let content = to_content(&v).unwrap();
    let error = from_content::<()>(content).unwrap_err();
    let expected = Error::unexpected(
        Found::Enum {
            name: "Foo".to_owned(),
            variant: "Bar".to_owned(),
            data: Box::new(FoundData::Struct(vec![
                ("bar".to_owned(), Found::Bool(true)),
                ("baz".to_owned(), Found::Char('a')),
            ])),
        },
        Expected::Unit,
    );
    assert_eq!(error, expected);
    assert_eq!(
        error.to_string(),
        error_message("expected a unit, found Foo::Bar { bar: true, baz: 'a' }")
    );
}
