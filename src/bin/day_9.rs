use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(direction: &str) -> Self {
        match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction{direction}"),
        }
    }
}

#[derive(Default)]
struct Snake {
    segments: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Snake {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn new(size: usize) -> Self {
        Self {
            segments: vec![(0, 0); size],
            visited: HashSet::new(),
        }
    }

    fn make_move(&mut self, direction: &Direction) {
        let delta = Self::DIRECTIONS[*direction as usize];

        self.segments[0].0 += delta.0;
        self.segments[0].1 += delta.1;

        for i in 1..self.segments.len() {
            let row_diff = self.segments[i - 1].0 - self.segments[i].0;
            let col_diff = self.segments[i - 1].1 - self.segments[i].1;

            if col_diff == 0 && row_diff.abs() > 1 {
                self.segments[i].0 += row_diff.signum();
            } else if row_diff == 0 && col_diff > 1 {
                self.segments[i].1 += col_diff.signum();
            } else if row_diff.abs() > 1 || col_diff.abs() > 1 {
                self.segments[i].0 += row_diff.signum();
                self.segments[i].1 += col_diff.signum();
            }
        }

        self.visited.insert(self.segments[self.segments.len() - 1]);
    }
}

fn main() {
    let input = aoc_2022::read_string_input("src/inputs/day_9.txt");
    let steps = parse_input(input);
    solve(&steps, 2);
    solve(&steps, 10);
}

fn parse_input(input: Vec<String>) -> Vec<(Direction, i32)> {
    let mut steps = Vec::new();
    for line in input.iter() {
        let (direction, distance) = line.split_once(' ').unwrap();
        let direction = Direction::parse(direction);
        let distance = distance.parse().unwrap();
        steps.push((direction, distance));
    }
    steps
}

fn solve(steps: &Vec<(Direction, i32)>, size: usize) {
    let mut snake = Snake::new(size);
    for (direction, distance) in steps {
        for _ in 0..*distance {
            snake.make_move(direction);
        }
    }
    println!("{}", snake.visited.len());
}
