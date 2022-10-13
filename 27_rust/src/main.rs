use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Path {
    to: usize,
    distance: usize,
}

#[derive(Clone, Copy)]
struct Route {
    from: usize,
    to: usize,
    distance: usize,
}

fn longest_route_to_leaf(from: usize, paths: &HashMap<usize, Vec<Path>>, cache: &mut HashMap<usize, Route>) -> Route {
    if let Some(&route) = cache.get(&from) {
        return route;
    }

    let mut max = Route {
        from: from,
        to: from,
        distance: 0,
    };
    for path in paths.get(&from).unwrap() {
        let route = longest_route_to_leaf(path.to, paths, cache);
        let distance = path.distance + route.distance;
        if distance > max.distance {
            max.distance = distance;
            max.to = route.to;
        }
    }

    cache.insert(from, max.clone());
    max
}

fn longest_route(through: usize, paths: &HashMap<usize, Vec<Path>>, cache: &mut HashMap<usize, Route>) -> Route {
    let mut routes = Vec::new();
    for path in paths.get(&through).unwrap() {
        let route = longest_route_to_leaf(path.to, paths, cache);
        routes.push(Route {
            from: through,
            to: route.to,
            distance: path.distance + route.distance,
        });
    }

    if routes.len() < 1 {
        return Route {
            from: through,
            to: through,
            distance: 0,
        };
    }
    
    if routes.len() == 1 {
        routes[0]
    } else {
        routes.sort_by(|a, b| b.distance.cmp(&a.distance));
        Route {
            from: routes[0].to,
            to: routes[1].to,
            distance: routes[0].distance + routes[1].distance,
        }
    }
}

fn find_longest_route(n: usize, paths: &HashMap<usize, Vec<Path>>) -> Route {
    let mut cache = HashMap::new();
    let mut max = Route {
        from: 0,
        to: 0,
        distance: 0,
    };
    for i in 1..=n {
        let route = longest_route(i, paths, &mut cache);
        if route.distance > max.distance {
            max = route;
        }
    }
    max
}

fn remove_path(paths: &mut HashMap<usize, Vec<Path>>, from: Option<usize>, to: usize) {
    let mut i = 0;
    match from {
        Some(from) => {
            let paths = paths.get_mut(&from).unwrap();
            while i < paths.len() {
                if paths[i].to == to {
                    paths.remove(i);
                    break;
                }
                i += 1;
            }
        },
        None => {
            paths.get_mut(&to).unwrap().clear();
            paths.entry(to).or_insert(Vec::new());
        }
    }
}


fn solve(n: usize, paths: &mut HashMap<usize, Vec<Path>>, lookup: &HashMap<usize, usize>, cloned_paths: &mut HashMap<usize, Vec<Path>>) {
    let route = find_longest_route(n, paths);

    match lookup.get(&route.from) {
        Some(&from) => remove_path(paths, Some(from), route.from),
        // If not found, it's the root node
        None => remove_path(paths, None, route.from),
    }

    let route_a = find_longest_route(n, paths);

    match lookup.get(&route.to) {
        Some(&to) => remove_path(cloned_paths, Some(to), route.to),
        // If not found, it's the root node
        None => remove_path(cloned_paths, None, route.to),
    }

    let route_b = find_longest_route(n, cloned_paths);
    if route_a.distance > route_b.distance {
        println!("{}", route_a.distance);
    } else {
        println!("{}", route_b.distance);
    }
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut paths = HashMap::new();
    let mut cloned_paths = HashMap::new();
    let mut lookup = HashMap::new();

    io::stdin().read_line(&mut buf)?;
    let n: usize = buf.trim().parse().unwrap();

    for _ in 1..n {
        buf.clear();
        io::stdin().read_line(&mut buf)?;
        let mut iter = buf.split_whitespace();
        let from: usize = iter.next().unwrap().parse().unwrap();
        let to: usize = iter.next().unwrap().parse().unwrap();
        let distance: usize = iter.next().unwrap().parse().unwrap();
        paths.entry(from).or_insert(Vec::new()).push(Path { to, distance });
        paths.entry(to).or_insert(Vec::new());
        cloned_paths.entry(from).or_insert(Vec::new()).push(Path { to, distance });
        cloned_paths.entry(to).or_insert(Vec::new());
        lookup.insert(to, from);
    }

    solve(n, &mut paths, &lookup, &mut cloned_paths);

    Ok(())
}
