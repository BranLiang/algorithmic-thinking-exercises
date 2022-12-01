use std::io;

fn main() {
    let mut max = 0;
    let mut current = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    println!("{}", max);
}
