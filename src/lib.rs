use std::{fs, str::FromStr};

pub fn read_input(day: u8) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(filename).expect("failed to read input")
}

pub fn split_lines(s: &str) -> Vec<&str> {
    s.lines().collect()
}

pub fn get_pair_from_whitespace_separated_elements<T: FromStr>(s: &str) -> Option<(T, T)> {
    let mut elements = s
        .split_whitespace()
        .filter_map(|s| s.parse::<T>().ok())
        .take(2);

    match (elements.next(), elements.next()) {
        (Some(t1), Some(t2)) => Some((t1, t2)),
        _ => None,
    }
}
