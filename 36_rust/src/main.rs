use std::io;
use std::collections::{HashSet, HashMap};

fn parse_rows_cols() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let rows = iter.next().unwrap().parse().unwrap();
    let cols = iter.next().unwrap().parse().unwrap();
    (rows, cols)
}

type Map = Vec<Vec<char>>;
type Position = (usize, usize);

fn portals(location: Position, map: &Map) -> HashSet<Position> {
    let mut portals = HashSet::new();
    let (row, col) = location;

    // Look up
    let mut row_1 = row - 1;
    while map[row_1][col] != '#' {
        row_1 -= 1;
    }
    portals.insert((row_1 + 1, col));

    // Look down
    let mut row_2 = row + 1;
    while map[row_2][col] != '#' {
        row_2 += 1;
    }
    portals.insert((row_2 - 1, col));

    // Look left
    let mut col_1 = col - 1;
    while map[row][col_1] != '#' {
        col_1 -= 1;
    }
    portals.insert((row, col_1 + 1));

    // Look right
    let mut col_2 = col + 1;
    while map[row][col_2] != '#' {
        col_2 += 1;
    }
    portals.insert((row, col_2 - 1));
    portals
}

fn next_locations(location: Position, map: &Map) -> HashSet<Position> {
    let mut next_locations = HashSet::new();
    let (row, col) = location;

    // Go up
    if map[row - 1][col] == '.' {
        next_locations.insert((row - 1, col));
    }

    // Go down
    if map[row + 1][col] == '.' {
        next_locations.insert((row + 1, col));
    }

    // Go left
    if map[row][col - 1] == '.' {
        next_locations.insert((row, col - 1));
    }

    // Go right
    if map[row][col + 1] == '.' {
        next_locations.insert((row, col + 1));
    }

    next_locations
}

fn min_distance_to_wall(location: Position, map: &Map) -> usize {
    let (row, col) = location;
    let mut min_distance = 0;
    let mut found = false;
    while !found {
        min_distance += 1;
        for i in 0..=min_distance {
            let row_diff = min_distance - i;
            let col_diff = i;
            if map[row - row_diff][col - col_diff] == '#' {
                found = true;
                break;
            }
            if map[row - row_diff][col + col_diff] == '#' {
                found = true;
                break;
            }
            if map[row + row_diff][col - col_diff] == '#' {
                found = true;
                break;
            }
            if map[row + row_diff][col + col_diff] == '#' {
                found = true;
                break;
            }
        }
    }
    min_distance - 1
}

fn solve(map: &Map, start: Position) {
    let mut distances: HashMap<Position, usize> = HashMap::new();
    distances.insert(start, 0);

    let mut current = HashMap::new();
    current.insert(start, portals(start, map));
    
    while !current.is_empty() {
        let mut next = HashMap::new();
        for (location, available_portals) in current.iter() {
            if map[location.0][location.1] == 'C' {
                println!("{}", distances[location]);
                return;
            }
            for next_location in next_locations(*location, map) {
                if !distances.contains_key(&next_location) || distances[&next_location] > distances[location] + 1 {
                    let mut next_portals = available_portals.clone();
                    next_portals.extend(portals(next_location, map));

                    distances.insert(next_location, distances[location] + 1);
                    next.insert(next_location, next_portals);
                }
            }

            let min_distance_to_wall = min_distance_to_wall(*location, map);
            for portal in available_portals {
                if !distances.contains_key(portal) || distances[portal] > distances[location] + min_distance_to_wall + 1 {
                    let mut next_portals = HashSet::from([*location]);
                    next_portals.extend(portals(*portal, map));

                    distances.insert(*portal, distances[location] + min_distance_to_wall + 1);
                    next.insert(*portal, next_portals);
                }
            }
        }
        current = next;
    }
}

fn main() {
    // Parse the number of rows and columns
    let (rows, cols) = parse_rows_cols();

    // Initialize the map with all walls, the labyrinth is surrounded by walls
    let mut map = vec![vec!['#'; cols + 2]; rows + 2];

    let mut start = (0, 0);

    // Parse the labyrinth
    for row in 1..rows + 1 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        for (col, c) in input.trim().chars().enumerate() {
            map[row][col + 1] = c;
            // Find the start location
            if c == 'S' {
                start = (row, col + 1);
            }
        }
    }

    // Print the map
    // for row in 0..rows + 2 {
    //     for col in 0..cols + 2 {
    //         print!("{}", map[row][col]);
    //     }
    //     println!("");
    // }

    solve(&map, start);
}
