use advent_of_code_2024_rust::{
    get_frequencies, get_pair_from_whitespace_separated_elements, read_input, split_lines,
};
use std::iter;

fn get_pair_wise_difference_of_sorted_vecs(mut fst: Vec<u32>, mut snd: Vec<u32>) -> u32 {
    fst.sort();
    snd.sort();

    iter::zip(fst, snd)
        .map(|pair| pair.0.abs_diff(pair.1))
        .sum()
}

fn get_similarity_score(fst: Vec<u32>, snd: Vec<u32>) -> u32 {
    let frequencies = get_frequencies(snd);

    fst.into_iter()
        .map(|n| n * (*frequencies.get(&n).unwrap_or(&0) as u32))
        .sum()
}

fn main() {
    let input = read_input(1);
    let (fst, snd): (Vec<_>, Vec<_>) = split_lines(&input)
        .iter()
        .filter_map(|s| get_pair_from_whitespace_separated_elements::<u32>(s))
        .unzip();

    let ans_part_one = get_pair_wise_difference_of_sorted_vecs(fst.clone(), snd.clone());
    let ans_part_two = get_similarity_score(fst, snd);

    println!(
        "The total difference between both lists is: {}",
        ans_part_one
    );

    println!(
        "The similarity score between both lists is: {}",
        ans_part_two
    )
}
