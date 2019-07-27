use json5;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct J5 {
    unquoted: String,
    singleQuotes: String,
    lineBreaks: String,
    hexadecimal: i32,
    leadingDecimalPoint: f64,
    andTrailing: f64,
    positiveSign: i32,
    trailingComma: String,
    andIn: Vec<String>,
    backwardsCompatible: String,
    inf: f64,
    ninf: f64,
    nan: f64,
    /*
    inf: i32,
    ninf: i32,
    nan: i32,
    */
}
fn main() {
    if let Ok(content) = std::fs::read_to_string("./test.json5") {
        let deserialized: J5 = json5::from_str(&content).unwrap();
        println!("deserialized = {:?}", deserialized);
    } else { println!("ファイル読み込みエラー。"); }
}
