use std::{io, collections::HashMap};

struct Semester {
    number: usize,
    total_score: usize,
    pass_score: usize,
}

impl Semester {
    pub fn parse(input: &str) -> Semester {
        let mut iter = input.split_whitespace();
        let number = iter.next().unwrap().parse().unwrap();
        let total_score = iter.next().unwrap().parse().unwrap();
        let pass_score = iter.next().unwrap().parse().unwrap();
        Semester {
            number,
            total_score,
            pass_score,
        }
    }
}

fn parse_number() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn parse_line() -> Semester {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    Semester::parse(input.trim())
}

fn solve(n: usize, t: usize, p: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if n == 0 && t == 0 {
        return 1;
    }
    if n == 0 {
        return 0;
    }
    let mut total = 0;
    for m in p..=t {
        total += solve(n - 1, t - m, p, memo)
    }
    memo.insert((n, t), total);
    total
}

fn main() {
    let n = parse_number();

    for _ in 0..n {
        let semester = parse_line();

        let mut memo = HashMap::new();
        let result = solve(semester.number, semester.total_score, semester.pass_score, &mut memo);
        println!("{}", result);
    }
}
