use std::cmp;

fn main() {
    let input = include_str!("input");

    let mut paper = 0;
    let mut ribbon = 0;
    for line in input.split("\n") {
        let mut it = line.split("x");
        let l: u32 = it.next().unwrap().parse().unwrap();
        let w: u32 = it.next().unwrap().parse().unwrap();
        let h: u32 = it.next().unwrap().parse().unwrap();

        let a = l * w;
        let b = w * h;
        let c = h * l;
        let min = cmp::min(a, cmp::min(b, c));
        paper += 2 * (a + b + c) + min;

        let a = l + w;
        let b = w + h;
        let c = h + l;
        let min = cmp::min(a, cmp::min(b, c));
        ribbon += l * w * h + 2 * min;
    }

    println!("{}", paper);
    println!("{}", ribbon);
}
