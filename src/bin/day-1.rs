use std::fs;

fn main() {
    let file_contents = fs::read_to_string("../inputs/day-1.txt").expect("Input file not found");
    let lines: Vec<&str> = file_contents.split("\n\n").collect();
    let mut elves = Vec::new();

    for l in lines.iter() {
        let l: Vec<&str> = l.split('\n').collect();
        let mut cals = Vec::new();
        for n in l.iter() {
            let n = n.parse::<i32>().unwrap();
            cals.push(n);
        }
        elves.push(cals);
    }

    let mut nums = Vec::new();
    for e in elves.iter() {
        let n: i32 = e.iter().sum();
        nums.push(n);
    }
    nums.sort();
    nums.reverse();
    let top_three: i32 = nums.iter().take(3).sum();
    print!("{:#?}", top_three)
}
