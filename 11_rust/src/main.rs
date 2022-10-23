use std::io;

fn parse_line() -> (usize, usize, usize) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let h = iter.next().unwrap().parse().unwrap();
    let j = iter.next().unwrap().parse().unwrap();
    let n = iter.next().unwrap().parse().unwrap();
    (h, j, n)
}

fn parse_section() -> (usize, usize) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    (a, b)
}

fn load_distances(rope: &mut Vec<i32>, h: usize, j: usize) {
    rope[0] = 0;
    let mut cur_positions = Vec::new();
    cur_positions.push(0);
    while !cur_positions.is_empty() {
        let mut next_positions = Vec::new();
        for cur_pos in cur_positions {
            if cur_pos < h && rope[cur_pos + j] == -2 {
                next_positions.push(cur_pos + j);
            }
            let start = if cur_pos > j { cur_pos - j } else { 0 };
            for next_pos in start..cur_pos {
                if rope[next_pos] == -2 {
                    next_positions.push(next_pos);
                }
            }
            for pos in next_positions.iter() {
                rope[*pos] = rope[cur_pos] + 1;
            }
        }
        cur_positions = next_positions;
    }
}

fn find_min_distance(rope: &Vec<i32>, h: usize, j: usize) -> Option<i32> {
    let mut min_distance = -1;
    for pos in h..h+j {
        if rope[pos] != -2 {
            if min_distance == -1 || rope[pos] < min_distance {
                min_distance = rope[pos];
            }
        }
    }
    if min_distance == -1 {
        None
    } else {
        Some(min_distance)
    }
}

fn main() {
    let (h, j, n) = parse_line();
    let mut rope = vec![-2; h + j];
    for _ in 0..n {
        let (a, b) = parse_section();
        for i in a..=b {
            rope.insert(i, -1);
        }
    }
    load_distances(&mut rope, h, j);
    match find_min_distance(&rope, h, j) {
        Some(distance) => println!("{}", distance),
        None => println!("-1"),        
    }
}
