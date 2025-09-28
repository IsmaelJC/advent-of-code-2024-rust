use advent_of_code_2024_rust::{
    get_pair_from_whitespace_separated_elements, read_input, split_lines,
};
use std::iter;

fn main() {
    let input = read_input(1);
    let (mut fst, mut snd): (Vec<_>, Vec<_>) = split_lines(&input)
        .iter()
        .filter_map(|s| get_pair_from_whitespace_separated_elements::<u32>(s))
        .unzip();

    fst.sort();
    snd.sort();

    let total_difference: u32 = iter::zip(fst, snd)
        .map(|pair| pair.0.abs_diff(pair.1))
        .sum();

    println!(
        "The total difference between both lists is: {}",
        total_difference
    );
}
