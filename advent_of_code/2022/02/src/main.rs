use std::io;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self, other: &Shape) -> i32 {
        match self {
            Shape::Rock => {
                let base = 1;
                match other {
                    Shape::Rock => base + 3,
                    Shape::Paper => base + 0,
                    Shape::Scissors => base + 6,
                }
            },
            Shape::Paper => {
                let base = 2;
                match other {
                    Shape::Rock => base + 6,
                    Shape::Paper => base + 3,
                    Shape::Scissors => base + 0,
                }
            },
            Shape::Scissors => {
                let base = 3;
                match other {
                    Shape::Rock => base + 0,
                    Shape::Paper => base + 6,
                    Shape::Scissors => base + 3,
                }
            },
        }
    }
}

fn parse_line(line: &str) -> (Shape, Shape) {
    let mut shapes = line.split_whitespace();
    let shape1 = match shapes.next().unwrap() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Invalid shape"),
    };
    let shape2 = match shapes.next().unwrap() {
        "X" => {
            match shape1 {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            }
        },
        "Y" => {
            match shape1 {
                Shape::Rock => Shape::Rock,
                Shape::Paper => Shape::Paper,
                Shape::Scissors => Shape::Scissors,
            }
        },
        "Z" => {
            match shape1 {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            }
        },
        _ => panic!("Invalid shape"),
    };
    (shape1, shape2)
}

fn main() {
    let mut score = 0;

    for line in io::stdin().lines() {
        let (other, me) = parse_line(&line.unwrap());
        score += me.score(&other);
    }

    println!("{}", score);
}
