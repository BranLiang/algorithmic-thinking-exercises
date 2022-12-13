use std::io;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Packet {
    List(Vec<Box<Packet>>),
    Number(usize),
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl Packet {
    pub fn from_str(s: &str) -> Self {
        let mut chars = s.chars().peekable();
        Packet::from_chars(&mut chars)
    }

    fn from_chars(chars: &mut Peekable<Chars>) -> Self {
        match chars.next() {
            Some('[') => {
                let mut data = Vec::new();
                while let Some(c) = chars.peek() {
                    match c {
                        ',' => {
                            chars.next();
                            continue;
                        },
                        ']' => {
                            chars.next();
                            break;
                        },
                        _ => data.push(Box::new(Packet::from_chars(chars))),
                    }
                }
                Packet::List(data)
            },
            Some(n @ '0'..='9') => {
                let mut data = String::new();
                data.push(n);
                while let Some(n @ '0'..='9') = chars.peek() {
                    data.push(*n);
                    chars.next();
                }
                Packet::Number(data.parse().unwrap())
            },
            _ => panic!("Invalid packet"),
        }
    }
}

fn main() {
    let mut pairs = Vec::new();

    let mut pair: Vec<String> = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        pair.push(line);

        if pair.len() == 2 {
            let left = Packet::from_str(&pair[0]);
            let right = Packet::from_str(&pair[1]);
            pair.clear();
            pairs.push(Pair { left, right });
            continue;
        }
    }

    println!("{:#?}", pairs)
}
