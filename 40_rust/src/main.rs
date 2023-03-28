use std::io::{self, BufRead};

struct FenwickTree {
    data: Vec<Vec<i32>>,
}

impl FenwickTree {
    fn new(m: usize, n: usize) -> Self {
        FenwickTree {
            data: vec![vec![0; n + 1]; m + 1],
        }
    }

    fn update(&mut self, mut r: usize, c: usize, x: i32) {
        while r < self.data.len() {
            let mut col = c;
            while col < self.data[0].len() {
                self.data[r][col] += x;
                col += col & (col.wrapping_neg());
            }
            r += r & (r.wrapping_neg());
        }
    }

    fn prefix_sum(&self, mut r: usize, c: usize) -> i32 {
        let mut sum = 0;
        while r > 0 {
            let mut col = c;
            while col > 0 {
                sum += self.data[r][col];
                col &= col - 1;
            }
            r &= r - 1;
        }
        sum
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let (m, n) = (dimensions[0], dimensions[1]);

    let mut black_tree = FenwickTree::new(m, n);
    let mut white_tree = FenwickTree::new(m, n);

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
                let is_black = (r + c) % 2 == 0;
                let delta = x - if is_black {
                    black_tree.prefix_sum(r, c)
                        - black_tree.prefix_sum(r - 1, c)
                        - black_tree.prefix_sum(r, c - 1)
                        + black_tree.prefix_sum(r - 1, c - 1)
                } else {
                    white_tree.prefix_sum(r, c)
                        - white_tree.prefix_sum(r - 1, c)
                        - white_tree.prefix_sum(r, c - 1)
                        + white_tree.prefix_sum(r - 1, c - 1)
                };
                if is_black {
                    black_tree.update(r, c, delta);
                } else {
                    white_tree.update(r, c, delta);
                }
            }
            2 => {
                let (r1, c1, r2, c2) = (
                    input[1] as usize,
                    input[2] as usize,
                    input[3] as usize,
                    input[4] as usize,
                );
                let is_black = (r1 + c1) % 2 == 0;
                let black_sum = black_tree.prefix_sum(r2, c2)
                    - black_tree.prefix_sum(r1 - 1, c2)
                    - black_tree.prefix_sum(r2, c1 - 1)
                    + black_tree.prefix_sum(r1 - 1, c1 - 1);
                let white_sum = white_tree.prefix_sum(r2, c2)
                    - white_tree.prefix_sum(r1 - 1, c2)
                    - white_tree.prefix_sum(r2, c1 - 1)
                    + white_tree.prefix_sum(r1 - 1, c1 - 1);
                let result = if is_black {
                    black_sum - white_sum
                } else {
                    white_sum - black_sum
                };
                println!("{}", result);
            }
            _ => panic!("Invalid input"),
        }
    }
}
