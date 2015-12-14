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

    let mut scores = vec![0; reindeer.len()];
    for i in 1..2504 {
        let mut winners = vec![];
        let mut max = 0;
        for (i, score) in reindeer.iter().map(|r| r.distance(i)).enumerate() {
            if score > max {
                winners.clear();
                winners.push(i);
                max = score;
            } else if score == max {
                winners.push(i);
            }
        }
        for winner in winners {
            scores[winner] += 1;
        }
    }

    println!("{}", scores.iter().max().unwrap());
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
