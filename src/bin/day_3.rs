use std::collections::HashSet;

fn main() {
    let file_contents = aoc_2022::read_string_input("inputs/day_3.txt");
    part_1(&file_contents);
    part_2(&file_contents);
}

fn get_priority(c: char) -> i32 {
    let v = c as u8;
    let mut p = 0;
    p += match c {
        'a'..='z' => v - b'a' + 1,
        'A'..='Z' => v - b'A' + 27,
        _ => panic!("Bad input"),
    } as i32;
    p
}

fn part_1(input: &[String]) {
    let mut priority = 0;
    for line in input.iter() {
        let (left, right) = line.split_at(line.len() / 2);
        let left: HashSet<char> = HashSet::from_iter(left.chars());
        let right: HashSet<char> = HashSet::from_iter(right.chars());
        let shared: HashSet<&char> = left.intersection(&right).collect();
        let ch = shared.iter().next().unwrap();
        priority += get_priority(**ch);
    }
    println!("{:#?}", priority);
}

fn part_2(input: &[String]) {
    let mut priority = 0;
    for group in input.chunks(3) {
        let mut sets = group
            .iter()
            .map(|g| HashSet::from_iter(g.chars()))
            .collect::<Vec<HashSet<char>>>();

        let mut shared = sets.pop().unwrap();
        for set in sets {
            shared = set.intersection(&shared).copied().collect();
        }
        let ch = *shared.iter().next().unwrap();
        priority += get_priority(ch);
    }
    print!("{:#?}", priority);
}
