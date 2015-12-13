use std::cmp;

fn main() {
    let input = include_str!("input");

    let mut needed = 0;
    for line in input.split("\n") {
        let mut it = line.split("x");
        let l: u32 = it.next().unwrap().parse().unwrap();
        let w: u32 = it.next().unwrap().parse().unwrap();
        let h: u32 = it.next().unwrap().parse().unwrap();

        let a = l * w;
        let b = w * h;
        let c = h * l;

        let min = cmp::min(a, cmp::min(b, c));

        needed += 2 * (a + b + c) + min;
    }

    println!("{}", needed);
}
