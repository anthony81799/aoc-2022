use std::collections::HashMap;

fn main() {
    let input = aoc_2022::read_string_input("src/inputs/day_12.txt");
    let info = parse_input(input);
    part_1(&info);
    part_2(&info)
}

type Info = (Vec<Vec<u8>>, (usize, usize), (usize, usize));

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_surrounding_points(
    info: &Info,
    position: (usize, usize),
) -> Vec<(usize, usize)> {
    let ipos = (position.0 as i32, position.1 as i32);
    let width = info.0[0].len() as i32;
    let height = info.0.len() as i32;
    DIRECTIONS
        .iter()
        .map(|d| (ipos.0 + d.0, ipos.1 + d.1))
        .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < height && pos.1 < width)
        .map(|pos| (pos.0 as usize, pos.1 as usize))
        .collect()
}

fn parse_input(input: Vec<String>) -> Info {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row, line) in input.iter().enumerate() {
        let mut gridline = line.chars().map(|c| c as u8).collect::<Vec<u8>>();
        if let Some(start_point) = gridline.iter().position(|&p| p == b'S') {
            start = (row, start_point);
            gridline[start_point] = b'a';
        }
        if let Some(end_point) = gridline.iter().position(|&p| p == b'E') {
            end = (row, end_point);
            gridline[end_point] = b'z';
        }
        grid.push(gridline);
    }
    (grid, start, end)
}

fn part_1(info: &Info) {
    let shortest = find_shortest(info, info.1);
    println!("Part 1: {}", shortest.unwrap())
}

fn part_2(info: &Info) {
    let mut starting_points = Vec::new();
    for (row, line) in info.0.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == b'a' {
                starting_points.push((row, col));
            }
        }
    }
    println!(
        "Part 2: {}",
        starting_points
            .iter()
            .filter_map(|p| find_shortest(info, *p))
            .min()
            .unwrap()
    );
}

fn find_shortest(
    info: &Info,
    start_point: (usize, usize),
) -> Option<usize> {
    let mut to_visit = Vec::new();
    let mut shortest: HashMap<(usize, usize), usize> = HashMap::new();
    shortest.insert(start_point, 0);
    to_visit.extend(get_surrounding_points(info, start_point));
    while let Some(location) = to_visit.pop() {
        let elevation = info.0[location.0][location.1];
        let points = get_surrounding_points(info, location);
        let valid = points
            .iter()
            .filter(|pos| info.0[pos.0][pos.1] + 1 >= elevation)
            .copied()
            .collect::<Vec<(usize, usize)>>();
        let new_distance = valid.iter().filter_map(|pos| shortest.get(pos)).min();
        if new_distance.is_none() {
            continue;
        }
        let new_distance = new_distance.unwrap() + 1;
        let current_distance = shortest.entry(location).or_insert(usize::MAX);
        if *current_distance > new_distance {
            *current_distance = new_distance;
            to_visit.extend(points.iter());
        }
    }
    shortest.get(&info.2).copied()
}
