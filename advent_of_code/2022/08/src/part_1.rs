use std::io;
use std::cmp::max;
use std::cmp::min;

const SIZE: usize = 5;

fn main() {
    let mut forest = Vec::new();
    let mut left = [[0; SIZE]; SIZE];
    let mut right = [[0; SIZE]; SIZE];
    let mut top = [[0; SIZE]; SIZE];
    let mut bottom = [[0; SIZE]; SIZE];
    let rows = SIZE;
    let cols = SIZE;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        forest.push(row);
    }

    // left
    for i in 0..rows {
        left[i][0] = forest[i][0];
    }

    for i in 0..rows {
        for j in 1..cols {
            left[i][j] = max(left[i][j - 1], forest[i][j]);
        }
    }

    // right
    for i in 0..rows {
        right[i][cols - 1] = forest[i][cols - 1];
    }

    for i in 0..rows {
        for j in (0..cols - 1).rev() {
            right[i][j] = max(right[i][j + 1], forest[i][j]);
        }
    }

    // top
    for j in 0..cols {
        top[0][j] = forest[0][j];
    }

    for j in 0..cols {
        for i in 1..rows {
            top[i][j] = max(top[i - 1][j], forest[i][j]);
        }
    }

    // bottom
    for j in 0..cols {
        bottom[rows - 1][j] = forest[rows - 1][j];
    }

    for j in 0..cols {
        for i in (0..rows - 1).rev() {
            bottom[i][j] = max(bottom[i + 1][j], forest[i][j]);
        }
    }

    let mut count = 0;
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let min_height = min(
                min(left[i][j - 1], right[i][j + 1]),
                min(top[i - 1][j], bottom[i + 1][j])
            );
            if min_height < forest[i][j] {
                count += 1;
            }
        }
    }

    let border = SIZE * SIZE - (SIZE - 2) * (SIZE - 2);
    println!("{}", count + border);
}
