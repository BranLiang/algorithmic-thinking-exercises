// https://cses.fi/problemset/task/1068

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let mut result = n;
    loop {
        print!("{}", result);
        if result == 1 {
            break;
        }
        print!(" ");
        if result % 2 == 0 {
            result /= 2;
        } else {
            result = result * 3 + 1;
        }
    }
    print!("\n");
}
