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
            map.entry(to).or_insert_with(Vec::new).push(Path { to: from, distance });
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
type Dest = usize;
type Distance = usize;

fn load_min_distances(map: &HashMap<usize, Vec<Path>>, stores: &HashSet<usize>, from: usize) -> HashMap<(Dest, Bought), Distance> {
    let mut state_distances: HashMap<(Dest, Bought), Distance> = HashMap::new();
    let mut locked: HashSet<(Dest, Bought)> = HashSet::new();

    // Definition for the bool key:
    // false means cookie not bought
    // true means cookie bought
    state_distances.insert((from, false), 0);

    let mut cur_locations = HashSet::new();
    cur_locations.insert((from, false));

    while !cur_locations.is_empty() {
        let mut remove_locations = Vec::new();

        let mut min_non_cookie_distance: Option<(Dest, Distance)> = None;
        let mut min_with_cookie_distance: Option<(Dest, Distance)> = None;

        for (location, bought) in cur_locations.iter() {
            let mut counter = 0;
            let cur_distance: usize;
            {
                cur_distance = *state_distances.get(&(*location, *bought)).unwrap();
            }
            for path in map.get(&location).unwrap() {
                let bought = *bought || stores.contains(&path.to);
                // Ignore locked path states
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
                match state_distances.get(&(path.to, bought)) {
                    Some(distance) => {
                        if new_distance < *distance {
                            state_distances.insert((path.to, bought), new_distance);
                        }
                    },
                    _ => {
                        state_distances.insert((path.to, bought), new_distance);
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

        if let Some((location, _)) = min_non_cookie_distance {
            cur_locations.insert((location, false));
            locked.insert((location, false));
        }

        if let Some((location, _)) = min_with_cookie_distance {
            cur_locations.insert((location, true));
            locked.insert((location, true));
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
    let min_distances_from_end = load_min_distances(&map, &stores, n);

    println!("{:?}", min_distances_from_start);
    println!("{:?}", min_distances_from_end);

    // let mut distances = Vec::new();
    // for ((to, bought), distance1) in min_distances_from_start {
    //     if to == n {
    //         continue;
    //     }
    //     if bought {
    //         match min_distances_from_end.get(&(to, bought)) {
    //             Some(distance2) => {
    //                 distances.push(distance1 + distance2);
    //             },
    //             _ => {}
    //         }
    //     }
    //     match min_distances_from_end.get(&(to, !bought)) {
    //         Some(distance2) => {
    //             distances.push(distance1 + distance2);
    //         },
    //         _ => {}
    //     }
    // }
    // distances.sort();
    // let mut routes_count = 0;
    // let min_distance = distances[0];
    // for distance in distances {
    //     if distance > min_distance {
    //         break;
    //     }
    //     routes_count += 1; 
    // }
    // println!("{} {}", min_distance, routes_count %  1000000);
}
