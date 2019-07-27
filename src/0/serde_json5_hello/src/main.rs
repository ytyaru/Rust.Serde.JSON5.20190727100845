use json5;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = json5::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = json5::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
