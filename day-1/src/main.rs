fn main() {
    let input = include_bytes!("input");

    let mut floor = 0;
    for &byte in &input[..] {
        match byte {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => {}
        }
    }

    println!("{}", floor);
}
