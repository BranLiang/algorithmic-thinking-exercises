use std::io;

struct Move {
    amount: usize,
    from: usize,
    to: usize
}

fn parse_move(input: &str) -> Move {
    let mut parts = input.split_whitespace();
    parts.next();
    let amount = parts.next().unwrap().parse().unwrap();
    parts.next();
    let from = parts.next().unwrap().parse().unwrap();
    parts.next();
    let to = parts.next().unwrap().parse().unwrap();
    Move { amount, from, to }
}

fn operate(supplies: &mut Vec<Vec<char>>, m: &Move) {
    let mut crates = Vec::new();
    for _ in 0..m.amount {
        let from = &mut supplies[m.from];
        crates.push(from.pop().unwrap());
    }
    for item in crates.iter().rev() {
        supplies[m.to].push(*item);
    }
}

fn top_crates(supplies: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for i in 1..supplies.len() {
        let item = supplies[i].last().unwrap();
        result.push(*item);
    }
    result
}

fn main() {
    //         [M]     [B]             [N]
    // [T]     [H]     [V] [Q]         [H]
    // [Q]     [N]     [H] [W] [T]     [Q]
    // [V]     [P] [F] [Q] [P] [C]     [R]
    // [C]     [D] [T] [N] [N] [L] [S] [J]
    // [D] [V] [W] [R] [M] [G] [R] [N] [D]
    // [S] [F] [Q] [Q] [F] [F] [F] [Z] [S]
    // [N] [M] [F] [D] [R] [C] [W] [T] [M]
    //  1   2   3   4   5   6   7   8   9
    let mut supplies = Vec::new();
    supplies.push(vec![]);
    supplies.push(vec!['N', 'S', 'D', 'C', 'V', 'Q', 'T']);
    supplies.push(vec!['M', 'F', 'V']);
    supplies.push(vec!['F', 'Q', 'W', 'D', 'P', 'N', 'H', 'M']);
    supplies.push(vec!['D', 'Q', 'R', 'T', 'F']);
    supplies.push(vec!['R', 'F', 'M', 'N', 'Q', 'H', 'V', 'B']);
    supplies.push(vec!['C', 'F', 'G', 'N', 'P', 'W', 'Q']);
    supplies.push(vec!['W', 'F', 'R', 'L', 'C', 'T']);
    supplies.push(vec!['T', 'Z', 'N', 'S']);
    supplies.push(vec!['M', 'S', 'D', 'J', 'R', 'Q', 'H', 'N']);

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let m = parse_move(&line);
        operate(&mut supplies, &m);
    }
    println!("{}", top_crates(&supplies));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_move() {
        let m = parse_move("move 3 from 1 to 2");
        assert_eq!(m.amount, 3);
        assert_eq!(m.from, 1);
        assert_eq!(m.to, 2);
    }

    #[test]
    fn test_operate() {
        let mut supplies = Vec::new();
        supplies.push(vec![]);
        supplies.push(vec!['Z', 'N', 'D']);
        supplies.push(vec!['M', 'C']);
        supplies.push(vec!['P']);
        let m = Move { amount: 2, from: 1, to: 3 };
        operate(&mut supplies, &m);
        assert_eq!(supplies[1], vec!['Z']);
        assert_eq!(supplies[2], vec!['M', 'C']);
        assert_eq!(supplies[3], vec!['P', 'N', 'D']);
    }

    #[test]
    fn test_top_crates() {
        let mut supplies = Vec::new();
        supplies.push(vec![]);
        supplies.push(vec!['Z', 'N', 'D']);
        supplies.push(vec!['M', 'C']);
        supplies.push(vec!['P', 'D', 'N']);
        assert_eq!(top_crates(&supplies), "DCN");
    }
}
