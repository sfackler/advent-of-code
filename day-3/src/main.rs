use std::collections::HashSet;

fn main() {
    let input = include_str!("input");

    let mut location = (0, 0);

    let mut locations = HashSet::new();
    locations.insert((0, 0));

    for direction in input.chars() {
        match direction {
            '^' => location.1 += 1,
            '>' => location.0 += 1,
            'v' => location.1 -= 1,
            '<' => location.0 -= 1,
            _ => {}
        }
        locations.insert(location);
    }

    println!("{}", locations.len());
}
