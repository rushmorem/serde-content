#![cfg(feature = "derive")]
#![cfg(test)]

use crate::Content;
use crate::Data;
use crate::Deserializer;
use crate::Enum;
use crate::Number;
use crate::Struct;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use serde::de::Visitor;
use serde::Deserialize;

#[test]
fn deserialize_bool() {
    assert!(Deserializer::new(Content::Bool(true))
        .deserialize::<bool>()
        .unwrap());
    assert!(!Deserializer::new(Content::Bool(false))
        .deserialize::<bool>()
        .unwrap());
}

#[test]
fn deserialize_i8() {
    assert_eq!(
        0i8,
        Deserializer::new(Content::Number(Number::I8(0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1i8,
        Deserializer::new(Content::Number(Number::I8(1)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_i16() {
    assert_eq!(
        0i16,
        Deserializer::new(Content::Number(Number::I16(0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1i16,
        Deserializer::new(Content::Number(Number::I16(1)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_i32() {
    assert_eq!(
        0i32,
        Deserializer::new(Content::Number(Number::I32(0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1i32,
        Deserializer::new(Content::Number(Number::I32(1)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_i64() {
    assert_eq!(
        0i64,
        Deserializer::new(Content::Number(Number::I64(0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1i64,
        Deserializer::new(Content::Number(Number::I64(1)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_i128() {
    assert_eq!(
        0i128,
        Deserializer::new(Content::Number(Number::I128(0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1i128,
        Deserializer::new(Content::Number(Number::I128(1)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_u8() {
    assert_eq!(
        0u8,
        Deserializer::new(Content::Number(Number::U8(0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1u8,
        Deserializer::new(Content::Number(Number::U8(1)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_u16() {
    assert_eq!(
        0u16,
        Deserializer::new(Content::Number(Number::U16(0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1u16,
        Deserializer::new(Content::Number(Number::U16(1)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_u32() {
    assert_eq!(
        0u32,
        Deserializer::new(Content::Number(Number::U32(0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1u32,
        Deserializer::new(Content::Number(Number::U32(1)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_u64() {
    assert_eq!(
        0u64,
        Deserializer::new(Content::Number(Number::U64(0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1u64,
        Deserializer::new(Content::Number(Number::U64(1)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_u128() {
    assert_eq!(
        0u128,
        Deserializer::new(Content::Number(Number::U128(0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1u128,
        Deserializer::new(Content::Number(Number::U128(1)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_f32() {
    assert_eq!(
        0f32,
        Deserializer::new(Content::Number(Number::F32(0.0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1f32,
        Deserializer::new(Content::Number(Number::F32(1.0)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_f64() {
    assert_eq!(
        0f64,
        Deserializer::new(Content::Number(Number::F64(0.0)))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        1f64,
        Deserializer::new(Content::Number(Number::F64(1.0)))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_char() {
    assert_eq!(
        'a',
        Deserializer::new(Content::Char('a')).deserialize().unwrap()
    );
}

#[test]
fn deserialize_string() {
    let foo = String::from("foo");
    assert_eq!(
        foo,
        Deserializer::new(Content::String(Cow::Borrowed(&foo)))
            .deserialize::<&str>()
            .unwrap()
    );
    assert_eq!(
        foo,
        Deserializer::new(Content::String(Cow::Owned(foo.clone())))
            .deserialize::<String>()
            .unwrap()
    );
    assert_eq!(
        String::new(),
        Deserializer::new(Content::String(Cow::Borrowed("")))
            .deserialize::<&str>()
            .unwrap()
    );
    assert_eq!(
        String::new(),
        Deserializer::new(Content::String(Cow::Owned(String::new())))
            .deserialize::<String>()
            .unwrap()
    );
}

#[test]
fn deserialize_bytes() {
    #[derive(Debug, PartialEq)]
    struct Bytes(&'static [u8]);
    impl Deserialize<'static> for Bytes {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'static>,
        {
            struct BytesVisitor;
            impl Visitor<'static> for BytesVisitor {
                type Value = Bytes;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("bytes")
                }

                fn visit_borrowed_bytes<E>(self, v: &'static [u8]) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Bytes(v))
                }
            }

            deserializer.deserialize_bytes(BytesVisitor)
        }
    }
    assert_eq!(
        Bytes(b""),
        Deserializer::new(Content::Bytes(Cow::Borrowed(b"")))
            .deserialize()
            .unwrap(),
    );
    assert_eq!(
        Bytes(b"foo"),
        Deserializer::new(Content::Bytes(Cow::Borrowed(b"foo")))
            .deserialize()
            .unwrap(),
    );
}

#[test]
fn deserialize_option() {
    assert_eq!(
        None::<&str>,
        Deserializer::new(Content::Option(None))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        Some('a'),
        Deserializer::new(Content::Option(Some(Box::new(Content::Char('a')))))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        Some(()),
        Deserializer::new(Content::Unit).deserialize().unwrap()
    );
    assert_eq!(
        Some(true),
        Deserializer::new(Content::Bool(true))
            .deserialize()
            .unwrap()
    );
}

#[test]
fn deserialize_unit() {
    Deserializer::new(Content::Unit)
        .deserialize::<()>()
        .unwrap();
    assert_eq!(
        Some(()),
        Deserializer::new(Content::Option(Some(Box::new(Content::Unit))))
            .deserialize()
            .unwrap(),
    );
}

#[test]
fn deserialize_unit_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo;
    assert_eq!(
        Foo,
        Deserializer::new(Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::Unit
        })))
        .deserialize()
        .unwrap()
    );
}

#[test]
fn deserialize_unit_variant() {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Foo {
        Bar,
    }
    assert_eq!(
        Foo::Bar,
        Deserializer::new(Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::Unit
        })))
        .deserialize()
        .unwrap()
    );
}

#[test]
fn deserialize_newtype_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo(bool);
    assert_eq!(
        Foo(true),
        Deserializer::new(Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::NewType {
                value: Content::Bool(true)
            }
        })))
        .deserialize()
        .unwrap()
    );
}

#[test]
fn deserialize_newtype_variant() {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Foo {
        Bar(bool),
    }
    assert_eq!(
        Foo::Bar(true),
        Deserializer::new(Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::NewType {
                value: Content::Bool(true)
            }
        })))
        .deserialize()
        .unwrap()
    );
}

#[test]
fn deserialize_seq() {
    assert_eq!(
        Vec::<bool>::new(),
        Deserializer::new(Content::Seq(Vec::new()))
            .deserialize::<Vec<_>>()
            .unwrap()
    );
    assert_eq!(
        vec![true, false],
        Deserializer::new(Content::Seq(vec![
            Content::Bool(true),
            Content::Bool(false)
        ]))
        .deserialize::<Vec<_>>()
        .unwrap()
    );
}

#[test]
fn deserialize_tuple() {
    assert_eq!(
        (true,),
        Deserializer::new(Content::Tuple(vec![Content::Bool(true)]))
            .deserialize()
            .unwrap()
    );
    assert_eq!(
        (true, 'a', "foo"),
        Deserializer::new(Content::Tuple(vec![
            Content::Bool(true),
            Content::Char('a'),
            Content::String(Cow::Borrowed("foo"))
        ]))
        .deserialize()
        .unwrap()
    );
}

#[test]
fn deserialize_tuple_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo(bool, char);
    assert_eq!(
        Foo(true, 'a'),
        Deserializer::new(Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::Tuple {
                values: vec![Content::Bool(true), Content::Char('a')],
            }
        })))
        .deserialize()
        .unwrap()
    );
}

#[test]
fn deserialize_tuple_variant() {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Foo {
        Bar(bool, char),
    }
    assert_eq!(
        Foo::Bar(true, 'a'),
        Deserializer::new(Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::Tuple {
                values: vec![Content::Bool(true), Content::Char('a')],
            }
        })))
        .deserialize()
        .unwrap()
    );
}

#[test]
fn deserialize_map() {
    assert_eq!(
        BTreeMap::<(), ()>::new(),
        Deserializer::new(Content::Map(Vec::new()))
            .deserialize()
            .unwrap()
    );
    let mut map = BTreeMap::new();
    map.insert('f', false);
    map.insert('t', true);
    assert_eq!(
        map,
        Deserializer::new(Content::Map(vec![
            (Content::Char('f'), Content::Bool(false)),
            (Content::Char('t'), Content::Bool(true)),
        ]))
        .deserialize()
        .unwrap()
    );
}

#[test]
fn deserialize_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo {
        bar: bool,
        baz: char,
    }
    assert_eq!(
        Foo {
            bar: true,
            baz: 'a'
        },
        Deserializer::new(Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::Struct {
                fields: vec![("bar", Content::Bool(true)), ("baz", Content::Char('a'))],
            }
        })))
        .deserialize()
        .unwrap()
    );
}

#[test]
fn deserialize_struct_variant() {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Foo {
        Bar { bar: bool, baz: char },
    }
    assert_eq!(
        Foo::Bar {
            bar: true,
            baz: 'a',
        },
        Deserializer::new(Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::Struct {
                fields: vec![("bar", Content::Bool(true)), ("baz", Content::Char('a'))],
            }
        })))
        .deserialize()
        .unwrap()
    );
}
