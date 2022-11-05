use std::io;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn parse_n() -> usize {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    n.trim().parse().unwrap()
}

fn parse_line(max_heap: &mut BinaryHeap<usize>, min_heap: &mut BinaryHeap<Reverse<usize>>) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    iter.next();
    for num in iter {
        let num = num.parse().unwrap();
        min_heap.push(Reverse(num));
        max_heap.push(num);
    }
}

fn main() {
    // This is incorrect here, and min-max binary heap should be used instead
    let mut max_heap = BinaryHeap::new();
    let mut min_heap = BinaryHeap::new();

    let n = parse_n();
    
    let mut total_price = 0;
    
    for _ in 0..n {
        parse_line(&mut max_heap, &mut min_heap);
        let max = max_heap.pop().unwrap();
        let min = min_heap.pop().unwrap();
        total_price += max - min.0;
    }

    println!("{}", total_price);
}
