use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let p: f64 = iter.next().unwrap().parse().unwrap();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let y: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();

    let mut x = 0;
    let mut total = a;
    while total < b {
        total = (total as f64 * (1.0 + p / 100.0)) as usize;
        x += 1;
    }

    println!("{}", x + y);
}
