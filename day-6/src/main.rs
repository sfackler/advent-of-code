#[macro_use]
extern crate nom;

use nom::{space, multispace, digit, IResult};
use std::ops::Range;
use std::str::{self, FromStr};
use std::cmp;

fn main() {
    let input = include_bytes!("input");

    let commands = match commands(input) {
        IResult::Done(_, c) => c,
        r => panic!("{:?}", r),
    };

    let mut count = 0;
    let mut brightness = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            let mut on = false;
            let mut b = 0;
            for &(ref kind, ref rect) in &commands {
                if rect.contains((x, y)) {
                    match *kind {
                        Kind::Off => {
                            on = false;
                            b = cmp::max(0, b - 1);
                        }
                        Kind::On => {
                            on = true;
                            b += 1;
                        }
                        Kind::Toggle => {
                            on = !on;
                            b += 2;
                        }
                    }
                }
            }
            if on {
                count += 1;
            }
            brightness += b;
        }
    }

    println!("{}", count);
    println!("{}", brightness);
}

#[derive(Debug)]
enum Kind {
    Off,
    On,
    Toggle,
}

#[derive(Debug)]
struct Rectangle {
    x: Range<u32>,
    y: Range<u32>,
}

impl Rectangle {
    fn contains(&self, point: (u32, u32)) -> bool {
        point.0 >= self.x.start && point.0 < self.x.end && point.1 >= self.y.start &&
            point.1 < self.y.end
    }
}

named!(commands(&[u8]) -> Vec<(Kind, Rectangle)>,
    many0!(chain!(c: command ~ multispace, || c))
);

named!(command<(Kind, Rectangle)>,
    chain!(kind: kind ~ space ~ rect: rectangle, || (kind, rect))
);

named!(kind<Kind>,
    alt!(
        chain!(tag!("turn off"), || Kind::Off) |
        chain!(tag!("turn on"), || Kind::On) |
        chain!(tag!("toggle"), || Kind::Toggle)
    )
);

named!(rectangle<Rectangle>,
    chain!(min: coord ~ space ~ tag!("through") ~ space ~ max: coord,
        || Rectangle { x: min.0..max.0 + 1, y: min.1..max.1 + 1 }
    )
);

named!(coord<(u32, u32)>,
    chain!(x: number ~ tag!(",") ~ y: number, || (x, y))
);

named!(number<u32>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));
