use std::io;
use std::collections::HashSet;

type Rock = (usize, usize);
type Sand = (usize, usize);
type Pos = (usize, usize);

fn pour_sand(blocks: &HashSet<Pos>, max_y: usize) -> Sand {
    let mut x = 500;
    let mut y = 0;
    loop {
        // limit exceeded
        if y > max_y {
            break;
        }

        // down one step
        y += 1;
        if !blocks.contains(&(x, y)) {
            continue;
        } else {
            y -= 1;
        }

        // one step down and to the left
        y += 1;
        x -= 1;
        if !blocks.contains(&(x, y)) {
            continue;
        } else {
            y -= 1;
            x += 1;
        }

        // one step down and to the right
        y += 1;
        x += 1;
        if !blocks.contains(&(x, y)) {
            continue;
        } else {
            y -= 1;
            x -= 1;
        }
        break;
    }
    (x, y)
}

fn parse_edge(edge: &str) -> (usize, usize) {
    let mut edge = edge
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap());
    let x = edge.next().unwrap();
    let y = edge.next().unwrap();
    (x, y)
}

fn parse_rocks(path: &str) -> HashSet<Rock> {
    let mut rocks: HashSet<Rock> = HashSet::new();
    let mut edges = path.trim().split("->").peekable();
    while let Some(cur_edge) = edges.next() {
        if let Some(next_edge) = edges.peek() {
            let (cur_x, cur_y) = parse_edge(cur_edge);
            let (next_x, next_y) = parse_edge(next_edge);

            if cur_x == next_x {
                let (min_y, max_y) = if cur_y < next_y {
                    (cur_y, next_y)
                } else {
                    (next_y, cur_y)
                };
                for y in min_y..=max_y {
                    rocks.insert((cur_x, y));
                }
            } else if cur_y == next_y {
                let (min_x, max_x) = if cur_x < next_x {
                    (cur_x, next_x)
                } else {
                    (next_x, cur_x)
                };
                for x in min_x..=max_x {
                    rocks.insert((x, cur_y));
                }
            }
        }
    }
    rocks
}

fn main() {
    let mut rocks: HashSet<Rock> = HashSet::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        rocks.extend(parse_rocks(&line));
    }

    let mut max_y = 0;
    for rock in rocks.iter() {
        if rock.1 > max_y {
            max_y = rock.1;
        }
    }

    let mut blocks: HashSet<Rock> = HashSet::new();
    blocks.extend(&rocks);
    let mut sands: HashSet<Sand> = HashSet::new();
    loop {
        let (x, y) = pour_sand(&blocks, max_y);
        if y > max_y {
            break;
        }
        sands.insert((x, y));
        blocks.extend(&sands);
    }

    println!("{}", sands.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_edge() {
        let edge = "498,4";
        let (x, y) = parse_edge(edge);
        assert_eq!(x, 498);
        assert_eq!(y, 4);
    }

    #[test]
    fn test_parse_rocks() {
        let path = "498,4 -> 498,6 -> 496,6";
        let rocks = parse_rocks(path);
        assert_eq!(rocks.len(), 5);
        assert!(rocks.contains(&(498, 4)));
        assert!(rocks.contains(&(498, 5)));
        assert!(rocks.contains(&(498, 6)));
        assert!(rocks.contains(&(497, 6)));
        assert!(rocks.contains(&(496, 6)));
    }
}
