use std::ops::Add;

fn main() {
    let input = include_bytes!("input");

    let it = || {
        input.iter()
             .map(|&b| {
                 match b {
                     b'(' => 1,
                     b')' => -1,
                     _ => 0
                 }
             })
    };

    let floor = it().fold(0, Add::add);
    println!("{}", floor);

    let index = it().scan(0, |a, b| { *a = *a + b; Some(*a) }).position(|x| x == -1).unwrap() + 1;
    println!("{}", index);
}
