use std::io;
use std::collections::HashMap;

struct Translation {
    to: String,
    price: usize,
}

fn parse_n_m() -> (i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse::<i32>().unwrap();
    let m = iter.next().unwrap().parse::<i32>().unwrap();
    (n, m)
}

fn parse_targets() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn parse_translation(translations: &mut HashMap<String, Vec<Translation>>) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: String = iter.next().unwrap().parse().unwrap();
    let b: String = iter.next().unwrap().parse().unwrap();
    let cost = iter.next().unwrap().parse::<usize>().unwrap();
    translations.entry(a.clone()).or_insert(vec![]).push(Translation {
        to: b.clone(),
        price: cost,
    });
    translations.entry(b).or_insert(vec![]).push(Translation {
        to: a,
        price: cost,
    });
}

fn generate_min_cost(translations: &HashMap<String, Vec<Translation>>) -> HashMap<String, (usize, usize)> {
    let mut min_move_costs = HashMap::new();
    min_move_costs.insert("English".to_string(), (0, 0));

    let mut new_moves: usize;

    let mut cur_langs = vec!["English".to_string()];
    while !cur_langs.is_empty() {
        let mut next_langs = vec![];
        for lang in cur_langs {
            {
                let (moves, _) = min_move_costs.get(&lang).unwrap();
                new_moves = *moves + 1;
            }
            for translation in translations.get(&lang).unwrap() {
                match min_move_costs.get(&translation.to) {
                    Some((min_moves, min_cost)) => {
                        if new_moves < *min_moves || (new_moves == *min_moves && translation.price < *min_cost) {
                            min_move_costs.insert(translation.to.clone(), (new_moves, translation.price));
                            next_langs.push(translation.to.clone());
                        }
                    },
                    None => {
                        min_move_costs.insert(translation.to.clone(), (new_moves, translation.price));
                        next_langs.push(translation.to.clone());
                    },
                }
            }
        }
        cur_langs = next_langs;
    }
    min_move_costs
}

fn main() {
    let (_, m) = parse_n_m();
    let targets = parse_targets();
    let mut translations = HashMap::new();
    for _ in 0..m {
        parse_translation(&mut translations);
    }
    let min_costs = generate_min_cost(&translations);

    let mut total_cost = 0;
    for target in targets {
        match min_costs.get(&target) {
            Some((_, cost)) => total_cost += cost,
            None => {
                println!("Impossible");
                return;
            }
        }
    }

    println!("{}", total_cost);
}
