use std::{io, collections::HashMap};

fn parse_t() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn parse_numbers() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    (a, b)
}

#[derive(Debug)]
struct Foxling {
    min: usize,
    max: usize,
}

fn solve(foxlings: &Vec<Foxling>, n: usize, m: usize, current: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&result) = cache.get(&(current, m)) {
        return result;
    }

    if current == n {
        if m == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let fox = &foxlings[current];
    let mut total = 0;
    for cracks in fox.min..fox.max + 1 {
        if m >= cracks {
            total += solve(foxlings, n, m - cracks, current + 1, cache);
        }
    }
    total = total % 1000000007;

    cache.insert((current, m), total);

    total
}

fn main() {
    let t = parse_t();
    for _ in 0..t {
        let (n, m) = parse_numbers();
        let mut foxlings = Vec::with_capacity(n);
        for _ in 0..n {
            let (min, max) = parse_numbers();
            foxlings.push(Foxling { min, max });
        }
        let mut cache = HashMap::new();
        let result = solve(&foxlings,n, m, 0, &mut cache);
        println!("{}", result);
    }
}
