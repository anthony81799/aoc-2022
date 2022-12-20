use std::{
    fmt::Debug,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub fn read_number_input<T: AsRef<Path>, U: FromStr>(path: T) -> Vec<Vec<U>>
where
    <U as FromStr>::Err: Debug,
{
    fs::read_to_string(path)
        .expect("Unable ro open file {path}")
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split('\n')
                .map(|n| n.parse::<U>().expect("Unable to Parse"))
                .collect::<Vec<U>>()
        })
        .collect()
}

pub fn read_string_input(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("Unable ro open file {path}");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn read_input_buffer(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn read_char_input(path: impl AsRef<Path>) -> Vec<char> {
    fs::read_to_string(path)
        .expect("Unable ro open file {path}")
        .chars()
        .collect()
}
