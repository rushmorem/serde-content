#![cfg(feature = "derive")]
#![cfg(test)]

use crate::to_content;
use crate::Content;
use crate::Data;
use crate::Enum;
use crate::Number;
use crate::Struct;
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use serde::Serialize;

#[test]
fn serialize_bool() {
    assert_eq!(to_content(true).unwrap(), Content::Bool(true));
    assert_eq!(to_content(false).unwrap(), Content::Bool(false));
}

#[test]
fn serialize_i8() {
    assert_eq!(to_content(0i8).unwrap(), Content::Number(Number::I8(0)));
    assert_eq!(to_content(1i8).unwrap(), Content::Number(Number::I8(1)));
}

#[test]
fn serialize_i16() {
    assert_eq!(to_content(0i16).unwrap(), Content::Number(Number::I16(0)));
    assert_eq!(to_content(1i16).unwrap(), Content::Number(Number::I16(1)));
}

#[test]
fn serialize_i32() {
    assert_eq!(to_content(0i32).unwrap(), Content::Number(Number::I32(0)));
    assert_eq!(to_content(1i32).unwrap(), Content::Number(Number::I32(1)));
}

#[test]
fn serialize_i64() {
    assert_eq!(to_content(0i64).unwrap(), Content::Number(Number::I64(0)));
    assert_eq!(to_content(1i64).unwrap(), Content::Number(Number::I64(1)));
}

#[test]
fn serialize_i128() {
    assert_eq!(to_content(0i128).unwrap(), Content::Number(Number::I128(0)));
    assert_eq!(to_content(1i128).unwrap(), Content::Number(Number::I128(1)));
}

#[test]
fn serialize_u8() {
    assert_eq!(to_content(0u8).unwrap(), Content::Number(Number::U8(0)));
    assert_eq!(to_content(1u8).unwrap(), Content::Number(Number::U8(1)));
}

#[test]
fn serialize_u16() {
    assert_eq!(to_content(0u16).unwrap(), Content::Number(Number::U16(0)));
    assert_eq!(to_content(1u16).unwrap(), Content::Number(Number::U16(1)));
}

#[test]
fn serialize_u32() {
    assert_eq!(to_content(0u32).unwrap(), Content::Number(Number::U32(0)));
    assert_eq!(to_content(1u32).unwrap(), Content::Number(Number::U32(1)));
}

#[test]
fn serialize_u64() {
    assert_eq!(to_content(0u64).unwrap(), Content::Number(Number::U64(0)));
    assert_eq!(to_content(1u64).unwrap(), Content::Number(Number::U64(1)));
}

#[test]
fn serialize_u128() {
    assert_eq!(to_content(0u128).unwrap(), Content::Number(Number::U128(0)));
    assert_eq!(to_content(1u128).unwrap(), Content::Number(Number::U128(1)));
}

#[test]
fn serialize_f32() {
    assert_eq!(to_content(0f32).unwrap(), Content::Number(Number::F32(0.0)));
    assert_eq!(to_content(1f32).unwrap(), Content::Number(Number::F32(1.0)));
}

#[test]
fn serialize_f64() {
    assert_eq!(to_content(0f64).unwrap(), Content::Number(Number::F64(0.0)));
    assert_eq!(to_content(1f64).unwrap(), Content::Number(Number::F64(1.0)));
}

#[test]
fn serialize_char() {
    assert_eq!(to_content('a').unwrap(), Content::Char('a'));
}

#[test]
fn serialize_string() {
    assert_eq!(
        to_content("foo").unwrap(),
        Content::String(Cow::Borrowed("foo"))
    );
    assert_eq!(
        to_content("foo").unwrap(),
        Content::String(Cow::Owned("foo".to_owned()))
    );
    assert_eq!(
        to_content(String::new()).unwrap(),
        Content::String(Cow::Borrowed(""))
    );
    assert_eq!(
        to_content(String::new()).unwrap(),
        Content::String(Cow::Owned(String::new()))
    );
}

#[test]
fn serialize_bytes() {
    struct Bytes(&'static [u8]);
    impl Serialize for Bytes {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_bytes(self.0)
        }
    }
    assert_eq!(
        to_content(Bytes(b"")).unwrap(),
        Content::Bytes(Cow::Borrowed(b""))
    );
    assert_eq!(
        to_content(Bytes(b"foo")).unwrap(),
        Content::Bytes(Cow::Borrowed(b"foo"))
    );
}

