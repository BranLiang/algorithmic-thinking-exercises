use std::io;
use std::collections::HashSet;

#[derive(Debug)]
enum Move {
    R(usize),
    L(usize),
    U(usize),
    D(usize),
}

impl Move {
    fn from_str(s: &str) -> Move {
        let (dir, dist) = s.trim().split_at(1);
        let dist = dist.trim().parse::<usize>().unwrap();
        match dir {
            "R" => Move::R(dist),
            "L" => Move::L(dist),
            "U" => Move::U(dist),
            "D" => Move::D(dist),
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

struct Knot {
    head: Position,
    next: Option<Box<Knot>>,
}

impl Knot {
    fn new() -> Knot {
        let mut head = Knot {
            head: Position { x: 0, y: 0 },
            next: None,
        };
        for _ in 0..9 {
            head = Knot {
                head: Position { x: 0, y: 0 },
                next: Some(Box::new(head)),
            };
        }
        head
    }

    fn tail(&self) -> Position {
        match self.next {
            Some(ref next) => next.tail(),
            None => self.head,
        }
    }

    fn move_up(&mut self) {
        self.head.y += 1;
        self.move_next();
    }

    fn move_down(&mut self) {
        self.head.y -= 1;
        self.move_next()
    }

    fn move_left(&mut self) {
        self.head.x -= 1;
        self.move_next()
    }

    fn move_right(&mut self) {
        self.head.x += 1;
        self.move_next()
    }

    fn move_next(&mut self) {
        match self.next {
            Some(ref mut next) => {
                let x_diff = self.head.x - next.head.x;
                let y_diff = self.head.y - next.head.y;
                if x_diff.abs() == 2 {
                    next.head.x += x_diff / 2;
                    if y_diff.abs() == 1 {
                        next.head.y += y_diff;
                    }
                }
                if y_diff.abs() == 2 {
                    next.head.y += y_diff / 2;
                    if x_diff.abs() == 1 {
                        next.head.x += x_diff;
                    }
                }
                next.move_next();
            },
            None => {}
        }
    }
}

fn main() {
    let mut knot = Knot::new();
    let mut visited = HashSet::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let m = Move::from_str(&line);
        match m {
            Move::R(dist) => {
                for _ in 0..dist {
                    knot.move_right();
                    let tail = knot.tail();
                    visited.insert((tail.x, tail.y));
                }
            }
            Move::L(dist) => {
                for _ in 0..dist {
                    knot.move_left();
                    let tail = knot.tail();
                    visited.insert((tail.x, tail.y));
                }
            }
            Move::U(dist) => {
                for _ in 0..dist {
                    knot.move_up();
                    let tail = knot.tail();
                    visited.insert((tail.x, tail.y));
                }
            }
            Move::D(dist) => {
                for _ in 0..dist {
                    knot.move_down();
                    let tail = knot.tail();
                    visited.insert((tail.x, tail.y));
                }
            }
        }
    }
    println!("{}", visited.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_new() {
        let k = Knot::new();
        assert_eq!(k.head, Position { x: 0, y: 0 });
        assert_eq!(k.tail(), Position { x: 0, y: 0 });
    }
}
