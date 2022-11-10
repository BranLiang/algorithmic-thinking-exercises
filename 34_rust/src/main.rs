use std::io;

fn parse_n_b() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    (n, b)
}

fn parse(n: usize, b: usize) -> (Vec<isize>, usize) {
    let mut res = Vec::new();
    let mut index = 0;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    for i in 1..=n {
        let x: isize = iter.next().unwrap().parse().unwrap();
        if x < b as isize {
            res.push(-1);
        } else if x == b as isize {
            res.push(0);
            index = i;
        } else {
            res.push(1);
        }
    }
    (res, index)
}

fn sums(v: &Vec<isize>) -> (Vec<isize>, isize, isize) {
    let mut res = Vec::new();
    let mut max = 0;
    let mut min = 0;
    res.push(0);
    let mut sum = 0;
    for x in v {
        sum += x;
        if sum > max {
            max = sum;
        }
        if sum < min {
            min = sum;
        }
        res.push(sum);
    }
    (res, max, min)
}

fn main() {
    let (n, b) = parse_n_b();
    let (v, i) = parse(n, b);
    let (s, max, min) = sums(&v);

    // i_a >= i, and i_b < i, and s[i_a] - s[i_b] == 0
    // size(i_a) * size(i_b)

    let mut counter = 0;

    for sum in min..=max {
        let mut i_a = 0;
        let mut i_b = 0;
        for index in i..=n {
            if s[index] == sum {
                i_a += 1;
            }
        }
        for index in 0..i {
            if s[index] == sum {
                i_b += 1;
            }
        }
        counter += i_a * i_b;
    }

    println!("{}", counter);
}