#[test]
fn serialize_option() {
    assert_eq!(to_content(None::<&str>).unwrap(), Content::Option(None));
    assert_eq!(
        to_content(Some('a')).unwrap(),
        Content::Option(Some(Box::new(Content::Char('a'))))
    );
}

#[test]
fn serialize_unit() {
    assert_eq!(to_content(()).unwrap(), Content::Unit);
    assert_eq!(
        to_content(Some(())).unwrap(),
        Content::Option(Some(Box::new(Content::Unit)))
    );
}

#[test]
fn serialize_unit_struct() {
    #[derive(Serialize)]
    struct Foo;
    assert_eq!(
        to_content(Foo).unwrap(),
        Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::Unit
        }))
    );
}

#[test]
fn serialize_unit_variant() {
    #[derive(Serialize)]
    enum Foo {
        Bar,
    }
    assert_eq!(
        to_content(Foo::Bar).unwrap(),
        Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::Unit
        }))
    );
}

#[test]
fn serialize_newtype_struct() {
    #[derive(Serialize)]
    struct Foo(bool);
    assert_eq!(
        to_content(Foo(true)).unwrap(),
        Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::NewType {
                value: Content::Bool(true)
            }
        }))
    );
}

#[test]
fn serialize_newtype_variant() {
    #[derive(Serialize)]
    enum Foo {
        Bar(bool),
    }
    assert_eq!(
        to_content(Foo::Bar(true)).unwrap(),
        Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::NewType {
                value: Content::Bool(true)
            }
        }))
    );
}

#[test]
fn serialize_seq() {
    assert_eq!(
        to_content(Vec::<bool>::new()).unwrap(),
        Content::Seq(Vec::new())
    );
    assert_eq!(
        to_content(vec![true, false]).unwrap(),
        Content::Seq(vec![Content::Bool(true), Content::Bool(false)])
    );
}

#[test]
fn serialize_tuple() {
    assert_eq!(
        to_content((true,)).unwrap(),
        Content::Tuple(vec![Content::Bool(true)])
    );
    assert_eq!(
        to_content((true, 'a', "foo")).unwrap(),
        Content::Tuple(vec![
            Content::Bool(true),
            Content::Char('a'),
            Content::String(Cow::Borrowed("foo"))
        ])
    );
}

#[test]
fn serialize_tuple_struct() {
    #[derive(Serialize)]
    struct Foo(bool, char);
    assert_eq!(
        to_content(Foo(true, 'a')).unwrap(),
        Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::Tuple {
                values: vec![Content::Bool(true), Content::Char('a')],
            }
        }))
    );
}

#[test]
fn serialize_tuple_variant() {
    #[derive(Serialize)]
    enum Foo {
        Bar(bool, char),
    }
    assert_eq!(
        to_content(Foo::Bar(true, 'a')).unwrap(),
        Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::Tuple {
                values: vec![Content::Bool(true), Content::Char('a')],
            }
        }))
    );
}

#[test]
fn serialize_map() {
    assert_eq!(
        to_content(BTreeMap::<(), ()>::new()).unwrap(),
        Content::Map(Vec::new())
    );
    let mut map = BTreeMap::new();
    map.insert('f', false);
    map.insert('t', true);
    assert_eq!(
        to_content(map).unwrap(),
        Content::Map(vec![
            (Content::Char('f'), Content::Bool(false)),
            (Content::Char('t'), Content::Bool(true)),
        ])
    );
}

#[test]
fn serialize_struct() {
    #[derive(Serialize)]
    struct Foo {
        bar: bool,
        baz: char,
    }
    assert_eq!(
        to_content(Foo {
            bar: true,
            baz: 'a'
        })
        .unwrap(),
        Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::Struct {
                fields: vec![("bar", Content::Bool(true)), ("baz", Content::Char('a'))],
            }
        }))
    );
}

#[test]
fn serialize_struct_variant() {
    #[derive(Serialize)]
    #[allow(dead_code)]
    enum Foo {
        Bar { bar: bool, baz: char },
        Baz { bar: bool, baz: char },
    }
    assert_eq!(
        to_content(Foo::Baz {
            bar: true,
            baz: 'a',
        })
        .unwrap(),
        Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 1,
            variant: "Baz",
            data: Data::Struct {
                fields: vec![("bar", Content::Bool(true)), ("baz", Content::Char('a'))],
            }
        }))
    );
}
