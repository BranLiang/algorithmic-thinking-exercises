use std::io;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Dictionary {
    passwords: HashMap<usize, Node>,
    counter: usize,
}

impl Dictionary {
    fn new() -> Self {
        Dictionary {
            passwords: HashMap::new(),
            counter: 0,
        }
    }
    
    fn insert(&mut self, passwd: &str) {
        let mut passwd = passwd;
        while passwd.len() > 0 {
            let key = passwd.len();
            match self.passwords.get_mut(&key) {
                Some(node) => node.insert(passwd, self.counter),
                None => {
                    let node = Node::new(passwd, self.counter);
                    self.passwords.insert(key, node);
                }
            }
            passwd = &passwd[1..];
        }
        self.counter += 1;
    }

    fn count(&self, passwd: &str) -> usize {
        let mut found = HashSet::new();
        let mut count = 0;
        for key in passwd.len()..=10 {
            match self.passwords.get(&key) {
                Some(tree) => count += tree.count(passwd, &mut found),
                None => (),
            }
        }
        count
    }
       
}

#[derive(Debug)]
struct Node {
    id: usize,
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: &str, id: usize) -> Node {
        Node {
            value: value.to_string(),
            left: None,
            right: None,
            id,
        }
    }

    fn insert(&mut self, value: &str, id: usize) {
        if value < &self.value {
            match self.left {
                Some(ref mut node) => node.insert(value, id),
                None => self.left = Some(Box::new(Node::new(value, id))),
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(value, id),
                None => self.right = Some(Box::new(Node::new(value, id))),
            }
        }
    }

    fn count(&self, value: &str, found: &mut HashSet<usize>) -> usize {
        let mut count = 0;
        if self.value.starts_with(value) {
            if !found.contains(&self.id) {
                count += 1;
                found.insert(self.id);
            }
        }
        match self.right {
            Some(ref node) => count += node.count(value, found),
            None => (),
        }
        if value < &self.value {
            match self.left {
                Some(ref node) => count += node.count(value, found),
                None => (),
            }
        }
        count
    }

}

fn main() -> io::Result<()> {
    // Read the size of the instructions
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let q = buf.trim().parse::<u32>().unwrap();

    // Intiailize the data structures for nodes
    let mut trees = Dictionary::new();

    // Handle each instruction
    for _ in 0..q {
        buf.clear();
        io::stdin().read_line(&mut buf)?;
        let mut iter = buf.split_whitespace();
        let op = iter.next().unwrap();
        let passwd = iter.next().unwrap();
        match op {
            "1" => {
                trees.insert(passwd);
            }
            "2" => {
                println!("{}", trees.count(passwd));
            }
            _ => panic!("Unknown operation"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_comparison() {
        assert!("abc" < "abd");
        assert!("abc" < "abcd");
        assert!("ab" < "abd");
        assert!("aa" < "ab");
        assert!("b" > "abc");
    }

    #[test]
    fn test_dictionary() {
        let mut dict = Dictionary::new();
        dict.insert("abc");
        dict.insert("ab");
        dict.insert("bc");
        assert_eq!(dict.count("ab"), 2);
        assert_eq!(dict.count("b"), 3);
        assert_eq!(dict.count("c"), 2);
        assert_eq!(dict.count("ac"), 0);
    }
}
