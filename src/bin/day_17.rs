use std::collections::HashMap;

fn main() {
    let jets = parse_input();
    part_1(jets.clone());
    println!("Part 2: {}", part_2(jets));
}

fn parse_input() -> Vec<char> {
    aoc_2022::read_char_input("src/bin/day_17.txt")
}

fn part_1(jets: Vec<char>) {
    let mut chamber = Chamber::new(jets);
    chamber.draw();
    for _ in 0..2022 {
        chamber.drop_one();
    }
    println!("Part 1: {}", chamber.height())
}

fn part_2(jets: Vec<char>) -> usize {
    let mut chamber = Chamber::new(jets);
    let mut cycle_finder = HashMap::new();
    cycle_finder.insert(
        (chamber.piece_num, chamber.jet_num, 0usize),
        (0usize, 0usize),
    );
    let mut drops = 0;
    loop {
        chamber.drop_one();
        drops += 1;
        let height = chamber.height();
        if height < 4 {
            continue;
        }

        let shape = ((chamber.rocks[height - 1] as usize) << 24)
            | ((chamber.rocks[height - 2] as usize) << 16)
            | ((chamber.rocks[height - 3] as usize) << 8)
            | (chamber.rocks[height - 4] as usize);
        if let Some(entry) = cycle_finder.get(&(chamber.piece_num, chamber.jet_num, shape)) {
            let delta_height = height - entry.0;
            let delta_drops = drops - entry.1;
            let remaining_drops = Chamber::PASS - entry.1;
            let needed_drops = remaining_drops / delta_drops;
            let left_over_drops = remaining_drops % delta_drops;
            let integral_height = entry.0 + delta_height * needed_drops;
            for _ in 0..left_over_drops {
                chamber.drop_one();
            }
            let left_over_height = chamber.height() - height;
            return integral_height + left_over_height;
        } else {
            cycle_finder.insert((chamber.piece_num, chamber.jet_num, shape), (height, drops));
        }
    }
}

#[derive(Default)]
struct Chamber {
    jets: Vec<char>,
    rocks: Vec<u8>,
    piece_num: usize,
    jet_num: usize,
}

impl Chamber {
    const PIECES: [[u8; 4]; 5] = [
        [0b0000000, 0b0000000, 0b0000000, 0b0011110],
        [0b0000000, 0b0001000, 0b0011100, 0b0001000],
        [0b0000000, 0b0000100, 0b0000100, 0b0011100],
        [0b0010000, 0b0010000, 0b0010000, 0b0010000],
        [0b0000000, 0b0000000, 0b0011000, 0b0011000],
    ];

    const PASS: usize = 1000000000000;

    fn new(jets: Vec<char>) -> Self {
        Self {
            jets,
            rocks: vec![0, 0, 0, 0, 0, 0, 0],
            piece_num: 0,
            jet_num: 0,
        }
    }

    fn print_row(row: u8) {
        let mut bit = 0x40;
        while bit > 0 {
            print!("{}", if (bit & row) != 0 { '#' } else { '.' });
            bit >>= 1;
        }
    }

    fn draw(&self) {
        let mut top = self.rocks.len() - 1;
        while top > 0 {
            top -= 1;
            print!("|");
            Self::print_row(self.rocks[top]);
            println!("|");
        }
        println!("+-------+");
    }

    fn drop_one(&mut self) {
        let mut piece = Self::PIECES[self.piece_num];
        self.piece_num = (self.piece_num + 1) % Self::PIECES.len();

        let mut last = self.rocks.len() - 7;
        while self.rocks[last] != 0 {
            self.rocks.push(0);
            last += 1;
        }

        let mut bottom = self.rocks.len() - 4;

        loop {
            let jet = self.jets[self.jet_num];
            self.jet_num = (self.jet_num + 1) % self.jets.len();

            match jet {
                '<' => {
                    if self.can_go_left(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p <<= 1;
                        }
                    }
                }
                '>' => {
                    if self.can_go_right(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p >>= 1;
                        }
                    }
                }
                _ => panic!("bad jet {jet}"),
            }

            if bottom > 0 && self.can_go_to(bottom - 1, &piece) {
                bottom -= 1;
            } else {
                break;
            }
        }

        let mut p_row = 4;
        while p_row > 0 {
            p_row -= 1;
            self.rocks[bottom] |= piece[p_row];
            bottom += 1;
        }
    }

    fn height(&self) -> usize {
        let mut top = self.rocks.len();
        while top > 0 && self.rocks[top - 1] == 0 {
            top -= 1;
        }
        top
    }

    fn can_go_left(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut p_row = 4;
        while p_row > 0 {
            p_row -= 1;
            if (piece[p_row] & 0x40) != 0 || (self.rocks[bottom] & (piece[p_row] << 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_right(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut p_row = 4;
        while p_row > 0 {
            p_row -= 1;
            if (piece[p_row] & 0x01) != 0 || (self.rocks[bottom] & (piece[p_row] >> 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_to(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut p_row = 4;
        while p_row > 0 {
            p_row -= 1;
            if (self.rocks[bottom] & piece[p_row]) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }
}
