use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
struct Cave {
    tile: HashMap<(i32, i32), char>,
    bottom: i32,
    has_floor: bool,
}

impl Cave {
    fn convert(s: Option<&str>) -> Option<(i32, i32)> {
        if let Some(s) = s {
            let (x, y) = s.split_once(',').unwrap();
            Some((x.parse().unwrap(), y.parse().unwrap()))
        } else {
            None
        }
    }

    fn draw_line(&mut self, start: (i32, i32), end: (i32, i32)) {
        let dx = (end.0 - start.0).signum();
        let dy = (end.1 - start.1).signum();

        self.bottom = self.bottom.max(start.1.max(end.1));

        let mut point = start;
        self.tile.insert(point, '#');
        while point != end {
            point.0 += dx;
            point.1 += dy;
            self.tile.insert(point, '#');
        }
    }

    fn drop_one(&mut self) -> bool {
        let mut sand = (500, 0);
        if self.has_floor && self.tile.contains_key(&sand) {
            return false;
        }
        while let Some(next_pos) = self.fall(sand) {
            if !self.has_floor && next_pos.1 > self.bottom {
                return false;
            }
            sand = next_pos;
            if self.has_floor && sand.1 == self.bottom + 1 {
                break;
            }
        }
        self.tile.insert(sand, 'o');
        true
    }

    fn fall(&self, pos: (i32, i32)) -> Option<(i32, i32)> {
        for dx in [0, -1, 1] {
            let new_pos = (pos.0 + dx, pos.1 + 1);
            if !self.tile.contains_key(&new_pos) {
                return Some(new_pos);
            }
        }
        None
    }
}

fn main() {
    let input = aoc_2022::read_string_input("src/inputs/day_14.txt");
    let mut cave = Cave::default();
    parse_input(input, &mut cave);
    part_1(&mut cave.clone());
    part_2(&mut cave.clone());
}

fn parse_input(input: Vec<String>, cave: &mut Cave) {
    for line in input.iter() {
        let mut iter = line.split(" -> ");
        let mut start = Cave::convert(iter.next()).unwrap();
        while let Some(end) = Cave::convert(iter.next()) {
            cave.draw_line(start, end);
            start = end;
        }
    }
}

fn part_1(cave: &mut Cave) {
    let mut count = 0;
    while cave.drop_one() {
        count += 1;
    }
    println!("Part 1: {count}");
}
fn part_2(cave: &mut Cave) {
    let mut count = 0;
    cave.has_floor = true;
    while cave.drop_one() {
        count += 1;
    }
    println!("Part 2: {count}");
}
