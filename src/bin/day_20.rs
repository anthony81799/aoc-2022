fn main() {
    let mut numbers = parse_input(811589153); // 811589153 for part 2
    solve(&mut numbers, 10); // 10 for part 2
}

fn parse_input(scale: i64) -> Vec<Val> {
    let input = aoc_2022::read_string_input("src/bin/day_20.txt");
    let mut nums = Vec::new();
    for (index, line) in input.iter().enumerate() {
        nums.push(Val {
            original_index: index,
            value: line.parse::<i64>().unwrap() * scale,
        });
    }
    nums
}

fn solve(numbers: &mut Vec<Val>, cycles: i64) {
    let sum = Val::decrypt(numbers, cycles);
    println!("Answer: {sum}")
}

struct Val {
    original_index: usize,
    value: i64,
}

impl Val {
    fn decrypt(numbers: &mut Vec<Val>, cycles: i64) -> i64 {
        let max_i = numbers.len() as i64 - 1;
        for _ in 0..cycles {
            for current in 0..numbers.len() {
                let index = numbers
                    .iter()
                    .position(|n| n.original_index == current)
                    .unwrap();
                let mut new_index = index as i64 + numbers[index].value;
                new_index = ((new_index % max_i) + max_i) % max_i;
                let number = numbers.remove(index);
                numbers.insert(new_index as usize, number);
            }
        }
        let zero_index = numbers.iter().position(|n| n.value == 0).unwrap();
        let mut sum = 0;
        for n in 1..4 {
            sum += numbers[(zero_index + n * 1000) % numbers.len()].value;
        }
        sum
    }
}
