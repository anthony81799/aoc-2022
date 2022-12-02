use std::{fmt::Debug, fs, path::Path, str::FromStr};

pub fn read_number_input<T: AsRef<Path>, U: FromStr>(path: T) -> Vec<Vec<U>>
where
    <U as FromStr>::Err: Debug,
{
    fs::read_to_string(path)
        .expect("Unable to open file")
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split('\n')
                .map(|n| n.parse::<U>().expect("Unable to Parse"))
                .collect::<Vec<U>>()
        })
        .collect()
}
