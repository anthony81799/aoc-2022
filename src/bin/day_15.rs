use std::collections::HashSet;

fn main() {
    let sensors = parse_input();
    part_1(&sensors);
    part_2(&sensors)
}

fn parse_input() -> Vec<Sensor> {
    let mut sensors = Vec::new();
    let input = aoc_2022::read_string_input("src/inputs/day_15.txt");
    for line in input.iter() {
        sensors.push(Sensor::parse(line));
    }
    sensors
}

fn part_1(sensors: &[Sensor]) {
    let mut coverage = HashSet::new();
    for s in sensors {
        let radius = s.radius();
        let distance = (s.location.1 - 2000000).abs();
        if distance > radius {
            continue;
        }
        let rem = radius - distance;
        let left = s.location.0 - rem;
        let right = s.location.0 + rem;

        for x in left..=right {
            coverage.insert(x);
        }
    }
    let beacons: HashSet<i64> = HashSet::from_iter(
        sensors
            .iter()
            .filter(|s| s.beacon.1 == 2000000)
            .map(|s| s.beacon.0),
    );
    println!("Part 1: {}", coverage.len() - beacons.len())
}
fn part_2(sensors: &[Sensor]) {
    let mut row_data = vec![vec![0..=4000000]; 4000001];
    for s in sensors {
        let radius = s.radius();
        let top = 0.max(s.location.1 - radius);
        let bottom = 4000000.min(s.location.1 + radius);
        for row in top..=bottom {
            let distance = (s.location.1 - row).abs();
            let min_x = 0.max(s.location.0 - (radius - distance));
            let max_x = 4000000.min(s.location.0 + (radius - distance));
            let mut new_range = Vec::new();
            for r in &row_data[row as usize] {
                let start = r.start();
                if *start > max_x {
                    new_range.push(r.clone());
                    continue;
                }
                let end = r.end();
                if *end < min_x {
                    new_range.push(r.clone());
                    continue;
                }
                if *start < min_x {
                    new_range.push(*start..=min_x -1);
                }
                if *end > max_x {
                    new_range.push(max_x + 1..=*end);
                }
            }
            row_data[row as usize] = new_range;
        }
    }

    for (y, r) in row_data.iter().enumerate() {
        if !r.is_empty() {
            let x = *r[0].start();
            println!("Part 2: {}", x * 4000000 + y as i64);
            break;
        }
    }
}

#[derive(Debug, Clone)]
struct Sensor {
    location: (i64, i64),
    beacon: (i64, i64),
}

impl Sensor {
    fn parse(s: &str) -> Self {
        let (left, beacon) = s.split_once(": closest beacon is at x=").unwrap();
        let (_, sensor) = left.split_once("Sensor at x=").unwrap();

        Self {
            location: Self::coord(sensor),
            beacon: Self::coord(beacon),
        }
    }

    fn coord(s: &str) -> (i64, i64) {
        let (x, y) = s.split_once(", y=").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }

    fn radius(&self) -> i64 {
        (self.beacon.0 - self.location.0).abs() + (self.beacon.1 - self.location.1).abs()
    }
}
