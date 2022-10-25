use std::io;
use std::collections::{HashMap, HashSet};

fn parse_n_and_m() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();
    (n, m)
}

fn parse_edge() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let from = iter.next().unwrap().parse().unwrap();
    let to = iter.next().unwrap().parse().unwrap();
    (from, to)
}

fn connected(graph: &HashMap<usize, Vec<usize>>, node1: usize, node2: usize) -> bool {
    let mut visited = HashSet::new();

    let mut current = vec![node1];
    while !current.is_empty() {
        let mut next = vec![];
        for node in current {
            if node == node2 {
                return true;
            }
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            if let Some(neighbors) = graph.get(&node) {
                next.extend(neighbors);
            }
        }
        current = next;
    }
    false
}

fn main() {
    let (n, m) = parse_n_and_m();

    if m > n {
        println!("NO");
        return;
    }
    
    let mut graph = HashMap::new();
    let mut threshold = 0;

    for _ in 0..m {
        let (from, to) = parse_edge();

        if graph.contains_key(&from) && graph.contains_key(&to) {
            if connected(&graph, from, to) {
                threshold += 1;
                if threshold > 1 {
                    println!("NO");
                    return;
                }
            }
        }

        graph.entry(to).or_insert(Vec::new()).push(from);
        graph.entry(from).or_insert(Vec::new()).push(to);
    }
    println!("YES");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_connected() {
        let mut graph = HashMap::new();
        link(&mut graph, 1, 2);
        link(&mut graph, 3, 4);
        assert!(!connected(&graph, 1, 3));
    }

    #[test]
    fn test_connected() {
        let mut graph = HashMap::new();
        link(&mut graph, 1, 2);
        link(&mut graph, 2, 3);
        link(&mut graph, 3, 4);
        assert!(connected(&graph, 1, 4));
    }

    fn link(graph: &mut HashMap<usize, Vec<usize>>, from: usize, to: usize) {
        graph.entry(from).or_insert(Vec::new()).push(to);
        graph.entry(to).or_insert(Vec::new()).push(from);
    }
}
