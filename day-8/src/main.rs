fn main() {
    let input = include_bytes!("input");

    let mut decode = 0;
    let mut encode = 0;
    for line in input.split(|&b| b == b'\n') {
        decode += decode_diff(line);
        encode += encode_diff(line);
    }
    println!("{}", decode);
    println!("{}", encode);
}

fn decode_diff(s: &[u8]) -> u32 {
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

fn encode_diff(s: &[u8]) -> usize {
    s.iter()
     .cloned()
     .filter(|&b| b == b'\\' || b == b'\"')
     .count() + 2
}
