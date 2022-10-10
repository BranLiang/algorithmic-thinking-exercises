use std::io::{self, BufRead};

fn found_diff_index(s1: &str, s2: &str) -> usize {
    for (i, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate() {
        if c1 != c2 {
            return i + 1;
        }
    }
    s1.len()
}

fn found_reverse_diff_index(s1: &str, s2: &str) -> usize {
    for (i, (c1, c2)) in s1.chars().rev().zip(s2.chars().rev()).enumerate() {
        if c1 != c2 {
            return s1.len() - i;
        }
    }
    1
}

fn main() -> io::Result<()> {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    let mut line1 = String::new();
    handle.read_line(&mut line1)?;

    let mut line2 = String::new();
    handle.read_line(&mut line2)?;

    let end_index = found_diff_index(&line1, &line2);
    let start_index = found_reverse_diff_index(&line1, &line2);

    if start_index > end_index {
        println!("0");
    } else {
        println!("{}", end_index - start_index + 1);
        for n in start_index..end_index {
            print!("{} ", n);
        }
        println!("{}", end_index);
    }

    Ok(())
}
