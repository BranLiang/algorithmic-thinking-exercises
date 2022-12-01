use std::io;

fn main() {
    let mut calories = Vec::new();
    let mut current = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            calories.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    calories.sort();
    let sum: i32 = calories.iter().rev().take(3).sum();
    println!("{}", sum);
}
