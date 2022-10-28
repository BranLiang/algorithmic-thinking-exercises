use std::io;

fn parse_n_and_t() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let t = iter.next().unwrap().parse().unwrap();
    (n, t)
}

fn parse_levels() {
    
}

fn main() {
    println!("Hello, world!");
}
