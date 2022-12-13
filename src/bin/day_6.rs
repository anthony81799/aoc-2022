use std::collections::VecDeque;

fn main() {
    let file_contents = aoc_2022::read_input_buffer("src/inputs/day_6.txt");
    solve(&file_contents, 4);
    solve(&file_contents, 14);
}

fn solve(file_contents: &str, bound: usize) {
    let mut last_four = VecDeque::new();
    let mut count = 0;
    for ch in file_contents.chars() {
        last_four.push_back(ch);
        if last_four.len() > bound {
            last_four.pop_front();
        }
        count += 1;
        if start_of_payload(&last_four, bound) {
            println!("{count}");
            return;
        }
    }
}

fn start_of_payload(buff: &VecDeque<char>, bound: usize) -> bool {
    if buff.len() != bound {
        return false;
    }
    for ch in buff.iter() {
        let count = buff.iter().filter(|&c| *c == *ch).count();
        if count > 1 {
            return false;
        }
    }
    true
}
