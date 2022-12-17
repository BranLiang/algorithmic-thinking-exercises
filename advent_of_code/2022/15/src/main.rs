use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

type Pos = (isize, isize);

// Parse: Sensor at x=2, y=18
fn parse_sensor(input: &str) -> Pos {
    let mut parts = input.split_whitespace();
    parts.next();
    parts.next();
    let x = parts.next().unwrap();
    let x = x.strip_prefix("x=").unwrap();
    let x = x.strip_suffix(',').unwrap().parse().unwrap();

    let y = parts.next().unwrap();
    let y = y.strip_prefix("y=").unwrap();
    let y = y.parse().unwrap();

    (x, y)
}

fn parse_beacon(input: &str) -> Pos {
    let mut parts = input.split_whitespace();
    parts.next();
    parts.next();
    parts.next();
    parts.next();
    let x = parts.next().unwrap();
    let x = x.strip_prefix("x=").unwrap();
    let x = x.strip_suffix(',').unwrap().parse().unwrap();

    let y = parts.next().unwrap();
    let y = y.strip_prefix("y=").unwrap();
    let y = y.parse().unwrap();

    (x, y)
}

fn scanned_range(sensor: &Pos, beacon: &Pos, y: isize) -> Option<(isize, isize)> {
    let (x1, y1) = sensor;
    let (x2, y2) = beacon;

    let x_diff = (x2 - x1).abs() + (y1 - y2).abs() - (y1 - y).abs();
    if x_diff >= 0 {
        (x1 - x_diff, x1 + x_diff).into()
    } else {
        None
    }
}

fn merge_ranges(ranges: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let mut merged = Vec::new();
    let mut ranges = ranges.to_vec();
    ranges.sort_by_key(|(x1, _)| *x1);

    let mut current = ranges.remove(0);
    for (x1, x2) in ranges {
        if x1 <= current.1 + 1 {
            current.1 = current.1.max(x2);
        } else {
            merged.push(current);
            current = (x1, x2);
        }
    }
    merged.push(current);

    merged
}

fn main() {
    let mut sensors: HashMap<Pos, Pos> = HashMap::new();
    let mut beacons: HashSet<Pos> = HashSet::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (sensor_input, beacon_input) = line.split_once(':').unwrap();

        let sensor = parse_sensor(sensor_input);
        let beacon = parse_beacon(beacon_input);

        beacons.insert(beacon);
        sensors.insert(sensor, beacon);
    }

    for y in 0..=4000000 {
        let mut ranges = Vec::new();
        for (sensor, beacon) in sensors.iter() {
            if let Some(range) = scanned_range(sensor, beacon, y) {
                ranges.push(range);
            }
        }

        let ranges = merge_ranges(&ranges);
        if ranges.len() >= 2 {
            println!("{}", (ranges[0].1 + 1) * 4000000 + y);
            break;
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_sensor() {
        let pos = parse_sensor("Sensor at x=2, y=18");
        assert_eq!(pos.0, 2);
        assert_eq!(pos.1, 18);
    }

    #[test]
    fn test_parse_beacon() {
        let pos = parse_beacon(" closest beacon is at x=-2, y=15");
        assert_eq!(pos.0, -2);
        assert_eq!(pos.1, 15);
    }

    #[test]
    fn test_merge_ranges() {
        let ranges = merge_ranges(&[(-2, 2), (1, 4), (5, 6)]);
        assert_eq!(ranges, vec![(-2, 6)]);

        let ranges = merge_ranges(&[(-2, 2), (1, 4), (-1, 6)]);
        assert_eq!(ranges, vec![(-2, 6)]);

        let ranges = merge_ranges(&[(-2, 2), (3, 4), (6, 7)]);
        assert_eq!(ranges, vec![(-2, 4), (6, 7)]);
    }
}
