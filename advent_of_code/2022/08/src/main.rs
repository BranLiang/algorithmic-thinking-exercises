use std::io;

const SIZE: usize = 99;

fn left(pos: (usize, usize), forest: &Vec<Vec<u32>>) -> u32 {
    let (x, y) = pos;
    let mut count = 0;

    for j in (0..y).rev() {
        if forest[x][j] < forest[x][y] {
            count += 1;
        }
        if forest[x][j] >= forest[x][y] {
            count += 1;
            break;
        }
    }
    count
}

fn right(pos: (usize, usize), forest: &Vec<Vec<u32>>, cols: usize) -> u32 {
    let (x, y) = pos;
    let mut count = 0;

    for j in y+1..cols {
        if forest[x][j] < forest[x][y] {
            count += 1;
        }
        if forest[x][j] >= forest[x][y] {
            count += 1;
            break;
        }
    }
    count
}

fn top(pos: (usize, usize), forest: &Vec<Vec<u32>>) -> u32 {
    let (x, y) = pos;
    let mut count = 0;

    for i in (0..x).rev() {
        if forest[i][y] < forest[x][y] {
            count += 1;
        }
        if forest[i][y] >= forest[x][y] {
            count += 1;
            break;
        }
    }
    count
}

fn bottom(pos: (usize, usize), forest: &Vec<Vec<u32>>, rows: usize) -> u32 {
    let (x, y) = pos;
    let mut count = 0;

    for i in x+1..rows {
        if forest[i][y] < forest[x][y] {
            count += 1;
        }
        if forest[i][y] >= forest[x][y] {
            count += 1;
            break;
        }
    }
    count
}

fn main() {
    let mut forest = Vec::new();
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

    let mut max = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let l = left((i, j), &forest);
            let r = right((i, j), &forest, cols);
            let t = top((i, j), &forest);
            let b = bottom((i, j), &forest, rows);

            let score = l * r * t * b;
            if score > max {
                max = score;
            }
        }
    }

    println!("{}", max);

}
