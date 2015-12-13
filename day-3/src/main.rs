use std::collections::HashSet;

fn main() {
    let input = include_str!("input");

    let mut location = (0, 0);

    let mut locations = HashSet::new();
    locations.insert((0, 0));

    for direction in input.chars() {
        location = shift(location, direction);
        locations.insert(location);
    }

    println!("{}", locations.len());

    locations.clear();
    locations.insert((0, 0));
    let mut location_a = (0, 0);
    let mut location_b = (0, 0);
    for (i, direction) in input.chars().enumerate() {
        let location = if i % 2 == 0 {
            &mut location_a
        } else {
            &mut location_b
        };

        *location = shift(*location, direction);
        locations.insert(*location);
    }

    println!("{}", locations.len());
}

fn shift(mut location: (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        '^' => location.1 += 1,
        '>' => location.0 += 1,
        'v' => location.1 -= 1,
        '<' => location.0 -= 1,
        _ => {}
    }
    location
}
