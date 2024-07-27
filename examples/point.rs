#![cfg(feature = "derive")]

use serde::{Deserialize, Serialize};
use serde_content::{Deserializer, Serializer};

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to the Content type.
    let serialized = Serializer::new().serialize(&point).unwrap();

    // Pretty print the serialised Content.
    dbg!(&serialized);

    // Convert the Content back to a Point.
    let deserialized: Point = Deserializer::new(serialized).deserialize().unwrap();

    // Pretty print the deserialised Point.
    dbg!(deserialized);
}
