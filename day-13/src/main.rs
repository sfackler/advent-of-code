extern crate permutohedron;

use permutohedron::Heap;
use std::collections::{HashMap, HashSet};
use std::cmp;

fn main() {
    let input = include_str!("input");

    let mut people = HashSet::new();
    let mut feels = HashMap::new();

    for line in input.lines() {
        let words = line.split(" ").collect::<Vec<_>>();
        let a = words[0];
        let b = words[10];
        let b = &b[..b.len() - 1];

        let mut units: i32 = words[3].parse().unwrap();
        if words[2] == "lose" {
            units *= -1;
        }

        people.insert(a);
        *feels.entry((a, b)).or_insert(0) += units;
        *feels.entry((b, a)).or_insert(0) += units;
    }

    let mut people = people.into_iter().collect::<Vec<_>>();
    let mut people = Heap::new(&mut people);

    let mut max = i32::min_value();
    while let Some(people) = people.next_permutation() {
        let mut delta = feels[&(people[0], people[people.len() - 1])];
        for pair in people.windows(2) {
            delta += feels[&(pair[0], pair[1])];
        }
        max = cmp::max(max, delta);
    }

    println!("{}", max);
}
