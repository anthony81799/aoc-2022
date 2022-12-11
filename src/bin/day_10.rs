fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<String> = aoc_2022::read_string_input("src/inputs/day_10.txt");

    let mut x = 1;
    let mut cycles = 0;
    let mut sum = 0;

    for line in &input {
        match line.split_at(4) {
            ("noop", _) => {
                do_step(x, &mut cycles, &mut sum);
            }
            ("addx", arg) => {
                do_step(x, &mut cycles, &mut sum);
                do_step(x, &mut cycles, &mut sum);
                x += arg.trim().parse::<i64>().unwrap();
            }
            (_, _) => unreachable!(),
        }
    }
    println!("{}", sum);

    Ok(())
}

fn do_step(x: i64, cycles: &mut i64, sum: &mut i64) {
    let beam = *cycles % 40;
    if beam >= x - 1 && beam <= x + 1 {
        print!("#");
    } else {
        print!(".");
    }
    if beam == 39 {
        println!();
    }
    *cycles += 1;
    if (*cycles + 20) % 40 == 0 {
        *sum += x * *cycles;
    }
}
