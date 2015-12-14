extern crate serde_json;

use serde_json::Value;
use std::ops::Add;

fn main() {
    let input = include_str!("input");
    let parsed: Value = serde_json::de::from_str(input).unwrap();
    println!("{}", sum(&parsed));
}

fn sum(value: &Value) -> f64 {
    match *value {
        Value::Null | Value::Bool(_) | Value::String(_) => 0.,
        Value::I64(v) => v as f64,
        Value::U64(v) => v as f64,
        Value::F64(v) => v,
        Value::Array(ref vs) => vs.iter().map(sum).fold(0., Add::add),
        Value::Object(ref vs) => vs.values().map(sum).fold(0., Add::add),
    }
}
