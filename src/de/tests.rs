#![cfg(feature = "derive")]
#![cfg(test)]

use crate::from_content;
use crate::Content;
use crate::Data;
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
use serde::Deserializer;

#[test]
fn deserialize_bool() {
    assert_eq!(true, from_content(Content::Bool(true)).unwrap());
    assert_eq!(false, from_content(Content::Bool(false)).unwrap());
}

#[test]
fn deserialize_i8() {
    assert_eq!(0i8, from_content(Content::Number(Number::I8(0))).unwrap());
    assert_eq!(1i8, from_content(Content::Number(Number::I8(1))).unwrap());
}

#[test]
fn deserialize_i16() {
    assert_eq!(0i16, from_content(Content::Number(Number::I16(0))).unwrap());
    assert_eq!(1i16, from_content(Content::Number(Number::I16(1))).unwrap());
}

#[test]
fn deserialize_i32() {
    assert_eq!(0i32, from_content(Content::Number(Number::I32(0))).unwrap());
    assert_eq!(1i32, from_content(Content::Number(Number::I32(1))).unwrap());
}

#[test]
fn deserialize_i64() {
    assert_eq!(0i64, from_content(Content::Number(Number::I64(0))).unwrap());
    assert_eq!(1i64, from_content(Content::Number(Number::I64(1))).unwrap());
}

#[test]
fn deserialize_i128() {
    assert_eq!(
        0i128,
        from_content(Content::Number(Number::I128(0))).unwrap()
    );
    assert_eq!(
        1i128,
        from_content(Content::Number(Number::I128(1))).unwrap()
    );
}

#[test]
fn deserialize_u8() {
    assert_eq!(0u8, from_content(Content::Number(Number::U8(0))).unwrap());
    assert_eq!(1u8, from_content(Content::Number(Number::U8(1))).unwrap());
}

#[test]
fn deserialize_u16() {
    assert_eq!(0u16, from_content(Content::Number(Number::U16(0))).unwrap());
    assert_eq!(1u16, from_content(Content::Number(Number::U16(1))).unwrap());
}

#[test]
fn deserialize_u32() {
    assert_eq!(0u32, from_content(Content::Number(Number::U32(0))).unwrap());
    assert_eq!(1u32, from_content(Content::Number(Number::U32(1))).unwrap());
}

#[test]
fn deserialize_u64() {
    assert_eq!(0u64, from_content(Content::Number(Number::U64(0))).unwrap());
    assert_eq!(1u64, from_content(Content::Number(Number::U64(1))).unwrap());
}

#[test]
fn deserialize_u128() {
    assert_eq!(
        0u128,
        from_content(Content::Number(Number::U128(0))).unwrap()
    );
    assert_eq!(
        1u128,
        from_content(Content::Number(Number::U128(1))).unwrap()
    );
}

#[test]
fn deserialize_f32() {
    assert_eq!(
        0f32,
        from_content(Content::Number(Number::F32(0.0))).unwrap()
    );
    assert_eq!(
        1f32,
        from_content(Content::Number(Number::F32(1.0))).unwrap()
    );
}

#[test]
fn deserialize_f64() {
    assert_eq!(
        0f64,
        from_content(Content::Number(Number::F64(0.0))).unwrap()
    );
    assert_eq!(
        1f64,
        from_content(Content::Number(Number::F64(1.0))).unwrap()
    );
}

#[test]
fn deserialize_char() {
    assert_eq!('a', from_content(Content::Char('a')).unwrap());
}

#[test]
fn deserialize_string() {
    let foo = String::from("foo");
    assert_eq!(
        foo,
        from_content::<&str>(Content::String(Cow::Borrowed(&foo))).unwrap()
    );
    assert_eq!(
        foo,
        from_content::<String>(Content::String(Cow::Owned(foo.clone()))).unwrap()
    );
    assert_eq!(
        String::new(),
        from_content::<&str>(Content::String(Cow::Borrowed(""))).unwrap()
    );
    assert_eq!(
        String::new(),
        from_content::<String>(Content::String(Cow::Owned(String::new()))).unwrap()
    );
}

