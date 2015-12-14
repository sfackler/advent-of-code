extern crate serde_json;

use serde_json::Value;
use std::collections::BTreeMap;
use std::ops::Add;

fn main() {
    let input = include_str!("input");
    let parsed: Value = serde_json::de::from_str(input).unwrap();
    println!("{}", sum(&parsed, true));
    println!("{}", sum(&parsed, false));
}

fn sum(value: &Value, red_ok: bool) -> f64 {
    let s = |v| sum(v, red_ok);

    match *value {
        Value::Null | Value::Bool(_) | Value::String(_) => 0.,
        Value::I64(v) => v as f64,
        Value::U64(v) => v as f64,
        Value::F64(v) => v,
        Value::Array(ref vs) => vs.iter().map(s).fold(0., Add::add),
        Value::Object(ref vs) if red_ok || !has_red(vs) => vs.values().map(s).fold(0., Add::add),
        Value::Object(_) => 0.
    }
}

fn has_red(vs: &BTreeMap<String, Value>) -> bool {
    vs.values().flat_map(Value::as_string).any(|s| s == "red")
}
