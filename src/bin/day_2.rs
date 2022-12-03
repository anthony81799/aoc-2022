fn main() {
    let file_contents = aoc_2022::read_string_input("inputs/day_2.txt");
    let mut games = Vec::new();
    for l in file_contents.iter() {
        games.push(create_game_state(l))
    }
    part_1(&games);
    part_2(&games);
}

fn part_1(input: &[(char, char)]) {
    let mut score = 0;
    for l in input.iter() {
        match *l {
            ('A', 'X') => score += 4,
            ('A', 'Y') => score += 8,
            ('A', 'Z') => score += 3,
            ('B', 'X') => score += 1,
            ('B', 'Y') => score += 5,
            ('B', 'Z') => score += 9,
            ('C', 'X') => score += 7,
            ('C', 'Y') => score += 2,
            ('C', 'Z') => score += 6,
            _ => score += 0,
        }
    }
    println!("Part 1: {:#?}", score);
}

fn part_2(input: &[(char, char)]) {
    let mut score = 0;
    for l in input.iter() {
        match l {
            ('A', 'X') => score += 3,
            ('A', 'Y') => score += 4,
            ('A', 'Z') => score += 8,
            ('B', 'X') => score += 1,
            ('B', 'Y') => score += 5,
            ('B', 'Z') => score += 9,
            ('C', 'X') => score += 2,
            ('C', 'Y') => score += 6,
            ('C', 'Z') => score += 7,
            _ => score += 0,
        }
    }
    println!("Part 2: {:#?}", score);
}

fn create_game_state(s: &str) -> (char, char) {
    let (o, m) = s.split_once(' ').unwrap();
    (
        o.chars().take(1).next().unwrap(),
        m.chars().take(1).next().unwrap(),
    )
}
