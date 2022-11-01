use std::{io, collections::HashMap};

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

fn max(skills: &Vec<Skill>, t: usize, removed: Vec<usize>, memo: &mut HashMap<(usize, Vec<usize>), usize>) -> usize {
    if let Some(&result) = memo.get(&(t, removed.clone())) {
        return result;
    }

    let mut result = 0;

    for (i, skill) in skills.iter().enumerate() {
        if removed.contains(&i) {
            continue;
        }

        let mut time = 0;
        let mut exp = 0;

        for level in skill {
            time += level.time;  
            if time > t {
                break;
            }

            exp += level.experience;

            let mut new_removed = removed.clone();
            new_removed.push(i);
            new_removed.sort();

            let current_max = max(skills, t - time, new_removed, memo) + exp;
            if current_max > result {
                result = current_max;
            }
        }
    }

    memo.insert((t, removed), result);

    result
}

fn main() {
    let (n, t) = parse_n_and_t();
    let skills = parse_levels(n);

    let removed = Vec::new();
    let mut memo = HashMap::new();
    println!("{}", max(&skills, t, removed, &mut memo));
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
        assert_eq!(max(&skills, 20, vec![], &mut HashMap::new()), 20);
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
        assert_eq!(max(&skills, 32, vec![], &mut HashMap::new()), 22);
    }
}