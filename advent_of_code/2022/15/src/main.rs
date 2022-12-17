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

// fn pos_sacnned(sensor: &Pos, closest_beacon: &Pos, pos: &Pos) -> bool {
//     let (x1, y1) = sensor;
//     let (x2, y2) = closest_beacon;
//     (x1 - x2).abs() + (y1 - y2).abs() >= (x1 - pos.0).abs() + (y1 - pos.1).abs()
// }

fn pos_scanned_at_row(sensor: &Pos, closest_beacon: &Pos, row: isize) -> HashSet<Pos> {
    let (x1, y1) = sensor;
    let (x2, y2) = closest_beacon;
    let x_diff = (x1 - x2).abs() + (y1 - y2).abs() - (y1 - row).abs();
    let mut scanned: HashSet<Pos> = HashSet::new();
    for x in x1 - x_diff..=x1 + x_diff {
        scanned.insert((x, row));
    }
    scanned
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

    let mut scanned: HashSet<Pos> = HashSet::new();
    for (sensor, beacon) in sensors.iter() {
        for pos in pos_scanned_at_row(sensor, beacon, 2000000) {
            if !beacons.contains(&pos) && !sensors.contains_key(&pos) {
                scanned.insert(pos);
            }
        }
    }
    println!("{}", scanned.len());
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
}
