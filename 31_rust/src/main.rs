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
    if from < to {
        (from, to)
    } else {
        (to, from)
    }
}

fn find_edges(graph: &HashMap<usize, Vec<usize>>, node: usize) -> Vec<usize> {
    let mut edges = Vec::new();
    if !graph.contains_key(&node) {
        edges.push(node);
        return edges;
    }
    for parent in graph.get(&node).unwrap() {
        edges.extend(&find_edges(graph, *parent));
    }
    edges
}

fn connected(graph1: &HashMap<usize, Vec<usize>>, graph2: &HashMap<usize, Vec<usize>>, node1: usize, node2: usize) -> bool {
    let edges1 = find_edges(graph1, node1);
    let edges2 = find_edges(graph1, node2);
    let edges3 = find_edges(graph2, node1);
    let edges4 = find_edges(graph2, node2);

    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    for edge in edges1 {
        set1.insert(edge);
    }
    for edge in edges3 {
        set1.insert(edge);
    }
    for edge in edges2 {
        set2.insert(edge);
    }
    for edge in edges4 {
        set2.insert(edge);
    }
    set1.intersection(&set2).count() > 0
}

fn main() {
    let (n, m) = parse_n_and_m();

    if m > n {
        println!("NO");
        return;
    }
    
    let mut to_root_graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut to_leaf_graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut nodes: HashSet<usize> = HashSet::new();

    let mut counter = 0;

    for _ in 0..m {
        let (from, to) = parse_edge();

        if nodes.contains(&from) && nodes.contains(&to) {
            if connected(&to_root_graph, &to_leaf_graph, from, to) {
                counter += 1;
                if counter > 1 {
                    println!("NO");
                    return;
                }
            }
        }

        nodes.insert(from);
        nodes.insert(to);
        to_root_graph.entry(to).or_insert(Vec::new()).push(from);
        to_leaf_graph.entry(from).or_insert(Vec::new()).push(to);
    }
    println!("YES");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_edges() {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        graph.entry(2).or_insert(Vec::new()).push(1);
        graph.entry(5).or_insert(Vec::new()).push(2);
        graph.entry(5).or_insert(Vec::new()).push(4);
        graph.entry(4).or_insert(Vec::new()).push(3);
        graph.entry(2).or_insert(Vec::new()).push(6);
        let roots = find_edges(&graph, 5);
        assert_eq!(roots.len(), 3);
        assert!(roots.contains(&1));
        assert!(roots.contains(&3));
        assert!(roots.contains(&6));
    }

    #[test]
    fn test_find_edges_with_no_parents() {
        let graph: HashMap<usize, Vec<usize>> = HashMap::new();
        let roots = find_edges(&graph, 5);
        assert_eq!(roots.len(), 1);
        assert!(roots.contains(&5));
    }
}
