use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Load(String),
    Count(String),
}

fn parse_instruction(cmd: &str) -> Instruction {
    let mut parts = cmd.split_whitespace();
    let cmd = parts.next().unwrap();
    let arg = parts.next().unwrap();
    match cmd {
        "1" => Instruction::Load(arg.to_string()),
        "2" => Instruction::Count(arg.to_string()),
        _ => panic!("Unknown command: {}", cmd),
    }
}

fn substrings(s: &str) -> HashSet<String> {
    let mut result = HashSet::new();
    for i in 0..s.len() {
        for j in i..s.len() {
            result.insert(s[i..j+1].to_string());
        }
    }
    result
}

fn main() -> io::Result<()> {
    let mut handle = io::stdin().lock();

    let mut buf = String::new();
    handle.read_line(&mut buf)?;
    let q = buf.trim().parse::<usize>().unwrap();

    let mut database = HashMap::new();

    for _ in 0..q {
        buf.clear();
        handle.read_line(&mut buf)?;
        let instruction = parse_instruction(&buf);
        match instruction {
            Instruction::Load(s) => {
                for sub in substrings(&s) {
                    let count = database.entry(sub).or_insert(0);
                    *count += 1;
                }
            },
            Instruction::Count(s) => {
                let count = database.get(&s).unwrap_or(&0);
                println!("{}", count);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substrings() {
        let set = substrings("abc");
        assert_eq!(set.len(), 6);
        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(set.contains("c"));
        assert!(set.contains("ab"));
        assert!(set.contains("bc"));
        assert!(set.contains("abc"));
    }

    #[test]
    fn test_substrings_dedup() {
        let set = substrings("abab");
        assert_eq!(set.len(), 7);
        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(set.contains("ab"));
        assert!(set.contains("ba"));
        assert!(set.contains("aba"));
        assert!(set.contains("bab"));
        assert!(set.contains("abab"));
    }
}
