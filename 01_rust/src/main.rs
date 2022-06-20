const SHAPES: usize = 6;

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

fn main() {
    println!("Hello, world!");
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
}
