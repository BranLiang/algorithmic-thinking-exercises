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

fn find_root(graph: &HashMap<usize, Vec<usize>>, node: usize) -> usize {
    let mut root = node;
    while graph.contains_key(&root) {
        root = graph[&root][0];
    }
    root
}

fn find_roots(graph: &HashMap<usize, Vec<usize>>, nodes: &Vec<usize>) -> HashSet<usize> {
    let mut roots = HashSet::new();
    for node in nodes {
        roots.insert(find_root(graph, *node));
    }
    roots
}

fn main() {
    let (n, m) = parse_n_and_m();

    if m > n {
        println!("NO");
        return;
    }
    
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut nodes = HashSet::new();

    for _ in 0..m {
        let (from, to) = parse_edge();
        if graph.contains_key(&to) {
            nodes.insert(to);
        }
        graph.entry(to).or_insert(Vec::new()).push(from);
    }

    let mut counter = 0;
    for node in nodes {
        let parents = &graph[&node];
        let roots = find_roots(&graph, parents);
        counter += parents.len() - roots.len();
        if counter > 1 {
            println!("NO");
            break;
        }
    }
    if counter <= 1 {
        println!("YES");
    }
}
