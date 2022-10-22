use std::io;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Pawn {
    position: Position
}

impl Pawn {
    fn move_up(&mut self) {
        self.position.row += 1;
    }

    fn win(&self, board: &Board) -> bool {
        self.position.row >= board.rows
    }
}

#[derive(Debug, Clone)]
struct Knight {
    position: Position
}

impl Knight {
    fn next_round_knights(&self, board: &Board) -> Vec<Knight> {
        let mut knights = Vec::new();
        for (row_move, col_move) in [
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
        ] {
            let row = self.position.row as i32 + row_move;
            let col = self.position.col as i32 + col_move;
            if row > 0 && row <= board.rows as i32 && col > 0 && col <= board.cols as i32 {
                let position = Position { row: row as usize, col: col as usize };
                knights.push(Knight { position });
            }
        }
        knights
    }
}

#[derive(Debug)]
struct Board {
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
struct Game {
    board: Board,
    pawn: Pawn,
    knight: Knight,
    distances: HashMap<Position, usize>,
}

impl Game {
    fn new(board: Board, pawn: Pawn, knight: Knight) -> Game {
        Game {
            board,
            pawn,
            knight,
            distances: HashMap::new(),
        }
    }

    fn load_distances(&mut self) {
        self.distances.insert(self.knight.position.clone(), 0);
        let mut stack = vec![self.knight.clone()];
        while !stack.is_empty() {
            let knight = stack.pop().unwrap();
            let distance = self.distances_to(&knight.position).unwrap();
            for next_knight in knight.next_round_knights(&self.board) {
                if !self.distances.contains_key(&next_knight.position) {
                    self.distances.insert(next_knight.position.clone(), distance + 1);
                    stack.push(next_knight);
                }
            }
        }
    }

    fn distances_to(&self, position: &Position) -> Option<usize> {
        self.distances.get(position).map(|distance| *distance)
    }

    fn play(&self) {
        let mut pawn_distance = 0;
        let mut pawn = self.pawn.clone();
        while !pawn.win(&self.board) {
            pawn.move_up();
            pawn_distance += 1;

            if let Some(knight_distance) = self.distances_to(&pawn.position) {
                if (knight_distance <= pawn_distance) && (pawn_distance - knight_distance) % 2 == 0 {
                    println!("Win in {} knight move(s).", pawn_distance);
                    return;
                }
            }
        }

        pawn_distance = 0;
        pawn.position.row = self.pawn.position.row;
        pawn.move_up();
        while !pawn.win(&self.board) {
            pawn.move_up();
            pawn_distance += 1;

            if let Some(knight_distance) = self.distances_to(&pawn.position) {
                if (knight_distance <= pawn_distance) && (pawn_distance - knight_distance) % 2 == 0 {
                    println!("Stalemate in {} knight move(s).", pawn_distance);
                    return;
                }
            }
        }

        println!("Loss in {} knight move(s).", pawn_distance);
    }
}

fn game_start() -> Game {
    let rows = parse_number();
    let cols = parse_number();
    let board = Board { rows, cols };

    let position = Position { row: parse_number(), col: parse_number() };
    let pawn = Pawn { position };

    let position = Position { row: parse_number(), col: parse_number() };
    let knight = Knight { position };

    Game::new(board, pawn, knight)
}

fn parse_number() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = parse_number();
    for _ in 0..n {
        let mut game = game_start();
        game.load_distances();
        game.play();
    }
}
