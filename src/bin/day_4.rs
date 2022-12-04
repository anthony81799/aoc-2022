fn main() {
    let vec_map = |t: Vec<&str>| t.iter().map(|s| s.parse::<u8>().unwrap()).collect();
    let input: Vec<Vec<u8>> = aoc_2022::read_string_input("inputs/day_4.txt")
        .iter()
        .map(|l| l.split(','))
        .map(|mut s| (s.next().unwrap().split('-'), s.next().unwrap().split('-')))
        .map(|(e1, e2)| vec_map(e1.chain(e2).collect()))
        .collect();

    let p1 = input
        .iter()
        .filter(|nums| {
            (nums[0] >= nums[2] && nums[1] <= nums[3]) || (nums[2] >= nums[0] && nums[3] <= nums[1])
        })
        .count();
    let p2 = input
        .iter()
        .filter(|nums| nums[0] <= nums[3] && nums[2] <= nums[1])
        .count();
    println!("{p1} {p2}");
}
