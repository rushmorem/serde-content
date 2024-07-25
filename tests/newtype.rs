#![cfg(feature = "derive")]

extern crate alloc;

use alloc::string::String;
use serde::{Deserialize, Serialize};
use serde_content::{from_content, to_content};

#[test]
fn string() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo(String);
    let foo = Foo(String::new());
    let content = to_content(&foo.0).unwrap();
    assert_eq!(foo, from_content(content).unwrap());
    //
    let foo = Foo("bar".to_owned());
    let content = to_content(&foo.0).unwrap();
    assert_eq!(foo, from_content(content).unwrap());
}

#[test]
fn newtype_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Bar(String);
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo(Bar);
    let foo = Foo(Bar(String::new()));
    let content = to_content(&foo.0).unwrap();
    assert_eq!(foo, from_content(content).unwrap());
    //
    let foo = Foo(Bar("bar".to_owned()));
    let content = to_content(&foo.0).unwrap();
    assert_eq!(foo, from_content(content).unwrap());
}

#[test]
fn tuple_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Bar(String, usize);
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo(Bar);
    let foo = Foo(Bar(String::new(), 0));
    let content = to_content(&foo.0).unwrap();
    assert_eq!(foo, from_content(content).unwrap());
    //
    let foo = Foo(Bar("bar".to_owned(), 56));
    let content = to_content(&foo.0).unwrap();
    assert_eq!(foo, from_content(content).unwrap());
}

#[test]
fn object_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Bar {
        foo: String,
        baz: usize,
    }
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo(Bar);
    let foo = Foo(Bar {
        foo: String::new(),
        baz: 0,
    });
    let content = to_content(&foo.0).unwrap();
    assert_eq!(foo, from_content(content).unwrap());
    //
    let foo = Foo(Bar {
        foo: "bar".to_owned(),
        baz: 56,
    });
    let content = to_content(&foo.0).unwrap();
    assert_eq!(foo, from_content(content).unwrap());
}

#[test]
fn newtype_enum() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Bar {
        Baz(String),
    }
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo(Bar);
    let foo = Foo(Bar::Baz(String::new()));
    let content = to_content(&foo.0).unwrap();
    assert_eq!(foo, from_content(content).unwrap());
    //
    let foo = Foo(Bar::Baz("bar".to_owned()));
    let content = to_content(&foo.0).unwrap();
    assert_eq!(foo, from_content(content).unwrap());
}
