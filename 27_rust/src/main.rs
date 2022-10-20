use std::io::{self, StdinLock};
use std::io::BufRead;
use std::collections::{HashMap, HashSet};

type Map = HashMap<usize, Vec<Edge>>;

#[derive(Debug)]
struct Edge {
    to: usize,
    length: usize,
}

#[derive(Debug)]
struct Route {
    to: usize,
    through: usize,
    distance: usize,
}

fn parse(line: &str, nodes: &HashSet<usize>) -> (usize, Edge) {
    let mut parts = line.split_whitespace();

    let from = parts.next().unwrap().parse::<usize>().unwrap();
    let to = parts.next().unwrap().parse::<usize>().unwrap();
    let length = parts.next().unwrap().parse::<usize>().unwrap();

    if nodes.contains(&from) && !nodes.contains(&to) {
        return (from, Edge { to, length });
    }
    if nodes.contains(&to) && !nodes.contains(&from) {
        return (to, Edge { to: from, length });
    }
    if from < to {
        return (from, Edge { to, length });
    }
    (to, Edge { to: from, length })
}

fn load(handle: &mut StdinLock, edges: &mut Map, parents: &mut HashMap<usize, usize>, nodes: &mut HashSet<usize>) {
    for line in handle.lines() {
        let line = line.unwrap();
        let (from, edge) = parse(&line, &nodes);
        let to = edge.to;
        nodes.insert(from);
        nodes.insert(to);
        edges.entry(from).or_insert(Vec::new()).push(edge);
        parents.insert(to, from);
    }
}

fn remove_edge(from: usize, to: usize, edges: &mut Map) -> usize {
    let mut i = 0;
    let mut length = 0;
    for edge in edges.get_mut(&from).unwrap() {
        if edge.to == to {
            length = edge.length;
            edges.get_mut(&from).unwrap().remove(i);
            break;
        }
        i += 1;
    }
    length
}

fn add_edge(from: usize, to: usize, length: usize, edges: &mut Map) {
    edges.entry(from).or_insert(Vec::new()).push(Edge { to, length });
}

fn find_longest_wing_distance(from: usize, edges: &Map) -> (usize, usize) {
    let mut distance = 0;
    let mut to = from;
    match edges.get(&from) {
        Some(to_edges) => {
            for edge in to_edges {
                let (to2, distance2) = find_longest_wing_distance(edge.to, edges);
                if distance2 + edge.length > distance {
                    distance = distance2 + edge.length;
                    to = to2;
                }
            }
        },
        None => (),
    }
    (to, distance)
}

fn find_longest(from: usize, edges: &Map, parents: &HashMap<usize, usize>, paths: &mut Vec<(usize, usize)>, fill_paths: bool) -> usize {
    let mut longest = Route { to: from, through: from, distance: 0 };
    let mut second_longest = Route { to: from, through: from, distance: 0 };

    match edges.get(&from) {
        Some(to_edges) => {
            for edge in to_edges {
                let (to, distance) = find_longest_wing_distance(edge.to, edges);
                if distance + edge.length > longest.distance {
                    second_longest = longest;
                    longest = Route { to: to, through: edge.to, distance: distance + edge.length };
                } else if distance + edge.length > second_longest.distance {
                    second_longest = Route { to: to, through: edge.to, distance: distance + edge.length };
                }
            }
        },
        None => (),
    }

    if fill_paths {
        if second_longest.distance != 0 {
            paths.push((*parents.get(&longest.to).unwrap(), longest.to));
            paths.push((*parents.get(&second_longest.to).unwrap(), second_longest.to));
        } else {
            paths.push((from, longest.through));
            paths.push((*parents.get(&longest.to).unwrap(), longest.to));
        }
    }
    longest.distance + second_longest.distance
}

fn find_second_longest(n: usize, edges: &mut Map, parents: &HashMap<usize, usize>) -> usize {
    let mut longest = 0;
    let mut tmp_paths = Vec::new();
    let mut paths = Vec::new();
    for from in 1..n {
        let distance = find_longest(from, edges, parents, &mut tmp_paths, true);
        if distance > longest {
            paths.clear();
            paths.push(tmp_paths.pop().unwrap());
            paths.push(tmp_paths.pop().unwrap());
            longest = distance;
        }
    }

    let mut second_longest = 0;
    for (from, to) in paths {
        let length = remove_edge(from, to, edges);
        for from in 1..n {
            let distance = find_longest(from, edges, parents, &mut tmp_paths, false);
            if distance > second_longest {
                second_longest = distance;
            }
        }
        add_edge(from, to, length, edges);
    }
    second_longest
}

