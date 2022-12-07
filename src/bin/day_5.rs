fn main() {
    let mut stacks = vec![
        "WMLF".to_owned(),
        "BZVMF".to_owned(),
        "HVRSLQ".to_owned(),
        "FSVQPMTJ".to_owned(),
        "LSW".to_owned(),
        "FVPMRJW".to_owned(),
        "JQCPNRF".to_owned(),
        "VHPSZWRB".to_owned(),
        "BMJCGHZW".to_owned(),
    ];

    let file_contents = aoc_2022::read_string_input("src/inputs/day_5.txt");
    for line in file_contents.iter() {
        let task: Vec<usize> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        move_crate(task, &mut stacks, true); // To solve part 1 use false
    }
    let top: String = stacks.iter().map(|s| s.chars().last().unwrap()).collect();
    print!("{top}")
}

fn move_crate(task: Vec<usize>, stacks: &mut [String], keep_still: bool) {
    let mut crates: String = (0..task[0])
        .map(|_| stacks[task[1] - 1].pop().unwrap())
        .collect();

    if keep_still {
        crates = crates.chars().rev().collect();
    }

    stacks[task[2] - 1].push_str(&crates);
}
