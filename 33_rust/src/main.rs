use std::io;
use std::collections::HashSet;

fn is_palindrome(s: &str) -> bool {
    let mut chars = s.chars();
    while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
        if a != b {
            return false;
        }
    }
    true
}

fn existed(s: &str, len: usize) -> bool {
    let mut visited = HashSet::new();
    visited.insert(&s[0..len]);
    for j in 1..s.len() - len + 1 {
        let sub = &s[j..j + len];
        if visited.contains(sub) {
            return true;
        }
        visited.insert(sub);
    }
    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    for i in (1..input.len()).rev() {
        let s = &input[0..i];
        if is_palindrome(s) {
            if existed(&input, s.len()) {
                println!("{}", s.len());
                return;
            }
        }
    }
    println!("0");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("abba"), true);
        assert_eq!(is_palindrome("abab"), false);
    }

    #[test]
    fn test_existed() {
        assert_eq!(existed("ababab", 3), true);
        assert_eq!(existed("abab", 3), false);
        assert_eq!(existed("abab", 2), true);
        assert_eq!(existed("abab", 1), true);
    }
}
