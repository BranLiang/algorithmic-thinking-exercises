use std::io;
use std::collections::{HashMap, HashSet};

type Rates = HashMap<String, usize>;
type Tunnels = HashMap<String, Vec<String>>;

fn parse_rate(input: &str) -> usize {
    let rate = input.strip_prefix("rate=").unwrap();
    let rate = rate.strip_suffix(';').unwrap();
    rate.parse().unwrap()
}

fn parse_line(rates: &mut Rates, tunnels: &mut Tunnels, line: &str) {
    let mut iter = line.split_whitespace();
    iter.next();
    let from = iter.next().unwrap();
    iter.next();
    iter.next();
    let rate = parse_rate(iter.next().unwrap());
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    let to: Vec<String> = iter.map(|s| {
        match s.strip_suffix(',') {
            Some(s) => s.to_string(),
            None => s.to_string(),
        }
    }).collect();
    rates.insert(from.to_string(), rate);
    tunnels.insert(from.to_string(), to);
}

fn build_key(time_left: usize, from: &str, release_pressure: usize, opened: &HashSet<String>) -> String {
    let mut key = format!("{} {} {} ", from, time_left, release_pressure);
    let mut sorted_opened: Vec<String> = opened.iter().map(|s| s.to_string()).collect();
    sorted_opened.sort();
    for v in sorted_opened {
        key.push_str(&v);
    }
    key
}

fn max_pressure(
    rates: &Rates,
    tunnels: &Tunnels,
    opened: &HashSet<String>,
    time_left: usize,
    from: &str,
    release_pressure: usize,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if time_left == 0 {
        return release_pressure;
    }

    let key = build_key(time_left, from, release_pressure, opened);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut max = release_pressure;

    let rate = rates.get(from).unwrap();
    if time_left >= 2 && *rate > 0 && !opened.contains(from) {
        // Open value
        let time_left = time_left - 1;
        let mut opened = opened.clone();
        opened.insert(from.to_string());
        for to in tunnels.get(from).unwrap() {
            let new_pressure = max_pressure(
                rates,
                tunnels,
                &opened,
                time_left - 1,
                to,
                release_pressure + rate * time_left,
                cache,
            );
            if new_pressure > max {
                max = new_pressure;
            }
        }
    }
    // Doesn't open value
    if time_left >= 1 {
        for to in tunnels.get(from).unwrap() {
            let new_pressure = max_pressure(
                rates,
                tunnels,
                &opened,
                time_left - 1,
                to,
                release_pressure,
                cache,
            );
            if new_pressure > max {
                max = new_pressure;
            }
        }
    }

    cache.insert(key, max);

    max
}


fn main() {
    let mut rates: Rates = HashMap::new();
    let mut tunnels: Tunnels = HashMap::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        parse_line(&mut rates, &mut tunnels, &line);
    }

    let mut cache = HashMap::new();
    println!("{}", max_pressure(&rates, &tunnels, &HashSet::new(), 30, "AA", 0, &mut cache));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rate() {
        assert_eq!(parse_rate("rate=100;"), 100);
    }

    #[test]
    fn test_parse_line() {
        let mut rates: Rates = HashMap::new();
        let mut tunnels: Tunnels = HashMap::new();
        parse_line(&mut rates, &mut tunnels,
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
        );
        assert_eq!(rates.get("AA"), Some(&0));
        assert_eq!(tunnels.get("AA"), Some(&vec!["DD".to_string(), "II".to_string(), "BB".to_string()]));
    }
}
