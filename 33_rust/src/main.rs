use std::io;

fn is_palindrome(s: &str) -> bool {
    let mut chars = s.chars();
    while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
        if a != b {
            return false;
        }
    }
    true
}

fn maybe_palindrome(len: usize, sums: &Vec<usize>) -> bool {
    let left = sums[len / 2];
    if len % 2 == 0 {
        let right = sums[len] - sums[len / 2];
        left == right
    } else {
        let right = sums[len] - sums[len / 2 + 1];
        left == right
    }
}

fn existed(s: &str, len: usize) -> bool {
    s[1..].contains(&s[..len])
}

fn substring_sums(s: &str) -> Vec<usize> {
    let mut sums = Vec::new();
    sums.push(0);
    for (i, c) in s.chars().enumerate() {
        sums.push(sums[i] + c as usize - 'a' as usize + 1);
    }
    sums
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let sums = substring_sums(&input);

    for i in (1..input.len()).rev() {
        let s = &input[0..i];
        
        if maybe_palindrome(i, &sums) && is_palindrome(s) {
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

    #[test]
    fn test_substring_sums() {
        assert_eq!(substring_sums("abab"), vec![0, 1, 3, 4, 6]);
    }

    #[test]
    fn test_maybe_parlindrome() {
        assert_eq!(maybe_palindrome(1, &vec![0, 1, 3, 4, 6]), true);
        assert_eq!(maybe_palindrome(2, &vec![0, 1, 3, 4, 6]), false);
        assert_eq!(maybe_palindrome(3, &vec![0, 1, 3, 4, 6]), true);
    }
}
