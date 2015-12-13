extern crate regex;

use std::collections::HashSet;
use regex::Regex;

fn main() {
    let input = include_str!("input");
    let vowels = Regex::new("[aeiou]").unwrap();
    let forbidden = Regex::new("ab|cd|pq|xy").unwrap();

    let mut count = 0;
    for line in input.lines() {
        if vowels.find_iter(line).count() >= 3 && duplicates(line) && forbidden.find(line).is_none() {
            count += 1;
        }
    }

    println!("{}", count);

    count = 0;
    for line in input.lines() {
        if dup_cluster(line) && split_dup(line) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn duplicates(s: &str) -> bool {
    let mut last = '\0';

    for ch in s.chars() {
        if ch == last {
            return true;
        }
        last = ch;
    }
    false
}

fn dup_cluster(s: &str) -> bool {
    let mut last: &[u8] = &[0, 0];
    let mut clusters = HashSet::new();
    for w in s.as_bytes().windows(2) {
        if clusters.contains(w) {
            return true;
        }
        clusters.insert(last);
        last = w;
    }
    false
}

fn split_dup(s: &str) -> bool {
    s.as_bytes().windows(3).any(|w| w[0] == w[2])
}
