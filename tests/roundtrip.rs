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

#[test]
fn roundtrip_bool() {
    let v = true;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = false;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_i8() {
    let v = 0i8;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1i8;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_i16() {
    let v = 0i16;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1i16;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_i32() {
    let v = 0i32;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1i32;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_i64() {
    let v = 0i64;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1i64;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_i128() {
    let v = 0i128;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1i128;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_u8() {
    let v = 0u8;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1u8;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_u16() {
    let v = 0u16;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1u16;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_u32() {
    let v = 0u32;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1u32;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_u64() {
    let v = 0u64;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1u64;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_u128() {
    let v = 0u128;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1u128;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_f32() {
    let v = 0f32;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1f32;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_f64() {
    let v = 0f64;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = 1f64;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_char() {
    let v = 'a';
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_string() {
    let v = "";
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content::<String>(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = "foo";
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content::<String>(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_bytes() {
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
    let v = Bytes("".as_bytes().to_vec());
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = Bytes("foo".as_bytes().to_vec());
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_option() {
    let v = None::<&str>;
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = Some("foo".to_owned());
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_unit() {
    let v = ();
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = Some(());
    let content = to_content(v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_unit_struct() {
    #[derive(Serialize, Deserialize)]
    struct Foo;
    let content = to_content(Foo).unwrap();
    from_content::<Foo>(content.clone()).unwrap();
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_unit_variant() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Foo {
        Bar,
    }
    let v = Foo::Bar;
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_newtype_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo(bool);
    let v = Foo(true);
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(dbg!(content.clone())).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_newtype_variant() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Foo {
        Bar(bool),
    }
    let v = Foo::Bar(true);
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_seq() {
    let v = Vec::<bool>::new();
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content::<Vec<_>>(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = vec![true, false];
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content::<Vec<_>>(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_tuple() {
    let v = (true,);
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let v = (true, 'a', "foo".to_owned());
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_tuple_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo(bool, char);
    let v = Foo(true, 'a');
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_tuple_variant() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Foo {
        Bar(bool, char),
    }
    let v = Foo::Bar(true, 'a');
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_map() {
    let v = BTreeMap::<(), ()>::new();
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
    //
    let mut v = BTreeMap::new();
    v.insert('f', false);
    v.insert('t', true);
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_struct() {
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
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}

#[test]
fn roundtrip_struct_variant() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Foo {
        Bar { bar: bool, baz: char },
    }
    let v = Foo::Bar {
        bar: true,
        baz: 'a',
    };
    let content = to_content(&v).unwrap();
    assert_eq!(v, from_content(content.clone()).unwrap());
    assert_eq!(content.clone(), from_content(content).unwrap());
}
