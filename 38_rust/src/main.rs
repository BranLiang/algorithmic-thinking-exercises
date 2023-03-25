use std::io;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};

// Function to read input data
fn read_input<T: FromStr>() -> Vec<T> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
        .split_whitespace()
        .map(|x| x.parse::<T>().ok().unwrap())
        .collect()
}

// Struct to represent a passageway
struct Passageway {
    a: usize,
    b: usize,
    m: u32,
}

fn dfs(
    node: usize,
    parent: usize,
    tree: &HashMap<usize, Vec<(usize, u32)>>,
    chambers_with_items: &HashSet<usize>,
) -> (u32, bool) {
    let mut total_magic_points = 0;
    let mut items_found = false;

    if chambers_with_items.contains(&node) {
        items_found = true;
    }

    for &(child, m) in tree.get(&node).unwrap() {
        if child == parent {
            continue;
        }
        let (magic_points, found) = dfs(child, node, tree, chambers_with_items);
        if found {
            items_found = true;
            total_magic_points += m;
            total_magic_points += magic_points;
        }
    }

    (total_magic_points, items_found)
}

fn main() {
    // Read N and K
    let nk: Vec<usize> = read_input();
    let (n, k) = (nk[0], nk[1]);

    // Read passageways
    let mut passageways = Vec::with_capacity(n - 1);
    for _ in 0..(n - 1) {
        let abm: Vec<u32> = read_input();
        passageways.push(Passageway {
            a: abm[0] as usize,
            b: abm[1] as usize,
            m: abm[2],
        });
    }

    // Read chamber items
    let chambers_with_items: HashSet<usize> = (0..k).map(|_| read_input::<usize>()[0]).collect();

    // Build the tree
    let mut tree = HashMap::new();
    for passageway in &passageways {
        tree.entry(passageway.a).or_insert_with(Vec::new).push((passageway.b, passageway.m));
        tree.entry(passageway.b).or_insert_with(Vec::new).push((passageway.a, passageway.m));
    }

    // // Perform DFS to find the minimum magic points required
    let min_magic_points = dfs(1, 0, &tree, &chambers_with_items);

    println!("{}", min_magic_points.0);
}


