use std::io;
use std::iter::Peekable;
use std::str::Chars;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Box<Packet>>),
    Number(usize),
}

#[derive(Debug, Clone)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl Packet {
    fn to_list(&self) -> Self {
        match self {
            Packet::List(_) => self.clone(),
            Packet::Number(_) => {
                let mut list = Vec::new();
                list.push(Box::new(self.clone()));
                Packet::List(list)
            }
        }
    }

    fn compare(&self, other: &Self) -> Ordering {
        match self {
            Packet::List(data) => {
                match other {
                    Packet::List(other_data) => {
                        for (i, item) in data.iter().enumerate() {
                            if i >= other_data.len() {
                                break;
                            }
                            match item.compare(&other_data[i]) {
                                Ordering::Less => return Ordering::Less,
                                Ordering::Greater => return Ordering::Greater,
                                Ordering::Equal => continue
                            }
                        }
                        if data.len() < other_data.len() {
                            Ordering::Less
                        } else if data.len() > other_data.len() {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    },
                    Packet::Number(_) => {
                        self.compare(&other.to_list())
                    }
                }
            },
            Packet::Number(data) => {
                match other {
                    Packet::Number(other_data) => {
                        if data == other_data {
                            return Ordering::Equal;
                        } else if data < other_data {
                            return Ordering::Less;
                        } else {
                            return Ordering::Greater;
                        }
                    },
                    Packet::List(_) => {
                        self.to_list().compare(other)
                    },
                }
            },
        }
    }

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

    let mut indexes = Vec::new();
    for (i, pair) in pairs.iter().enumerate() {
        let index = i + 1;
        match pair.left.compare(&pair.right) {
            Ordering::Less => indexes.push(index),
            _ => continue,

        }
    }

    let mut sum = 0;
    for index in &indexes {
        sum += index;
    }

    println!("Part 1: {}", sum);

    let mut packets = Vec::new();
    packets.push(Packet::from_str("[[2]]"));
    packets.push(Packet::from_str("[[6]]"));
    for pair in &pairs {
        packets.push(pair.left.clone());
        packets.push(pair.right.clone());
    }
    packets.sort_by(|a, b| a.compare(b));
    let mut i = 0;
    let mut j = 0;
    for (index, packet) in packets.iter().enumerate() {
        if packet == &Packet::from_str("[[2]]") {
            i = index + 1;
        }
        if packet == &Packet::from_str("[[6]]") {
            j = index + 1;
        }
    }
    println!("Part 2: {}", j * i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_order() {
        let p1 = Packet::from_str("[1,1,3,1,1]");
        let p2 = Packet::from_str("[1,1,5,1,1]");
        assert_eq!(p1.compare(&p2), Ordering::Less);

        let p1 = Packet::from_str("[[1],[2,3,4]]");
        let p2 = Packet::from_str("[[1],4]");
        assert_eq!(p1.compare(&p2), Ordering::Less);

        let p1 = Packet::from_str("[[4,4],4,4]");
        let p2 = Packet::from_str("[[4,4],4,4,4]");
        assert_eq!(p1.compare(&p2), Ordering::Less);

        let p1 = Packet::from_str("[]");
        let p2 = Packet::from_str("[3]");
        assert_eq!(p1.compare(&p2), Ordering::Less);

        let p1 = Packet::from_str("[]");
        let p2 = Packet::from_str("[3]");
        assert_eq!(p1.compare(&p2), Ordering::Less);

        let p1 = Packet::from_str("[9]");
        let p2 = Packet::from_str("[[8,7,6]]");
        assert_eq!(p1.compare(&p2), Ordering::Greater);

        let p1 = Packet::from_str("[7,7,7,7]");
        let p2 = Packet::from_str("[7,7,7]");
        assert_eq!(p1.compare(&p2), Ordering::Greater);

        let p1 = Packet::from_str("[[[]]]");
        let p2 = Packet::from_str("[[]]");
        assert_eq!(p1.compare(&p2), Ordering::Greater);

        let p1 = Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let p2 = Packet::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(p1.compare(&p2), Ordering::Greater);
    }
}
