use std::io;
use std::io::prelude::*;
use std::iter::Peekable;
use std::str::Chars;

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    candy: u32
}

impl Node {
    fn from_number(candy: u32) -> Self {
        Node {
            left: None,
            right: None,
            candy
        }
    }

    fn from_nodes(left: Box<Node>, right: Box<Node>) -> Self {
        Node { left: Some(left), right: Some(right), candy: 0 }
    }

    pub fn from_line(line: &str) -> Self {
        let mut chars = line.chars().peekable();
        Node::from_chars(&mut chars)
    }

    fn from_chars(chars: &mut Peekable<Chars>) -> Self {
        match chars.next() {
            Some('(') => {
                let left = Node::from_chars(chars);
                chars.next(); // skip the space in between
                let right = Node::from_chars(chars);
                chars.next(); // skip the right bracket `)
                Node::from_nodes(
                    Box::new(left),
                    Box::new(right)
                )
            },
            Some(candy_n1 @ '0'..='9') => {
                if let Some(candy_n2 @ '0'..='9') = chars.clone().peek() {
                    chars.next(); // handle the second digit;
                    Node::from_number(format!("{}{}", candy_n1, candy_n2).parse().unwrap())
                } else {
                    Node::from_number(candy_n1.to_digit(10).unwrap())
                }
            },
            _ => Node::from_chars(chars)
        }
    }

    fn is_leaf_node(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn candy_count(&self) -> u32 {
        if self.is_leaf_node() {
            self.candy
        } else {
            self.left.as_ref().unwrap().candy_count() + self.right.as_ref().unwrap().candy_count()
        }
    }

    fn height(&self) -> u32 {
        if self.is_leaf_node() {
            0
        } else {
            let left_height = self.left.as_ref().unwrap().height();
            let right_height = self.right.as_ref().unwrap().height();
            if left_height > right_height {
                left_height + 1
            } else {
                right_height + 1
            }
        }
    }

    fn street_count(&self) -> u32 {
        if self.is_leaf_node() {
            0
        } else {
            self.left.as_ref().unwrap().street_count() + self.right.as_ref().unwrap().street_count() + 4
        }
    }

    pub fn minimum_streets(&self) -> u32 {
        self.street_count() - self.height()
    }
}

fn main() {
    for line in io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                let root = Node::from_line(&line);
                println!("{} {}", root.minimum_streets(), root.candy_count())
            },
            Err(e) => println!("Error: {}", e)
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_line() {
        let node = Node::from_line("((5 2) 3)\n");
        let right = node.right.as_ref().unwrap();
        assert_eq!(3, right.candy);
        assert!(right.is_leaf_node());
        let left = node.left.as_ref().unwrap();
        assert_eq!(7, left.candy_count());
    }
}
