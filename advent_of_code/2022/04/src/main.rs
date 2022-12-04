use std::io;

fn parse_assignments(input: &str) -> ((usize, usize), (usize, usize)) {
    let mut iter = input.split(',');
    let (x1, y1) = {
        let mut iter = iter.next().unwrap().split('-');
        (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
    };
    let (x2, y2) = {
        let mut iter = iter.next().unwrap().split('-');
        (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
    };
    ((x1, y1), (x2, y2))
}

// fn full_include(a: (usize, usize), b: (usize, usize)) -> bool {
//     (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1)
// }

fn overlap(a: (usize, usize), b: (usize, usize)) -> bool {
    (a.0 <= b.0 && a.1 >= b.0) || (b.0 <= a.0 && b.1 >= a.0)
}

fn main() {
    let mut count = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (a, b) = parse_assignments(&line);
        if overlap(a, b) {
            count += 1;
        }
    }
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assignments() {
        let input = "1-3,4-5";
        let ((x1, y1), (x2, y2)) = parse_assignments(input);
        assert_eq!(x1, 1);
        assert_eq!(y1, 3);
        assert_eq!(x2, 4);
        assert_eq!(y2, 5);
    }

    // #[test]
    // fn test_full_include() {
    //     let a = (1, 3);
    //     let b = (2, 4);
    //     let c = (1, 5);
    //     assert!(!full_include(a, b));
    //     assert!(!full_include(b, a));
    //     assert!(full_include(a, c));
    //     assert!(full_include(c, b));
    // }
}
