use std::{cmp::Ordering, str::Chars};

fn main() {
    let binding = aoc_2022::read_string_input("src/inputs/day_13.txt");
    let input = binding
        .iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&String>>();
    let mut pairs = parse_input(input);
    part_1(&pairs);
    part_2(&mut pairs);
}

fn parse_input(input: Vec<&String>) -> Vec<(Val, Val)> {
    let mut pairs = Vec::new();
    for pair in input.chunks(2) {
        pairs.push((Val::parse(pair[0]), Val::parse(pair[1])));
    }
    pairs
}

fn part_1(pairs: &[(Val, Val)]) {
    let mut sum = 0;
    for (index, pair) in pairs.iter().enumerate() {
        if pair.0 < pair.1 {
            sum += index + 1;
        }
    }
    println!("Part 1: {sum}");
}

fn part_2(pairs: &mut [(Val, Val)]) {
    let mut list = Vec::new();
    for pair in pairs.iter() {
        list.push(pair.0.clone());
        list.push(pair.1.clone());
    }
    let d2 = Val::parse("[[2]]");
    let d6 = Val::parse("[[6]]");
    list.push(d2.clone());
    list.push(d6.clone());
    list.sort();

    let mut answer = 1;
    for (index, val) in list.iter().enumerate() {
        if *val == d2 || *val == d6 {
            answer *= index + 1;
        }
    }
    println!("Part 2: {answer}");
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Val {
    Num(i32),
    List(Vec<Val>),
}

impl Val {
    fn parse(s: &str) -> Self {
        let mut c = s.chars();
        if c.next().unwrap() != '[' {
            panic!("bad input")
        }
        Self::parse_into(&mut c)
    }

    fn parse_into(c: &mut Chars) -> Self {
        let mut result = Vec::new();
        let mut num = -1;
        while let Some(ch) = c.next() {
            match ch {
                '[' => result.push(Self::parse_into(c)),
                ',' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                    };
                    num = -1;
                }
                ']' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                    }
                    return Self::List(result);
                }
                '0'..='9' => {
                    if num == -1 {
                        num = (ch as u8 - b'0') as i32;
                    } else {
                        num = (num * 10) + (ch as u8 - b'0') as i32;
                    }
                }
                _ => panic!("bad char {ch}"),
            };
        }
        Self::List(result)
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(left), Self::List(right)) => {
                let mut idx = 0;
                while idx < left.len() && idx < right.len() {
                    match (&left[idx], &right[idx]) {
                        (Self::Num(l), Self::Num(r)) => {
                            if l != r {
                                return l.cmp(r);
                            }
                        }
                        (Self::List(_), Val::Num(r)) => {
                            let check = left[idx].compare(&Self::List(vec![Self::Num(*r)]));
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Self::Num(l), Val::List(_)) => {
                            let check = Self::List(vec![Self::Num(*l)]).compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Self::List(_), Val::List(_)) => {
                            let check = left[idx].compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                    }
                    idx += 1;
                }
                left.len().cmp(&right.len())
            }
            _ => panic!("bad input"),
        }
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}