fn main() -> io::Result<()> {
    let mut edges = HashMap::new();
    let mut parents = HashMap::new();
    let mut nodes = HashSet::new();

    let mut handle = io::stdin().lock();

    // Read N
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();

    // Load edges and parents
    load(&mut handle, &mut edges, &mut parents, &mut nodes);
    
    println!("{}", find_second_longest(n, &mut edges, &parents));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "1 2 3";
        let mut nodes = HashSet::new();
        nodes.insert(1);
        let (from, edge) = parse(line, &nodes);
        assert_eq!(from, 1);
        assert_eq!(edge.to, 2);
        assert_eq!(edge.length, 3);
    }

    #[test]
    fn test_remove_edge() {
        let mut edges = HashMap::new();
        edges.insert(1, vec![Edge { to: 2, length: 1 }]);
        edges.insert(2, vec![Edge { to: 3, length: 1 }]);
        let n = remove_edge(1, 2, &mut edges);
        assert_eq!(n, 1);
        assert_eq!(edges.get(&1).unwrap().len(), 0);
        assert_eq!(edges.get(&2).unwrap().len(), 1);
    }

    #[test]
    fn test_find_longest_wing_distance() {
        let mut edges = HashMap::new();
        add_edge(1, 2, 1, &mut edges);
        add_edge(1, 3, 2, &mut edges);
        add_edge(2, 4, 2, &mut edges);
        let (to, distance) = find_longest_wing_distance(1, &edges);
        assert_eq!(to, 4);
        assert_eq!(distance, 3);
    }

    #[test]
    fn test_find_longest() {
        let mut edges = HashMap::new();
        add_edge(1, 2, 1, &mut edges);
        add_edge(1, 3, 2, &mut edges);
        add_edge(2, 4, 2, &mut edges);
        add_edge(4, 5, 1, &mut edges);
        let mut parents = HashMap::new();
        let mut paths = Vec::new();
        parents.insert(2, 1);
        parents.insert(3, 1);
        parents.insert(4, 2);
        parents.insert(5, 4);
        let distance = find_longest(1, &edges, &parents, &mut paths, true);
        assert_eq!(distance, 6);
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], (4, 5));
        assert_eq!(paths[1], (1, 3));
    }

    #[test]
    fn test_find_longest_with_single_route() {
        let mut edges = HashMap::new();
        add_edge(1, 2, 1, &mut edges);
        add_edge(2, 3, 2, &mut edges);
        add_edge(3, 4, 2, &mut edges);
        add_edge(4, 5, 1, &mut edges);
        let mut parents = HashMap::new();
        let mut paths = Vec::new();
        parents.insert(2, 1);
        parents.insert(3, 2);
        parents.insert(4, 3);
        parents.insert(5, 4);
        let distance = find_longest(2, &edges, &parents, &mut paths, true);
        assert_eq!(distance, 5);
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], (2, 3));
        assert_eq!(paths[1], (4, 5));
    }

    #[test]
    fn test_find_second_longest() {
        let mut edges = HashMap::new();
        add_edge(1, 2, 1, &mut edges);
        add_edge(1, 3, 2, &mut edges);
        add_edge(2, 4, 2, &mut edges);
        add_edge(4, 5, 1, &mut edges);
        let mut parents = HashMap::new();
        parents.insert(2, 1);
        parents.insert(3, 1);
        parents.insert(4, 2);
        parents.insert(5, 4);
        let distance = find_second_longest(5, &mut edges, &parents);
        assert_eq!(distance, 5);
    }

    #[test]
    fn test_find_second_longest_with_single_line() {
        let mut edges = HashMap::new();
        add_edge(1, 2, 1, &mut edges);
        add_edge(2, 3, 2, &mut edges);
        add_edge(3, 4, 3, &mut edges);
        add_edge(4, 5, 4, &mut edges);
        let mut parents = HashMap::new();
        parents.insert(2, 1);
        parents.insert(3, 2);
        parents.insert(4, 3);
        parents.insert(5, 4);
        let distance = find_second_longest(5, &mut edges, &parents);
        assert_eq!(distance, 9);
    }
}
