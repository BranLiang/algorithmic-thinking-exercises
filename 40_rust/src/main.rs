use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let (m, n) = (dimensions[0], dimensions[1]);

    let mut checkerboard = vec![vec![0; n]; m];

    for line in lines {
        let input_line = line.unwrap();
        let input: Vec<i32> = input_line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        match input[0] {
            0 => break,
            1 => {
                let (r, c, x) = (input[1] as usize, input[2] as usize, input[3]);
                checkerboard[r - 1][c - 1] = x;
            }
            2 => {
                let (r1, c1, r2, c2) = (
                    input[1] as usize,
                    input[2] as usize,
                    input[3] as usize,
                    input[4] as usize,
                );
                let mut sum = 0;
                for r in r1 - 1..r2 {
                    for c in c1 - 1..c2 {
                        if (r + c) % 2 == (r1 + c1) % 2 {
                            sum += checkerboard[r][c];
                        } else {
                            sum -= checkerboard[r][c];
                        }
                    }
                }
                println!("{}", sum);
            }
            _ => panic!("Invalid input"),
        }
    }
}
