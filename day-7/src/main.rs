extern crate lalrpop_util;

use std::collections::HashMap;
use ast::{Gate, Op, Source};
use std::cell::Cell;

mod ast;
mod parser;

fn main() {
    let input = include_str!("input");
    let raw = parser::parse_gates(input).unwrap();
    let graph = Graph::new(raw);
    let a = graph.evaluate("a");
    println!("{}", a);
    graph.reset();
    graph.ops[&"b"].value.set(Some(a));
    println!("{}", graph.evaluate("a"));
}

struct Node<'a> {
    op: Op<'a>,
    value: Cell<Option<u16>>,
}

struct Graph<'a> {
    ops: HashMap<&'a str, Node<'a>>,
}

impl<'a> Graph<'a> {
    fn new(raw: Vec<Gate<'a>>) -> Graph<'a> {
        let mut ops = HashMap::new();

        for gate in raw {
            let Gate { op, out } = gate;
            ops.insert(out, Node { op: op, value: Cell::new(None) });
        }

        Graph { ops: ops }
    }

    fn evaluate(&self, wire: &'a str) -> u16 {
        let node = &self.ops[&wire];

        if let Some(value) = node.value.get() {
            return value;
        }

        let value = match node.op {
            Op::Rshift(ref s, w) => self.evaluate_source(s) >> w,
            Op::Or(ref a, ref b) => self.evaluate_source(a) | self.evaluate_source(b),
            Op::Pass(ref s) => self.evaluate_source(s),
            Op::Not(ref s) => !self.evaluate_source(s),
            Op::And(ref a, ref b) => self.evaluate_source(a) & self.evaluate_source(b),
            Op::Lshift(ref s, w) => self.evaluate_source(s) << w,
        };

        node.value.set(Some(value));
        value
    }

    fn evaluate_source(&self, source: &Source<'a>) -> u16 {
        match *source {
            Source::Wire(w) => self.evaluate(w),
            Source::Lit(l) => l,
        }
    }

    fn reset(&self) {
        for op in self.ops.values() {
            op.value.set(None);
        }
    }
}
