use std::io;
use std::collections::HashMap;

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

fn find_longest_route(n: usize, paths: &HashMap<usize, Vec<Path>>) -> (usize, Route) {
    let mut cache = HashMap::new();
    let mut through = 0;
    let mut max = Route {
        from: 0,
        to: 0,
        distance: 0,
    };
    for i in 1..=n {
        let route = longest_route(i, paths, &mut cache);
        if route.distance > max.distance {
            max = route;
            through = i;
        }
    }
    (through, max)
}

fn remove_path(paths: &mut HashMap<usize, Vec<Path>>, from: usize, to: usize) {
    let mut i = 0;
    for path in paths.get_mut(&from).unwrap() {
        if path.to == to {
            paths.get_mut(&from).unwrap().remove(i);
            break;
        }
        i += 1;
    }
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut paths = HashMap::new();

    io::stdin().read_line(&mut buf)?;
    let n: usize = buf.trim().parse().unwrap();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf)?;
        let mut iter = buf.split_whitespace();
        let from: usize = iter.next().unwrap().parse().unwrap();
        let to: usize = iter.next().unwrap().parse().unwrap();
        let distance: usize = iter.next().unwrap().parse().unwrap();
        paths.entry(from).or_insert(Vec::new()).push(Path { to, distance });
    }

    Ok(())
}
