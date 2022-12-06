use std::{io, collections::HashSet};

struct Marker {
    data: Vec<char>,
    size: usize
}

impl Marker {
    fn new() -> Self {
        Marker {
            data: Vec::new(),
            size: 0
        }
    }

    fn add(&mut self, c: char) {
        if self.size == 14 {
            self.data.remove(0);
        } else {
            self.size += 1;
        }

        self.data.push(c);
    }

    fn is_valid(&self) -> bool {
        let mut set = HashSet::new();
        for c in self.data.iter() {
            set.insert(*c);
        }

        set.len() == 14
    }
}

fn main() {
    let mut marker = Marker::new();
    let mut value = 0;

    let line = io::stdin().lines().next().unwrap().unwrap();
    for c in line.chars() {
        value += 1;
        marker.add(c);

        if marker.is_valid() {
            break;
        }
    }

    println!("{}", value);
}
