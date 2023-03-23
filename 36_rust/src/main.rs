use std::io::{self, BufRead};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::{thread, time};

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn is_open_block(&self, pos: &Position) -> bool {
        self.0[pos.r][pos.c] != '#'
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Position {
    r: usize,
    c: usize,
}

impl Position {
    fn new(r: usize, c: usize) -> Position {
        Position { r, c }
    }

    fn neighbors(&self, grid: &Grid) -> Vec<Position> {
        let mut neighbors = Vec::new();

        // move up
        let pos = Position::new(self.r - 1, self.c);
        if grid.is_open_block(&pos) {
            neighbors.push(pos);
        }

        // move down
        let pos = Position::new(self.r + 1, self.c);
        if grid.is_open_block(&pos) {
            neighbors.push(pos);
        }

        // move left
        let pos = Position::new(self.r, self.c - 1);
        if grid.is_open_block(&pos) {
            neighbors.push(pos);
        }

        // move right
        let pos = Position::new(self.r, self.c + 1);
        if grid.is_open_block(&pos) {
            neighbors.push(pos);
        }

        neighbors
    }
}

#[derive(Debug, Clone)]
struct State {
    pos: Position,
    portal1: Option<Position>,
    portal2: Option<Position>,
    dist: usize,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.portal1.hash(state);
        self.portal2.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(pos: Position) -> State {
        State {
            pos,
            portal1: None,
            portal2: None,
            dist: 0,
        }
    }

    fn with_portal1(&mut self, portal1: Position) {
        self.portal1 = Some(portal1);
    }

    fn with_portal2(&mut self, portal2: Position) {
        self.portal2 = Some(portal2);
    }

    fn neighbors(&self, grid: &Grid) -> Vec<State> {
        let mut neighbors = self.pos.neighbors(grid);

        // Teleport from portal1 to portal2
        if let Some(portal1) = &self.portal1 {
            if portal1 == &self.pos {
                if let Some(portal2) = &self.portal2 {
                    if !neighbors.contains(&portal2) && portal2 != &self.pos {
                        neighbors.push(portal2.clone());
                    }
                }
            }
        }

        // Teleport from portal2 to portal1
        if let Some(portal2) = &self.portal2 {
            if portal2 == &self.pos {
                if let Some(portal1) = &self.portal1 {
                    if !neighbors.contains(&portal1) && portal1 != &self.pos {
                        neighbors.push(portal1.clone());
                    }
                }
            }
        }

        let mut result = Vec::new();

        for neighbor in neighbors {
            let mut state = State::new(neighbor);
            state.dist = self.dist + 1;
            result.extend(state.possible_portal_states(grid));
        }

        result
    }

    fn possible_portal_states(&self, grid: &Grid) -> Vec<State> {
        let mut states = Vec::new();
        let portals = self.possible_portals(&grid);

        // Double portals persisted
        for portal1 in &portals {
            for portal2 in &portals {
                if portal1 < portal2 {
                    let mut state = self.clone();
                    // Set portal1
                    state.with_portal1(portal1.clone());
                    // Set portal2
                    state.with_portal2(portal2.clone());
                    states.push(state);
                }
            }
        }

        states
    }

    fn possible_portals(&self, grid: &Grid) -> HashSet<Position> {
        let mut portals = HashSet::new();

        // Move up
        let mut pos = self.pos.clone();
        while grid.is_open_block(&pos) {
            pos.r -= 1;
        }
        pos.r += 1;
        portals.insert(pos.clone());

        // Move down
        let mut pos = self.pos.clone();
        while grid.is_open_block(&pos) {
            pos.r += 1;
        }
        pos.r -= 1;
        portals.insert(pos.clone());

        // Move left
        let mut pos = self.pos.clone();
        while grid.is_open_block(&pos) {
            pos.c -= 1;
        }
        pos.c += 1;
        portals.insert(pos.clone());

        // Move right
        let mut pos = self.pos.clone();
        while grid.is_open_block(&pos) {
            pos.c += 1;
        }
        pos.c -= 1;
        portals.insert(pos.clone());

        if let Some(portal1) = &self.portal1 {
            portals.insert(portal1.clone());
        }

        if let Some(portal2) = &self.portal2 {
            portals.insert(portal2.clone());
        }

        portals
    }
}

fn solve(grid: &Grid, start: &Position, cake: &Position) {
    let mut visited = HashSet::new();
    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();

    let start_state = State::new(start.clone());
    visited.insert(start_state.clone());
    dist.insert(start_state.clone(), 0);
    queue.push(start_state);

    while let Some(state) = queue.pop() {
        // Debug
        thread::sleep(time::Duration::from_millis(100));

        println!("State: {:?}", state);
        for state in dist.iter() {
            println!("dist: {:?}", state);
        }
        println!("");

        if &state.pos == cake {
            println!("{}", state.dist);
            return;
        }

        // Update the minimum distance
        dist.insert(state.clone(), state.dist);

        // Move before firing portals
        for neighbor in state.neighbors(grid) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                queue.push(neighbor);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let rc: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // false: open, true: wall
    let mut maze: Grid = Grid(vec![vec!['#'; rc[1] + 2]; rc[0] + 2]);
    let mut start = (0, 0);
    let mut cake = (0, 0);

    for i in 0..rc[0] {
        let line = lines.next().unwrap();
        for (j, c) in line.chars().enumerate() {
            let i = i + 1;
            let j = j + 1;
            maze.0[i][j] = c;
            match c {
                'S' => {
                    start = (i, j);
                },
                'C' => {
                    cake = (i, j);
                },
                _ => {},
            }
        }
    }
    solve(&maze, &Position::new(start.0, start.1), &Position::new(cake.0, cake.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_open_block() {
        let grid = grid();
        assert_eq!(grid.is_open_block(&Position::new(1, 1)), true);
        assert_eq!(grid.is_open_block(&Position::new(1, 2)), false);
    }

    #[test]
    fn test_position_neighbors() {
        let grid = grid();
        let pos = Position::new(1, 1);
        let neighbors = pos.neighbors(&grid);
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors.contains(&Position::new(2, 1)), true);

        let pos = Position::new(3, 2);
        let neighbors = pos.neighbors(&grid);
        assert_eq!(neighbors.len(), 3);
        assert_eq!(neighbors.contains(&Position::new(4, 2)), true);
        assert_eq!(neighbors.contains(&Position::new(3, 1)), true);
        assert_eq!(neighbors.contains(&Position::new(3, 3)), true);
    }

    #[test]
    fn test_state_hash() {
        let mut set = HashSet::new();
        set.insert(State::new(Position::new(1, 1)));
        set.insert(State::new(Position::new(1, 1)));
        assert_eq!(set.len(), 1);

        let mut state = State::new(Position::new(1, 1));
        state.with_portal1(Position::new(1, 2));
        set.insert(state);
        assert_eq!(set.len(), 2);

        set.insert(State::new(Position::new(1, 2)));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_state_ord() {
        let mut heap = BinaryHeap::new();
        let mut state = State::new(Position::new(1, 1));
        state.dist = 1;

        heap.push(state.clone());

        state.dist = 2;
        heap.push(state.clone());

        state.dist = 0;
        heap.push(state.clone());

        state.dist = 1;
        heap.push(state.clone());

        assert_eq!(heap.pop().unwrap().dist, 0);
        assert_eq!(heap.pop().unwrap().dist, 1);
        assert_eq!(heap.pop().unwrap().dist, 1);
        assert_eq!(heap.pop().unwrap().dist, 2);
    }

    #[test]
    fn test_possible_portals() {
        let grid = grid();
        let state = State::new(Position::new(1, 1));
        let portals = state.possible_portals(&grid);
        assert_eq!(portals.len(), 2);
        assert_eq!(portals.contains(&Position::new(1, 1)), true);
        assert_eq!(portals.contains(&Position::new(4, 1)), true);
    }

    #[test]
    fn test_state_possible_portal_states() {
        let grid = grid();
        let mut state = State::new(Position::new(3, 3));
        state.with_portal1(Position::new(1, 1));
        state.with_portal2(Position::new(4, 4));
        let states = state.possible_portal_states(&grid);
        assert_eq!(states.len(), 15);
    }

    fn grid() -> Grid {
        Grid(vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', '.', '#', '.', 'C', '#'],
            vec!['#', '.', '#', '.', '#', '#'],
            vec!['#', '.', '.', '.', '.', '#'],
            vec!['#', 'S', '.', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
        ])
    }
}
