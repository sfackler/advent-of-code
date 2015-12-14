extern crate permutohedron;

use permutohedron::Heap;
use std::collections::{HashMap, HashSet};
use std::cmp;

fn main() {
    let input = include_str!("input");

    let mut cities = HashSet::new();
    let mut distances = HashMap::new();

    for line in input.lines() {
        let mut it = line.split(" ");
        let start = it.next().unwrap();
        it.next();
        let end = it.next().unwrap();
        it.next();
        let distance: u32 = it.next().unwrap().parse().unwrap();

        cities.insert(start);
        cities.insert(end);
        distances.insert((start, end), distance);
        distances.insert((end, start), distance);
    }

    let mut cities = cities.into_iter().collect::<Vec<_>>();
    let mut cities = Heap::new(&mut cities);

    let mut min = u32::max_value();
    let mut max = 0;
    while let Some(cities) = cities.next_permutation() {
        let mut distance = 0;
        for jump in cities.windows(2) {
            distance += distances[&(jump[0], jump[1])];
        }
        max = cmp::max(max, distance);
        min = cmp::min(min, distance);
    }
    println!("{}", min);
    println!("{}", max);
}
