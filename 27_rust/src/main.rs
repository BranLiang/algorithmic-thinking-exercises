use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

type Map = HashMap<usize, Vec<Edge>>;

#[derive(Debug)]
struct Edge {
    to: usize,
    length: usize,
}

fn parse_n() -> usize {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    n.trim().parse().unwrap()
}

fn parse_line() -> (usize, Edge) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut parts = line.split_whitespace();

    let from = parts.next().unwrap().parse::<usize>().unwrap();
    let to = parts.next().unwrap().parse::<usize>().unwrap();
    let length = parts.next().unwrap().parse::<usize>().unwrap();

    return (from, Edge { to, length });
}

fn load(edges: &mut Map, n: usize) {
    for _ in 1..n {
        let (from, edge) = parse_line();
        edges.entry(edge.to).or_insert(vec![]).push(Edge { to: from, length: edge.length });
        edges.entry(from).or_insert(vec![]).push(edge);
    }
}

fn longest_from(edges: &Map, start: usize, skip: &Vec<usize>) -> (usize, usize) {
    let mut visited = HashSet::new();
    for &s in skip {
        visited.insert(s);
    }

    let mut stack = vec![(start, 0)];
    let mut max = 0;
    let mut end_node = start;

    while let Some((node, length)) = stack.pop() {
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);
        if length > max {
            max = length;
            end_node = node;
        }

        for edge in edges.get(&node).unwrap() {
            stack.push((edge.to, length + edge.length));
        }
    }

    (max, end_node)
}

fn longest_distance(edges: &Map, skip: &Vec<usize>) -> (usize, usize, usize) {
    let mut keys = edges.keys();
    let mut start = keys.next().unwrap();
    while skip.contains(start) {
        start = keys.next().unwrap();
    }

    let (_, start_node) = longest_from(edges, *start, skip);
    let (distance, end_node) = longest_from(edges, start_node, skip);
    (distance, start_node, end_node)
}

fn second_longest_distance(edges: &Map) -> usize {
    let (_, start_node, end_node) = longest_distance(edges, &vec![]);

    let mut max = 0;
    let (distance, _, _) = longest_distance(edges, &vec![start_node]);
    if distance > max {
        max = distance;
    }

    let (distance, _, _) = longest_distance(edges, &vec![end_node]);
    if distance > max {
        max = distance;
    }
    max
}

fn main() -> io::Result<()> {
    let mut edges = HashMap::new();

    // Input parsing
    let n = parse_n();
    load(&mut edges, n);

    println!("{}", second_longest_distance(&edges));

    Ok(())
}
