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

struct Rope {
    head: Position,
    tail: Position,
}

impl Rope {
    fn move_up(&mut self) {
        self.head.y += 1;
        self.move_tail();
    }

    fn move_down(&mut self) {
        self.head.y -= 1;
        self.move_tail()
    }

    fn move_left(&mut self) {
        self.head.x -= 1;
        self.move_tail()
    }

    fn move_right(&mut self) {
        self.head.x += 1;
        self.move_tail()
    }

    fn move_tail(&mut self) {
        let x_diff = self.head.x - self.tail.x;
        let y_diff = self.head.y - self.tail.y;
        if x_diff.abs() == 2 {
            self.tail.x += x_diff / 2;
            if y_diff.abs() == 1 {
                self.tail.y += y_diff;
            }
        }
        if y_diff.abs() == 2 {
            self.tail.y += y_diff / 2;
            if x_diff.abs() == 1 {
                self.tail.x += x_diff;
            }
        }
    }
}

fn main() {
    let mut rope = Rope {
        head: Position { x: 0, y: 0 },
        tail: Position { x: 0, y: 0 },
    };
    let mut visited = HashSet::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let m = Move::from_str(&line);
        match m {
            Move::R(dist) => {
                for _ in 0..dist {
                    rope.move_right();
                    visited.insert((rope.tail.x, rope.tail.y));
                }
            }
            Move::L(dist) => {
                for _ in 0..dist {
                    rope.move_left();
                    visited.insert((rope.tail.x, rope.tail.y));
                }
            }
            Move::U(dist) => {
                for _ in 0..dist {
                    rope.move_up();
                    visited.insert((rope.tail.x, rope.tail.y));
                }
            }
            Move::D(dist) => {
                for _ in 0..dist {
                    rope.move_down();
                    visited.insert((rope.tail.x, rope.tail.y));
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
    fn test_rope_move() {
        let mut rope = Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        rope.move_right();
        assert_eq!(rope.head, Position { x: 1, y: 0 });
        assert_eq!(rope.tail, Position { x: 0, y: 0 });
        rope.move_right();
        assert_eq!(rope.head, Position { x: 2, y: 0 });
        assert_eq!(rope.tail, Position { x: 1, y: 0 });
        rope.move_up();
        assert_eq!(rope.head, Position { x: 2, y: 1 });
        assert_eq!(rope.tail, Position { x: 1, y: 0 });
        rope.move_up();
        assert_eq!(rope.head, Position { x: 2, y: 2 });
        assert_eq!(rope.tail, Position { x: 2, y: 1 });
        rope.move_down();
        assert_eq!(rope.head, Position { x: 2, y: 1 });
        assert_eq!(rope.tail, Position { x: 2, y: 1 });
        rope.move_down();
        assert_eq!(rope.head, Position { x: 2, y: 0 });
        assert_eq!(rope.tail, Position { x: 2, y: 1 });
        rope.move_down();
        assert_eq!(rope.head, Position { x: 2, y: -1 });
        assert_eq!(rope.tail, Position { x: 2, y: 0 });
        rope.move_left();
        assert_eq!(rope.head, Position { x: 1, y: -1 });
        assert_eq!(rope.tail, Position { x: 2, y: 0 });
        rope.move_left();
        assert_eq!(rope.head, Position { x: 0, y: -1 });
        assert_eq!(rope.tail, Position { x: 1, y: -1 });
    }
}
