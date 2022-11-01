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

fn max(skills: &Vec<Skill>, t: usize, skip: usize) -> usize {
    let mut result = 0;

    if skip == skills.len() {
        return result;
    }

    // Max experience if no level is learned for current skill
    let current_max = max(skills, t, skip+1);
    if current_max > result {
        result = current_max;
    }

    let skill = &skills[skip];

    let mut time = 0;
    let mut exp = 0;

    for level in skill {
        time += level.time;  
        if time > t {
            break;
        }

        exp += level.experience;

        let current_max = max(skills, t - time, skip+1) + exp;
        if current_max > result {
            result = current_max;
        }
    }

    result
}

fn main() {
    let (n, t) = parse_n_and_t();
    let skills = parse_levels(n);

    println!("{}", max(&skills, t, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let skills = vec![
            vec![
                Level { time: 10, experience: 1 },
                Level { time: 10, experience: 19 },
            ],
            vec![
                Level { time: 10, experience: 10 },
            ],
            vec![
                Level { time: 20, experience: 15 },
            ]
        ];
        assert_eq!(max(&skills, 20, 0), 20);
    }

    #[test]
    fn test_max_with_more_time_than_needed() {
        let skills = vec![
            vec![
                Level { time: 10, experience: 1 },
                Level { time: 10, experience: 19 },
            ],
            vec![
                Level { time: 15, experience: 12 },
            ],
            vec![
                Level { time: 15, experience: 10 },
            ]
        ];
        assert_eq!(max(&skills, 32, 0), 22);
    }
}