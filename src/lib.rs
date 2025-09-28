use std::collections::HashMap;
use std::hash::Hash;
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

pub fn get_frequencies<T>(iter: impl IntoIterator<Item = T>) -> HashMap<T, usize>
where
    T: Eq + Hash,
{
    iter.into_iter().fold(HashMap::new(), |mut map, item| {
        map.entry(item)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        map
    })
}
