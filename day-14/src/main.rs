use std::cmp;

fn main() {
    let input = include_str!("input");

    let mut reindeer = vec![];

    for line in input.lines() {
        let words = line.split(" ").collect::<Vec<_>>();

        reindeer.push(Reindeer {
            speed: words[3].parse().unwrap(),
            duration: words[6].parse().unwrap(),
            rest: words[13].parse().unwrap(),
        });
    }

    let max = reindeer.iter().map(|r| r.distance(2503)).max().unwrap();
    println!("{}", max);
}

struct Reindeer {
    speed: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn distance(&self, time: u32) -> u32 {
        let interval = self.duration + self.rest;
        let whole_intervals = time / interval;
        let slack = cmp::min(self.duration, time - (interval * whole_intervals));

        (whole_intervals * self.duration + slack) * self.speed
    }
}
