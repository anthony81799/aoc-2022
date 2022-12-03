use std::fs;

fn main() {
    let binding = fs::read_to_string("inputs/day_2.txt")
        .expect("Unable to open file.");
    let file_contents: Vec<&str> = binding
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect();
    part_1(&file_contents);
    part_2(&file_contents);
    
}

fn part_1(input: &Vec<&str>) {
    let mut score = 0;
    for l in input.iter() {
        match *l {
            "A X" => score += 4,
            "A Y" => score += 8,
            "A Z" => score += 3,
            "B X" => score += 1,
            "B Y" => score += 5,
            "B Z" => score += 9,
            "C X" => score += 7,
            "C Y" => score += 2,
            "C Z" => score += 6,
            _ => score += 0
        }
    }
    println!("Part 1: {:#?}", score);
}

fn part_2(input: &Vec<&str>) {
    let mut score = 0;
    for l in input.iter() {
        match *l {
            "A X" => score += 3,
            "A Y" => score += 4,
            "A Z" => score += 8,
            "B X" => score += 1,
            "B Y" => score += 5,
            "B Z" => score += 9,
            "C X" => score += 2,
            "C Y" => score += 6,
            "C Z" => score += 7,
            _ => score += 0
        }
    }
    println!("Part 2: {:#?}", score);
}