#[test]
fn deserialize_bytes() {
    #[derive(Debug, PartialEq)]
    struct Bytes(&'static [u8]);
    impl Deserialize<'static> for Bytes {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'static>,
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
        from_content(Content::Bytes(Cow::Borrowed(b""))).unwrap(),
    );
    assert_eq!(
        Bytes(b"foo"),
        from_content(Content::Bytes(Cow::Borrowed(b"foo"))).unwrap(),
    );
}

#[test]
fn deserialize_option() {
    assert_eq!(None::<&str>, from_content(Content::Option(None)).unwrap());
    assert_eq!(
        Some('a'),
        from_content(Content::Option(Some(Box::new(Content::Char('a'))))).unwrap()
    );
    assert_eq!(Some(()), from_content(Content::Unit).unwrap());
    assert_eq!(Some(true), from_content(Content::Bool(true)).unwrap());
}

#[test]
fn deserialize_unit() {
    assert_eq!((), from_content(Content::Unit).unwrap());
    assert_eq!(
        Some(()),
        from_content(Content::Option(Some(Box::new(Content::Unit)))).unwrap(),
    );
}

#[test]
fn deserialize_unit_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo;
    assert_eq!(
        Foo,
        from_content(Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::Unit
        })))
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
        from_content(Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::Unit
        })))
        .unwrap()
    );
}

#[test]
fn deserialize_newtype_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo(bool);
    assert_eq!(
        Foo(true),
        from_content(Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::NewType {
                value: Content::Bool(true)
            }
        })))
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
        from_content(Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::NewType {
                value: Content::Bool(true)
            }
        })))
        .unwrap()
    );
}

#[test]
fn deserialize_seq() {
    assert_eq!(
        Vec::<bool>::new(),
        from_content::<Vec<_>>(Content::Seq(Vec::new())).unwrap()
    );
    assert_eq!(
        vec![true, false],
        from_content::<Vec<_>>(Content::Seq(vec![
            Content::Bool(true),
            Content::Bool(false)
        ]))
        .unwrap()
    );
}

#[test]
fn deserialize_tuple() {
    assert_eq!(
        (true,),
        from_content(Content::Tuple(vec![Content::Bool(true)])).unwrap()
    );
    assert_eq!(
        (true, 'a', "foo"),
        from_content(Content::Tuple(vec![
            Content::Bool(true),
            Content::Char('a'),
            Content::String(Cow::Borrowed("foo"))
        ]))
        .unwrap()
    );
}

#[test]
fn deserialize_tuple_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo(bool, char);
    assert_eq!(
        Foo(true, 'a'),
        from_content(Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::Tuple {
                values: vec![Content::Bool(true), Content::Char('a')],
            }
        })))
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
        from_content(Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::Tuple {
                values: vec![Content::Bool(true), Content::Char('a')],
            }
        })))
        .unwrap()
    );
}

#[test]
fn deserialize_map() {
    assert_eq!(
        BTreeMap::<(), ()>::new(),
        from_content(Content::Map(Vec::new())).unwrap()
    );
    let mut map = BTreeMap::new();
    map.insert('f', false);
    map.insert('t', true);
    assert_eq!(
        map,
        from_content(Content::Map(vec![
            (Content::Char('f'), Content::Bool(false)),
            (Content::Char('t'), Content::Bool(true)),
        ]))
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
        from_content(Content::Struct(Box::new(Struct {
            name: "Foo",
            data: Data::Struct {
                fields: vec![("bar", Content::Bool(true)), ("baz", Content::Char('a'))],
            }
        })))
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
        from_content(Content::Enum(Box::new(Enum {
            name: "Foo",
            variant_index: 0,
            variant: "Bar",
            data: Data::Struct {
                fields: vec![("bar", Content::Bool(true)), ("baz", Content::Char('a'))],
            }
        })))
        .unwrap()
    );
}
