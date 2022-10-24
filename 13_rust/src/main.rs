use std::{io, collections::{HashMap, HashSet}};

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn parse_number() -> usize {
    let line = read_line();
    line.trim().parse().unwrap()
}

type Cells = HashMap<usize, Vec<Passage>>;

#[derive(Debug)]
struct Passage {
    to: usize,
    cost: usize,
}

fn parse_passage(cells: &mut Cells) {
    let line = read_line();
    let mut iter = line.split_whitespace();
    let from = iter.next().unwrap().parse().unwrap();
    let to = iter.next().unwrap().parse().unwrap();
    let passage = Passage {
        to: from,
        cost: iter.next().unwrap().parse().unwrap(),
    };
    cells.entry(to).or_insert(vec![]).push(passage);
}

fn load_min_costs(cells: &Cells, start: usize) -> HashMap<usize, usize> {
    let mut min_costs = HashMap::new();
    min_costs.insert(start, 0);

    let mut locked: HashSet<usize> = HashSet::new();
    locked.insert(start);
    
    let mut cur_cells = HashSet::new();
    cur_cells.insert(start);
    while !cur_cells.is_empty() {
        let mut min_cost = 0;
        let mut min_cost_cell: Option<usize> = None;

        let mut remove_cells = vec![];
        for cell in cur_cells.iter() {
            let mut counter = 0;
            let passages = cells.get(&cell).unwrap();
            for passage in passages {
                if locked.contains(&passage.to) {
                    continue;
                }
                counter += 1;
                let cost = min_costs.get(cell).unwrap() + passage.cost;
                if cost < min_cost || min_cost == 0 {
                    min_cost = cost;
                    min_cost_cell = Some(passage.to);
                }
                match min_costs.get(&passage.to) {
                    Some(min_cost) => {
                        if cost < *min_cost {
                            min_costs.insert(passage.to, cost);
                        }
                    },
                    None => {
                        min_costs.insert(passage.to, cost);
                    }
                }
            }
            if counter == 0 {
                remove_cells.push(*cell);
            }
        }
        for cell in remove_cells {
            cur_cells.remove(&cell);
        }
        if let Some(min_cost_cell) = min_cost_cell {
            cur_cells.insert(min_cost_cell);
            locked.insert(min_cost_cell);
        }
    }
    min_costs
}

fn main() {
    let cases = parse_number();
    for _ in 0..cases {
        let mut cells = HashMap::new();
        read_line();
        let _cells_count = parse_number();
        let exit_cell = parse_number();
        let time_limit = parse_number();
        let passages_count = parse_number();
        for _ in 0..passages_count {
            parse_passage(&mut cells);
        }
        let min_costs = load_min_costs(&cells, exit_cell);

        let mut total = 0;
        for (_, cost) in min_costs {
            if cost <= time_limit {
                total += 1;
            }
        }
        println!("{}", total);
    }
}
