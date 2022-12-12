use std::{io, collections::HashMap};

type Map = Vec<Vec<char>>;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Location {
    row: usize,
    col: usize,
}

fn find_next_locations(cur: &Location, map: &Map) -> Vec<Location> {
    let mut locations = Vec::new();
    // Left
    if cur.col > 0 {
        if map[cur.row][cur.col - 1] as i32 <= map[cur.row][cur.col] as i32 + 1 {
            locations.push(
                Location { row: cur.row, col: cur.col - 1 }
            );
        }
    }

    // Right
    if cur.col < map[cur.row].len() - 1 {
        if map[cur.row][cur.col + 1] as i32 <= map[cur.row][cur.col] as i32 + 1 {
            locations.push(
                Location { row: cur.row, col: cur.col + 1 }
            );
        }
    }

    // Up
    if cur.row > 0 {
        if map[cur.row - 1][cur.col] as i32 <= map[cur.row][cur.col] as i32 + 1 {
            locations.push(
                Location { row: cur.row - 1, col: cur.col }
            );
        }
    }

    // Down
    if cur.row < map.len() - 1 {
        if map[cur.row + 1][cur.col] as i32 <= map[cur.row][cur.col] as i32 + 1 {
            locations.push(
                Location { row: cur.row + 1, col: cur.col }
            );
        }
    }

    locations
}

fn main() {
    let mut map: Map = Vec::new();

    let mut start = Location { row: 0, col: 0 };
    let mut dest = Location { row: 0, col: 0 };

    for (row, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        let mut current = Vec::new();
        for (col, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start.row = row;
                    start.col = col;
                    current.push('a');
                },
                'E' => {
                    dest.row = row;
                    dest.col = col;
                    current.push('z');
                },
                _ => {
                    current.push(c);
                }
            }
        }
        map.push(current);
    }

    let mut distances = HashMap::new();
    distances.insert(start.clone(), 0);

    let mut currect_locations = Vec::new();
    currect_locations.push(start);

    while !currect_locations.is_empty() {
        let mut next_locations = Vec::new();
        for cur in currect_locations {
            for location in find_next_locations(&cur, &map) {
                if !distances.contains_key(&location) {
                    distances.insert(location.clone(), distances[&cur] + 1);
                    next_locations.push(location);
                }
            }
        }
        currect_locations = next_locations;
    }

    println!("{}", distances[&dest]);
}
