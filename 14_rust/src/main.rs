use std::io;
use std::collections::{HashMap, HashSet};

fn parse_number() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

#[derive(Debug)]
struct Path {
    to: usize,
    distance: usize,
}

fn load_distances(map: &mut HashMap<usize, Vec<Path>>, n: usize) {
    let mut input = String::new();
    let handle = io::stdin();
    for from in 1..=n {
        input.clear();
        handle.read_line(&mut input).unwrap();
        let parts = input.split_whitespace();
        for (j, distance) in parts.enumerate() {
            let to = j + 1;
            if to == from {
                continue;
            }
            let distance = distance.parse().unwrap();
            map.entry(from).or_insert_with(Vec::new).push(Path { to, distance });
        }
    }
}

fn load_stores(stores: &mut HashSet<usize>) {
    let mut input = String::new();
    let handle = io::stdin();
    handle.read_line(&mut input).unwrap();
    let parts = input.split_whitespace();
    for store in parts {
        let store = store.trim().parse().unwrap();
        stores.insert(store);
    }
}

fn solve(map: &HashMap<usize, Vec<Path>>, stores: &HashSet<usize>, from: usize, n: usize){
    let mut distances: [HashMap<usize, usize>; 2] = [HashMap::new(), HashMap::new()];
    let mut counts: [HashMap<usize, usize>; 2] = [HashMap::new(), HashMap::new()];
    // HashSet<(location_num, with_or_not)>
    // with_or_not: 0 -> no cookie, 1 -> with cookie
    let mut locked: HashSet<(usize, usize)> = HashSet::new();

    // Initialize
    distances[0].insert(from, 0);
    counts[0].insert(from, 1);

    let states: [usize; 2] = [0, 1];

    loop {
        for state in states {
            let mut min_distance = usize::max_value();
            let mut min_location = 0;

            for (location, distance) in distances[state].iter() {
                if *distance < min_distance && !locked.contains(&(*location, state)) {
                    min_distance = *distance;
                    min_location = *location;
                }
            }

            if min_location == 0 {
                continue;
            }

            locked.insert((min_location, state));

            let count = counts[state].get(&min_location).unwrap().clone();

            if state == 0 && stores.contains(&min_location) {
                let distance = distances[0].get(&min_location).unwrap().clone();
                distances[1].insert(min_location, distance);
                counts[1].insert(min_location, count);
            } else {
                let paths = map.get(&min_location).unwrap();

                for path in paths {
                    let new_distance = min_distance + path.distance;

                    match distances[state].get_mut(&path.to) {
                        Some(distance) => {
                            if new_distance < *distance {
                                *distance = new_distance;
                                counts[state].insert(path.to, count);
                            } else if new_distance == *distance {
                                let new_count = counts[state].get(&path.to).unwrap() + count;
                                let new_count = new_count % 1000000 ;
                                counts[state].insert(path.to, new_count);
                            }
                        }
                        None => {
                            distances[state].insert(path.to, new_distance);
                            counts[state].insert(path.to, count);
                        }
                    }
                }
            }

        }

        if locked.contains(&(n, 1)) {
            break;
        }
    }

    let distance = distances[1].get(&n).unwrap();
    let count = counts[1].get(&n).unwrap();
    println!("{} {}", distance, count);
}


fn main() {
    let n = parse_number();
    let mut map = HashMap::new();
    load_distances(&mut map, n);

    parse_number();
    let mut stores = HashSet::new();
    load_stores(&mut stores);

    solve(&map, &stores, 1, n);
}
