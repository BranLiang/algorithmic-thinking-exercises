use std::collections::VecDeque;
use std::io::{self, BufRead};

type Pos = (usize, usize);

fn bfs(grid: &[Vec<char>], k: usize, n: usize, start: Pos, end: Pos, max_jump: i32) -> Option<usize> {
    let mut visited = vec![vec![false; n]; k];
    let mut queue = VecDeque::new();
    visited[start.0][start.1] = true;
    queue.push_back((start, 0));

    while let Some((curr, jumps)) = queue.pop_front() {
        if curr == end {
            return Some(jumps);
        }

        for i in 0..k {
            for j in 0..n {
                let dist = (((i as i32 - curr.0 as i32).pow(2) + (j as i32 - curr.1 as i32).pow(2)) as f32).sqrt().ceil() as i32;
                if !visited[i][j] && grid[i][j] == '*' && dist <= max_jump {
                    visited[i][j] = true;
                    queue.push_back(((i, j), jumps + 1));
                }
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let k: usize = lines.next().unwrap().parse().unwrap();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let max_jump: i32 = lines.next().unwrap().parse().unwrap();

    let grid: Vec<Vec<char>> = (0..k)
        .map(|_| lines.next().unwrap().chars().collect())
        .collect();

    loop {
        let start_line = lines.next().unwrap();
        if start_line == "-1 -1" {
            break;
        }
        let start: Vec<usize> = start_line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let start_pos = (start[0] - 1, start[1] - 1);

        let end: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let end_pos = (end[0] - 1, end[1] - 1);

        match bfs(&grid, k, n, start_pos, end_pos, max_jump) {
            Some(jumps) => println!("{}", jumps),
            None => println!("THERE IS NO TRY"),
        }
    }
}

