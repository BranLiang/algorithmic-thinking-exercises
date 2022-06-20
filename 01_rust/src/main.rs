use std::io;
use std::collections::{LinkedList, HashMap};

const SHAPES: usize = 6;
const SIZE: usize = 100_000;

type Snowflake = [i32;SHAPES];

fn identical_right(snow1: &Snowflake, snow2: &Snowflake, start: usize) -> bool {
    let mut iter = snow2.iter().cycle().skip(start);
    for shape1 in snow1 {
        if let Some(shape2) = iter.next() {
            if shape1 != shape2 {
                return false;
            }
        }
    }
    return true
}

fn identical_left(snow1: &Snowflake, snow2: &Snowflake, start: usize) -> bool {
    let mut iter = snow2.iter().rev().cycle().skip(SHAPES - start - 1);
    for shape1 in snow1 {
        if let Some(shape2) = iter.next() {
            if shape1 != shape2 {
                return false;
            }
        }
    }
    return true
}

fn are_identical(snow1: &Snowflake, snow2: &Snowflake) -> bool {
    for start in 0..SHAPES {
        if identical_left(snow1, snow2, start) {
            return true;
        }
        if identical_right(snow1, snow2, start) {
            return true;
        }
    }
    return false;
}

fn code(snow: &Snowflake) -> usize {
    (snow.iter().sum::<i32>() as usize) % SIZE
}

fn parse_input() -> HashMap<usize, LinkedList<Snowflake>> {
    // Initialize snowflakes collection
    let mut snowflakes: HashMap<usize, LinkedList<Snowflake>> = HashMap::new();

    // Read the line number
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    // Parse all the snowflakes
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut snowflake = [0;6];
        for (index, shape) in input.split_whitespace().enumerate() {
            snowflake[index] = shape.trim().parse().unwrap();
        }
        let code = code(&snowflake);
        match snowflakes.get_mut(&code) {
            Some(list) => {
                list.push_back(snowflake);
            },
            None => {
                let mut snowflake_list = LinkedList::new();
                snowflake_list.push_back(snowflake);
                snowflakes.insert(code, snowflake_list);
            }
        }
    }
    snowflakes
}

fn list_identical(snowflakes: &LinkedList<Snowflake>) -> bool {
    for (index, snow1) in snowflakes.iter().enumerate() {
        for snow2 in snowflakes.iter().skip(index + 1) {
            if are_identical(snow1, snow2) {
                return true;
            }
        }
    }
    return false;
}

fn identify_identical(snowflakes: &HashMap<usize, LinkedList<Snowflake>>) -> bool {
    for list in snowflakes.values() {
        if list_identical(list) {
            return true;
        }
    }
    return false;
}

fn main() {
    let snowflakes = parse_input();
    if identify_identical(&snowflakes) {
        println!("Twin snowflakes found.");
    } else {
        println!("No two snowflakes are alike.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_right() {
        let snow1 = [1, 2, 3, 4, 5, 6];
        let snow2 = [4, 5, 6, 1, 2, 3];
        assert!(identical_right(&snow1, &snow2, 3));
    }

    #[test]
    fn test_identical_left() {
        let snow1 = [1, 2, 3, 4, 5, 6];
        let snow2 = [2, 1, 6, 5, 4, 3];
        assert!(identical_left(&snow1, &snow2, 1));
    }

    #[test]
    fn test_are_identical() {
        let snow1 = [1, 2, 3, 4, 5, 6];
        let snow2 = [4, 5, 6, 1, 2, 3];
        assert!(are_identical(&snow1, &snow2));

        let snow1 = [1, 2, 3, 4, 5, 6];
        let snow2 = [1, 6, 5, 4, 3, 2];
        assert!(are_identical(&snow1, &snow2));
    }

    #[test]
    fn test_list_identical() {
        let mut list = LinkedList::new();
        list.push_back([1, 2, 3, 4, 5, 6]);
        list.push_back([4, 5, 6, 1, 3, 2]);
        list.push_back([1, 2, 1, 1, 1, 15]);
        list.push_back([4, 3, 2, 1, 6, 5]);
        assert!(list_identical(&list));

        let mut list = LinkedList::new();
        list.push_back([1, 2, 3, 4, 5, 6]);
        list.push_back([4, 5, 6, 1, 3, 2]);
        list.push_back([1, 2, 1, 1, 1, 15]);
        assert_eq!(false, list_identical(&list));
    }
}
