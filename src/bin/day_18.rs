use std::collections::{HashSet, VecDeque};

fn main() {
    let cubes = parse_input();
    part_1(&cubes);
    part_2(&cubes);
}

const DELTA: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn parse_input() -> Vec<(i32, i32, i32)> {
    let input = aoc_2022::read_string_input("src/bin/day_18.txt");
    let mut cubes = Vec::new();
    for line in input {
        let (x, rest) = line.split_once(',').unwrap();
        let (y, z) = rest.split_once(',').unwrap();
        cubes.push((x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()));
    }
    cubes
}

fn part_1(cubes: &Vec<(i32, i32, i32)>) {
    let mut sides = cubes.len() * 6;
    for c in cubes {
        for d in DELTA {
            let pos = (c.0 + d.0, c.1 + d.1, c.2 + d.2);
            if cubes.contains(&pos) {
                sides -= 1;
            }
        }
    }
    println!("Part 1: {sides}");
}

fn part_2(cubes: &Vec<(i32, i32, i32)>) {
    let mut x_range = (i32::MAX, i32::MIN);
    let mut y_range = (i32::MAX, i32::MIN);
    let mut z_range = (i32::MAX, i32::MIN);
    for c in cubes {
        x_range.0 = x_range.0.min(c.0);
        x_range.1 = x_range.1.max(c.0);
        y_range.0 = y_range.0.min(c.1);
        y_range.1 = y_range.1.max(c.1);
        z_range.0 = z_range.0.min(c.2);
        z_range.1 = z_range.1.max(c.2);
    }

    x_range = (x_range.0 - 1, x_range.1 + 1);
    y_range = (y_range.0 - 1, y_range.1 + 1);
    z_range = (z_range.0 - 1, z_range.1 + 1);

    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((x_range.0, y_range.0, z_range.0));

    let mut count = 0;
    while let Some(pos) = to_visit.pop_front() {
        if !seen.insert(pos) {
            continue;
        }

        for d in DELTA {
            let next_pos = (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2);
            if next_pos.0 < x_range.0
                || next_pos.0 > x_range.1
                || next_pos.1 < y_range.0
                || next_pos.1 > y_range.1
                || next_pos.2 < z_range.0
                || next_pos.2 > z_range.1
            {
                continue;
            }
            if cubes.contains(&next_pos) {
                count += 1;
            } else {
                to_visit.push_back(next_pos);
            }
        }
    }
    println!("Part 2: {count}");
}
