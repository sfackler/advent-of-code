use std::io::Write;

fn main() {
    let mut s = "1113122113".to_owned();
    for _ in 0..40 {
        s = step(&s);
    }
    println!("{}", s.len());

    for _ in 0..10 {
        s = step(&s);
    }
    println!("{}", s.len());
}

fn step(s: &str) -> String {
    let mut new = vec![];
    let mut last = '\0';
    let mut streak = 0;

    for b in s.chars() {
        if b != last {
            if streak != 0 {
                let _ = write!(new, "{}{}", streak, last);
            }
            last = b;
            streak = 1;
        } else {
            streak += 1;
        }
    }
    let _ = write!(new, "{}{}", streak, last);

    String::from_utf8(new).unwrap()
}
