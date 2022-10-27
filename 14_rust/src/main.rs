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

type Bought = bool;
type From = usize;
type Dest = usize;
type Distance = usize;
type Count = usize;

fn load_min_distances(map: &HashMap<usize, Vec<Path>>, stores: &HashSet<usize>, from: usize) -> HashMap<(Dest, Bought), (Distance, Count)> {
    let mut state_distances: HashMap<(Dest, Bought), (Distance, Count)> = HashMap::new();
    let mut locked: HashSet<(Dest, Bought)> = HashSet::new();
    let mut new_nodes: HashSet<(From, Bought)> = HashSet::new();
    new_nodes.insert((from, false));

    state_distances.insert((from, false), (0, 1));

    let mut cur_locations: HashSet<(From, Bought)> = HashSet::new();
    cur_locations.insert((from, false));

    while !cur_locations.is_empty() {
        let mut remove_locations = Vec::new();

        let mut min_non_cookie_distance: Option<(Dest, Distance)> = None;
        let mut min_with_cookie_distance: Option<(Dest, Distance)> = None;

        for (location, bought) in cur_locations.iter() {
            let is_new = new_nodes.contains(&(*location, *bought));

            let mut counter = 0;
            let cur_distance: usize;
            let cur_count: usize;
            {
                let (distance, count) = *state_distances.get(&(*location, *bought)).unwrap();
                cur_distance = distance;
                cur_count = count;
            }
            for path in map.get(&location).unwrap() {
                let bought = *bought || stores.contains(&path.to);
                
                if locked.contains(&(path.to, bought)) {
                    continue;
                }
                counter += 1;
                let new_distance = cur_distance + path.distance;
                
                // Find the minimum distance for that round
                if bought {
                    match min_with_cookie_distance {
                        Some((_, min)) => {
                            if new_distance < min {
                                min_with_cookie_distance = Some((path.to, new_distance));
                            }
                        },
                        _ => {
                            min_with_cookie_distance = Some((path.to, new_distance));
                        }
                    }
                } else {
                    match min_non_cookie_distance {
                        Some((_, min)) => {
                            if new_distance < min {
                                min_non_cookie_distance = Some((path.to, new_distance));
                            }
                        },
                        _ => {
                            min_non_cookie_distance = Some((path.to, new_distance));
                        }
                    }
                }

                // Update the distance if it's lower
                if is_new {
                    if let Some((distance, count)) = state_distances.get_mut(&(path.to, bought)) {
                        if *distance > new_distance {
                            *distance = new_distance;
                            *count = cur_count;
                        } else if *distance == new_distance {
                            *count += cur_count;
                            *count %= 1000000;
                        }
                    } else {
                        state_distances.insert((path.to, bought), (new_distance, cur_count));
                    }
                }
            }

            if counter == 0 {
                remove_locations.push((*location, *bought));
            }
        }

        for (location, bought) in remove_locations {
            cur_locations.remove(&(location, bought));
        }

        new_nodes.clear();

        if let Some((location, _)) = min_non_cookie_distance {
            cur_locations.insert((location, false));
            locked.insert((location, false));
            new_nodes.insert((location, false));
        }

        if let Some((location, _)) = min_with_cookie_distance {
            cur_locations.insert((location, true));
            locked.insert((location, true));
            new_nodes.insert((location, true));
        }
    }

    state_distances
}

fn main() {
    let n = parse_number();
    let mut map = HashMap::new();
    load_distances(&mut map, n);

    parse_number();
    let mut stores = HashSet::new();
    load_stores(&mut stores);

    let min_distances_from_start = load_min_distances(&map, &stores, 1);

    min_distances_from_start.get(&(n, true)).map(|(distance, count)| {
        println!("{} {}", distance, count);
    });
}
