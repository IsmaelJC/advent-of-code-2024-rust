use advent_of_code_2024_rust::{
    get_pair_from_whitespace_separated_elements, read_input, split_lines,
};
use std::iter;

fn get_pair_wise_difference_of_sorted_vecs(mut fst: Vec<u32>, mut snd: Vec<u32>) -> u32 {
    fst.sort();
    snd.sort();

    iter::zip(fst, snd)
        .map(|pair| pair.0.abs_diff(pair.1))
        .sum()
}

fn main() {
    let input = read_input(1);
    let (fst, snd): (Vec<_>, Vec<_>) = split_lines(&input)
        .iter()
        .filter_map(|s| get_pair_from_whitespace_separated_elements::<u32>(s))
        .unzip();

    let ans_part_one = get_pair_wise_difference_of_sorted_vecs(fst.clone(), snd.clone());

    println!(
        "The total difference between both lists is: {}",
        ans_part_one
    );
}
