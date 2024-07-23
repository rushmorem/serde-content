#![cfg(feature = "derive")]

use serde::Deserialize;
use serde::Serialize;
use serde_content::from_content;
use serde_content::to_content;
use serde_content::Content;
use serde_content::Data;
use serde_content::Struct;

#[test]
fn flatten() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo {
        bar: bool,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Baz {
        #[serde(flatten)]
        foo: Foo,
    }

    let foo = Foo { bar: true };
    let baz = Baz { foo };

    let content = to_content(&baz).unwrap();
    assert_eq!(baz, from_content(content).unwrap());
}

#[test]
fn skip() {
    #[derive(Debug, Serialize)]
    struct Foo {
        bar: bool,
        #[serde(skip)]
        _baz: usize,
    }

    let foo = Foo { bar: true, _baz: 9 };

    let content = to_content(&foo).unwrap();
    let expected = Content::Struct(Box::new(Struct {
        name: "Foo",
        data: Data::Struct {
            fields: vec![("bar", Content::Bool(true))],
        },
    }));
    assert_eq!(content, expected);
}

#[test]
fn untagged() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[serde(untagged)]
    enum Foo {
        Bar(usize),
    }

    let bar = 56;
    let foo = Foo::Bar(bar);

    let content = to_content(&foo).unwrap();
    assert_eq!(foo, from_content(content.clone()).unwrap());
    assert_eq!(bar, from_content(content).unwrap());
}