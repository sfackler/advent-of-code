extern crate regex;

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
