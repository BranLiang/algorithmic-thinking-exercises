use std::io;

fn parse_n_and_t() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let t = iter.next().unwrap().parse().unwrap();
    (n, t)
}

#[derive(Debug)]
struct Level {
    time: usize,
    experience: usize,
}

type Skill = Vec<Level>;

fn parse_levels(n: usize) -> Vec<Skill> {
    let mut skills = Vec::new();

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let mut skill = Vec::new();
        for _ in 0..l {
            let time = iter.next().unwrap().parse().unwrap();
            let experience = iter.next().unwrap().parse().unwrap();
            skill.push(Level { time, experience });
        }
        skills.push(skill);
    }

    skills
}

fn main() {
    let (n, t) = parse_n_and_t();
    let skills = parse_levels(n);

    println!("{:?}", skills);
}