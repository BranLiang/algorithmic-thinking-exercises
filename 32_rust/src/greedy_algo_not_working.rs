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

#[derive(Debug, PartialEq, Eq)]
struct State {
    time: usize,
    experience: usize
}

fn load_experiences_over_time(skills: &Skill) -> Vec<State> {
    let mut experiences = Vec::new();
    let mut time = 0;
    let mut experience = 0;
    for level in skills {
        time += level.time;
        experience += level.experience;
        experiences.push(State { time, experience });
    }
    experiences
}

fn max_efficiency(experiences: &Vec<State>) -> (f64, usize) {
    let mut max = 0.0;
    let mut max_index = 0;
    for (i, s) in experiences.iter().enumerate() {
        let efficiency = s.experience as f64 / s.time as f64;
        if efficiency >= max {
            max = efficiency;
            max_index = i;
        }
    }
    (max, max_index)
}

fn next_skill_level(skill_experiences: &Vec<Vec<State>>, t: usize) -> Option<(usize, usize)> {
    let mut i = 0;
    let mut j = 0;
    let mut max = 0.0;
    let mut time = 0;
    for (i_idx, skill) in skill_experiences.iter().enumerate() {
        if skill.len() == 0 {
            continue;
        }
        let (efficiency, j_idx) = max_efficiency(skill);
        let time_used = skill[j_idx].time;
        if time_used > t {
            continue;
        }
        if efficiency > max || (efficiency == max && time_used > time) {
            max = efficiency;
            time = time_used;
            i = i_idx;
            j = j_idx;
        }
    }
    if max == 0.0 {
        None
    } else {
        Some((i, j))
    }
}

fn experience_learned(experiences: &mut Vec<State>, i: usize) {
    let s = &experiences[i];
    let experience = s.experience;
    let time = s.time;

    experiences.drain(0..=i);
    for e in experiences.iter_mut() {
        e.experience -= experience;
        e.time -= time;
    }
}

fn main() {
    let (n, t) = parse_n_and_t();
    let skills = parse_levels(n);

    let mut skill_experiences = Vec::new();
    for skill in skills {
        skill_experiences.push(load_experiences_over_time(&skill));
    }

    let mut total_experience = 0;
    let mut time_left = t;

    loop {
        match next_skill_level(&skill_experiences, time_left) {
            Some((i, j)) => {
                total_experience += skill_experiences[i][j].experience;
                time_left -= skill_experiences[i][j].time;
                experience_learned(&mut skill_experiences[i], j);
            },
            None => break
        }
    }

    println!("{}", total_experience);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_experiences_over_time() {
        let skill = vec![
            Level { time: 10, experience: 1 },
            Level { time: 10, experience: 19 },
            Level { time: 5, experience: 1 },
        ];
        let experiences = load_experiences_over_time(&skill);
        assert_eq!(experiences, vec![
            State { time: 10, experience: 1 },
            State { time: 20, experience: 20 },
            State { time: 25, experience: 21 },
        ]);
    }

    #[test]
    fn test_max_efficiency() {
        let experiences = vec![
            State { time: 10, experience: 1 },
            State { time: 20, experience: 20 },
            State { time: 25, experience: 21 },
        ];
        let (max, max_index) = max_efficiency(&experiences);
        assert_eq!(max, 1.0);
        assert_eq!(max_index, 1);
    }

    #[test]
    fn test_experience_learned() {
        let mut experiences = vec![
            State { time: 10, experience: 1 },
            State { time: 20, experience: 20 },
            State { time: 25, experience: 21 },
        ];
        experience_learned(&mut experiences, 1);
        assert_eq!(experiences, vec![
            State { time: 5, experience: 1 }
        ]);
    }

    #[test]
    fn test_next_skill_level() {
        let skill_experiences = vec![
            vec![
                State { time: 10, experience: 1 },
                State { time: 20, experience: 20 },
            ],
            vec![
                State { time: 10, experience: 10 },
            ],
            vec![
                State { time: 20, experience: 15 },
            ]
        ];
        assert_eq!(next_skill_level(&skill_experiences, 20), Some((0, 1)));
    }
}
