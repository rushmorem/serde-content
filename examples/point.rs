#![cfg(feature = "derive")]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to the Content type.
    let serialized = serde_content::to_content(&point).unwrap();

    // Pretty print the serialised Content.
    dbg!(&serialized);

    // Convert the Content back to a Point.
    let deserialized: Point = serde_content::from_content(serialized).unwrap();

    // Pretty print the deserialised Point.
    dbg!(deserialized);
}
