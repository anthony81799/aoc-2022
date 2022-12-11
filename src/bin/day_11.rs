use std::collections::VecDeque;

#[derive(Debug, Default, Clone)]
enum Op {
    #[default]
    AddSelf,
    MultiplySelf,
    Add(i64),
    Multiply(i64),
}

impl Op {
    fn calculate(&self, value: i64) -> i64 {
        match self {
            Op::Add(n) => value + n,
            Op::AddSelf => value + value,
            Op::Multiply(n) => value * n,
            Op::MultiplySelf => value * value,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Op,
    test: i64,
    dest: (usize, usize),
    count: usize,
}

fn main() {
    let binding = aoc_2022::read_string_input("src/inputs/day_11.txt");
    let input: Vec<&String> = binding.iter().filter(|s| !s.is_empty()).collect();
    let mut monkies = parse_input(input);
    solve(&mut monkies.clone(), 20);
    solve(&mut monkies, 10000);
}

fn parse_input(input: Vec<&String>) -> Vec<Monkey> {
    let mut monkies: Vec<Monkey> = Vec::new();
    let mut monkey = Monkey::default();
    for line in input {
        let words = line.trim().split(' ').collect::<Vec<&str>>();
        match words[0] {
            "Monkey" => monkey = Monkey::default(),
            "Starting" => {
                let (_, strlist) = line.split_once(": ").unwrap();
                monkey.items = strlist.split(", ").map(|w| w.parse().unwrap()).collect()
            }
            "Operation:" => {
                monkey.operation = if words[4] == "+" {
                    if words[5] == "old" {
                        Op::AddSelf
                    } else {
                        Op::Add(words[5].parse().unwrap())
                    }
                } else {
                    if words[5] == "old" {
                        Op::MultiplySelf
                    } else {
                        Op::Multiply(words[5].parse().unwrap())
                    }
                }
            }
            "Test:" => monkey.test = words[3].parse().unwrap(),
            "If" => {
                if words[1] == "true:" {
                    monkey.dest.0 = words[5].parse().unwrap();
                } else {
                    monkey.dest.1 = words[5].parse().unwrap();
                    monkies.push(monkey);
                    monkey = Monkey::default();
                }
            }
            _ => panic!("unhandled value {}", words[0]),
        }
    }
    monkies
}

fn round(monkies: &mut Vec<Monkey>, managable: bool) {
    for m in 0..monkies.len() {
        while let Some(item) = monkies[m].items.pop_front() {
            let worry = if managable {
                monkies[m].operation.calculate(item) / 3
            } else {
                let modval: i64 = monkies.iter().map(|m| m.test).product();
                monkies[m].operation.calculate(item) % modval
            };
            let destination = if worry % monkies[m].test == 0 {
                monkies[m].dest.0
            } else {
                monkies[m].dest.1
            };
            monkies[destination].items.push_back(worry);
            monkies[m].count += 1;
        }
    }
}

fn solve(monkies: &mut Vec<Monkey>, rounds: i64) {
    let managable = rounds == 20;
    for _ in 0..rounds {
        round(monkies, managable);
    }
    let mut monkey_business = monkies.iter().map(|m| m.count).collect::<Vec<usize>>();
    monkey_business.sort_by(|a, b| b.cmp(a));
    println!(
        "monkey business: {:?}",
        monkey_business[0] * monkey_business[1]
    )
}
