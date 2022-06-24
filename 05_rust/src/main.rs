use std::io;

struct Human {
    name: String,
    children: Vec<String>,
}

struct Score {
    name: String,
    value: usize
}

impl Human {
    pub fn from_str(line: &str) -> Self {
        let mut iter = line.split_whitespace();
        let parent_name = iter.next().unwrap();
        let mut parent = Human::new(parent_name);
        let child_num: i32 = iter.next().unwrap().parse().unwrap();
        for _ in 0..child_num {
            let child_name = iter.next().unwrap();
            parent.children.push(child_name.to_string());
        }
        parent
    }

    pub fn new(name: &str) -> Self {
        Human { name: name.to_string(), children: Vec::new() }
    }

    pub fn score(&self, distance: i32, people: &Vec<Human>) -> usize {
        if distance == 1 {
            self.children.len()
        } else {
            let mut total = 0;
            for child in self.children.iter() {
                if let Some(child) = people.iter().find(|&p| p.name == *child ) {
                    total += child.score(distance - 1, people);
                }
            }
            total
        }
    }
}

fn main() {
    // Read the case number
    let mut buf = String::new();
    let handle = io::stdin();
    handle.read_line(&mut buf).unwrap();
    let case_num: usize = buf.trim().parse().unwrap();

    for j in 0..case_num {
        println!("Tree {}:", j + 1);
        let mut people: Vec<Human> = Vec::new();
        let mut scores: Vec<Score> = Vec::new();
        // Parsing the case configuration
        let mut buf = String::new();
        handle.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let line_count: usize = iter.next().unwrap().trim().parse().unwrap();
        let distance: i32 = iter.next().unwrap().trim().parse().unwrap();
        // Parsing lines
        for _ in 0..line_count {
            let mut buf = String::new();
            handle.read_line(&mut buf).unwrap();
            let person = Human::from_str(&buf);
            people.push(person);
        }
        // Calculating scores
        for person in people.iter() {
            let value = person.score(distance, &people);
            scores.push(Score { name: person.name.clone(), value });
        }
        // Sort
        scores.sort_by(|a, b| {
            if a.value != b.value {
                b.value.cmp(&a.value)
            } else {
                a.name.cmp(&b.name)
            }
        });
        // Output
        let mut i = 0;
        while i < 3 && i < line_count && scores[i].value > 0 {
            println!("{} {}", scores[i].name, scores[i].value);
            i += 1;
        }
        while i > 0 && i < line_count && scores[i].value == scores[i - 1].value {
            println!("{} {}", scores[i].name, scores[i].value);
            i += 1;
        }

        if j != case_num - 1 {
            println!();
        }
    }
}
