use std::io;
use std::io::BufRead;
use std::io::StdinLock;
use std::cmp::max;

#[derive(Debug)]
struct Game {
    win: bool,
    score: usize
}

impl Game {
    fn is_rivalry_with(&self, other: &Game) -> bool {
        if self.win && !other.win && self.score > other.score {
            return true;
        }
        if !self.win && other.win && self.score < other.score {
            return true;
        }
        false
    }
}

fn parse_number(handle: &mut StdinLock) -> usize {
    let mut line = String::new();
    handle.read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn parse_games(n: usize, handle: &mut StdinLock, games: &mut Vec<Game>) {
    let mut win_line = String::new();
    let mut score_line = String::new();

    handle.read_line(&mut win_line).unwrap();
    let mut wins = win_line.chars();

    handle.read_line(&mut score_line).unwrap();
    let mut scores = score_line.split_whitespace();

    for _ in 0..n {
        let win = wins.next().unwrap() == 'W';
        let score = scores.next().unwrap().parse::<usize>().unwrap();
        games.push(Game { win, score });
    }
}

fn solve(n: usize, geese_games: &Vec<Game>, hawks_games: &Vec<Game>) -> usize {
    let mut previous: Vec<usize> = vec![0; n + 1];
    let mut current: Vec<usize> = vec![0; n + 1];

    let (mut first, mut second, mut third, mut fourth);

    for i in 1..=n {
        for j in 1..=n {
            let geese = &geese_games[i-1];
            let hawks = &hawks_games[j-1];
            if geese.is_rivalry_with(hawks) {
                first = previous[j - 1] + geese.score + hawks.score;
            } else {
                first = 0;
            }
            second = previous[j];
            third = previous[j - 1];
            fourth = current[j - 1];
            current[j] = max(first, max(second, max(third, fourth)));
        }
        for j in 0..=n {
            previous[j] = current[j];
        }
    }
    current[n]
}

fn main() -> io::Result<()> {    
    let mut handle = io::stdin().lock();
    let n = parse_number(&mut handle);

    let mut geese_games = Vec::new();
    parse_games(n, &mut handle, &mut geese_games);

    let mut hawks_games = Vec::new();
    parse_games(n, &mut handle, &mut hawks_games);

    let totol_score = solve(n, &geese_games, &hawks_games);
    println!("{}", totol_score);

    Ok(())
}
