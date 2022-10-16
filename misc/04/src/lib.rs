use std::collections::HashMap;
use std::cmp::max;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn right(&self) -> Position {
        Position::new(self.x + 1, self.y)
    }

    fn down(&self) -> Position {
        Position::new(self.x, self.y + 1)
    }

    fn value(&self, grid: &Vec<Vec<i32>>) -> i32 {
        grid[self.x as usize][self.y as usize]
    }

    fn is_unreachable(&self, grid: &Vec<Vec<i32>>) -> bool {
        let n = grid.len() as i32;
        if self.x < 0 || self.x >= n || self.y < 0 || self.y >= n {
            return true;
        }
        self.value(grid) == -1
    }

    fn equals(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn count(dp: &mut HashMap<(Position, Position), i32>, grid: &Vec<Vec<i32>>, n: i32, player1: Position, player2: Position) -> i32 {
    if player1.is_unreachable(grid) || player2.is_unreachable(grid) {
        return i32::MIN;
    }

    match dp.get(&(player1, player2)) {
        Some(&ans) => {
            return ans;
        },
        None => {
            if player1.x == n - 1 && player1.y == n - 1 {
                return player1.value(grid);
            }
            let mut ans = player1.value(grid);
            if !player1.equals(&player2) {
                ans += player2.value(grid);
            }
            let option1 = count(dp, grid, n, player1.right(), player2.right());
            let option2 = count(dp, grid, n, player1.right(), player2.down());
            let option3 = count(dp, grid, n, player1.down(), player2.right());
            let option4 = count(dp, grid, n, player1.down(), player2.down());
            ans += max(max(option1, option2), max(option3, option4));
            dp.insert((player1, player2), ans);
            return ans;
        }
    }
}

fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len() as i32;
    let mut dp: HashMap<(Position, Position), i32> = HashMap::new();

    let v = count(&mut dp, &grid, n, Position::new(0, 0), Position::new(0, 0));

    max(0, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]];
        assert_eq!(cherry_pickup(grid), 5);
    }

    #[test]
    fn it_return_0() {
        let grid = vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]];
        assert_eq!(cherry_pickup(grid), 0);
    }
}
