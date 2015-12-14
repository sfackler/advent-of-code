fn main() {
    let input = include_bytes!("input");

    let mut out = 0;
    for line in input.split(|&b| b == b'\n') {
        out += diff(line);
    }
    println!("{}", out);
}

fn diff(s: &[u8]) -> u32 {
    #[derive(Copy, Clone)]
    enum State {
        Normal,
        Escape,
    }

    let mut state = State::Normal;
    let mut diff = 2;

    // skip leading and trailing "s
    for &b in &s[1..s.len() - 1] {
        match (state, b) {
            (State::Normal, b'\\') => state = State::Escape,
            (State::Escape, b'\\') |
            (State::Escape, b'\"') => {
                diff += 1;
                state = State::Normal;
            }
            (State::Escape, b'x') => {
                diff += 3;
                state = State::Normal;
            }
            _ => {}
        }
    }

    diff
}
