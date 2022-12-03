fn main() {
    let elves: Vec<Vec<i32>> = aoc_2022::read_number_input("inputs/day_1.txt");
    part_1(&elves);
    part_2(&elves);
}

fn part_1(elves: &Vec<Vec<i32>>) {
    let cals = sort_elves(elves);
    println!("Part 1: {:#?}", cals.iter().take(1).sum::<i32>())
}

fn part_2(elves: &Vec<Vec<i32>>) {
    let cals = sort_elves(elves);
    println!("Part 2: {:#?}", cals.iter().take(3).sum::<i32>())
}

fn sort_elves(elves: &[Vec<i32>]) -> Vec<i32> {
    let mut v = Vec::new();
    for e in elves.iter() {
        let n: i32 = e.iter().sum();
        v.push(n);
    }
    v.sort();
    v.reverse();
    v
}
