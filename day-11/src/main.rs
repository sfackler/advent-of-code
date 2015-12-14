use std::str;

fn main() {
    let mut password = *b"hxbxwxba";
    approx_next(&mut password);

    while !has_streak(password) || !has_pairs(password) {
        approx_next(&mut password);
    }

    println!("{}", str::from_utf8(&password).unwrap());
}

fn approx_next(v: &mut [u8; 8]) {
    match v.iter().rposition(|b| *b != b'z') {
        Some(i) => {
            v[i] += 1;
            if v[i] == b'i' || v[i] == b'o' || v[i] == b'l' {
                v[i] += 1;
            }

            for b in &mut v[i + 1..] {
                *b = b'a';
            }
        }
        None => *v = [b'a'; 8],
    }
}

fn has_streak(v: [u8; 8]) -> bool {
    v.windows(3)
     .any(|w| w[1] == w[0] + 1 && w[2] == w[1] + 1)
}

fn has_pairs(v: [u8; 8]) -> bool {
    let mut last = 0;
    let mut count = 0;

    for &b in &v[..] {
        if last == b {
            count += 1;
            last = 0;
        } else {
            last = b;
        }
    }

    count >= 2
}
