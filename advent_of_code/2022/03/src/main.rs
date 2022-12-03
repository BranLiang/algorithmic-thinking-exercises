use std::{io, collections::HashSet};

// fn parse_rucksack(line: &str) -> (Vec<char>, Vec<char>) {
//     let size = line.trim().len();
//     let (left, right) = line.split_at(size / 2);
//     let left = left.chars().collect();
//     let right = right.chars().collect();
//     (left, right)
// }

fn find_duplicates(left: &Vec<char>, right: &Vec<char>) -> HashSet<char> {
    let mut duplicates = HashSet::new();
    for c in left {
        if right.contains(c) {
            duplicates.insert(*c);
        }
    }
    duplicates
}

fn priority_for_item(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - 'a' as usize + 1,
        'A'..='Z' => item as usize - 'A' as usize + 27,
        _ => panic!("Invalid item: {}", item),
    }
}

fn main() {
    let mut sum = 0;

    let mut group: Vec<Vec<char>> = Vec::new();
    for line in io::stdin().lines() {
        group.push(line.unwrap().trim().chars().collect());
        if group.len() == 3 {
            let dup_1 = find_duplicates(&group[0], &group[1]);
            let dup_2 = find_duplicates(&group[0], &group[2]);
            dup_1.intersection(&dup_2).for_each(|c| {
                sum += priority_for_item(*c);
            });
            group.clear();
        }
    }
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse_rucksack() {
    //     let input = "ABCD";
    //     let (left, right) = parse_rucksack(input);
    //     assert_eq!(left, vec!['A', 'B']);
    //     assert_eq!(right, vec!['C', 'D']);
    // }

    #[test]
    fn test_find_duplicates() {
        let left = vec!['A', 'B', 'C'];
        let right = vec!['B', 'C', 'D'];
        let duplicates = find_duplicates(&left, &right);
        assert_eq!(duplicates.len(), 2);
        assert!(duplicates.contains(&'B'));
        assert!(duplicates.contains(&'C'));
    }

    #[test]
    fn test_priority_for_item() {
        assert_eq!(priority_for_item('a'), 1);
        assert_eq!(priority_for_item('z'), 26);
        assert_eq!(priority_for_item('A'), 27);
        assert_eq!(priority_for_item('Z'), 52);
    }
}
