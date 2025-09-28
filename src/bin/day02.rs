use std::convert::identity;

use advent_of_code_2024_rust::{read_input, split_lines};

#[derive(Clone, Copy)]
enum ReportMonotonicity {
    Increasing,
    Decreasing,
}

fn process_raw_report(raw_report: &&str) -> Vec<u32> {
    raw_report
        .split_whitespace()
        .map(|raw_level| raw_level.parse::<u32>().unwrap())
        .collect()
}

fn get_report_monotonicity(report: &[u32]) -> Option<ReportMonotonicity> {
    match report {
        [a, b, ..] if a < b => Some(ReportMonotonicity::Increasing),
        [a, b, ..] if a > b => Some(ReportMonotonicity::Decreasing),
        _ => None,
    }
}

fn window_preserves_monotonicity_and_differ_by_at_most_three(
    window: &[u32],
    monotonicity: ReportMonotonicity,
) -> bool {
    match window {
        [a, b, ..] => {
            let difference_is_at_most_3 = a.abs_diff(*b) <= 3;
            match monotonicity {
                ReportMonotonicity::Increasing => a < b && difference_is_at_most_3,
                ReportMonotonicity::Decreasing => a > b && difference_is_at_most_3,
            }
        }
        _ => false,
    }
}

fn report_is_safe(report: &[u32]) -> bool {
    let report_monotonicity = get_report_monotonicity(report);

    match report_monotonicity {
        Some(monotonicity) => report
            .windows(2)
            .map(|window| {
                window_preserves_monotonicity_and_differ_by_at_most_three(window, monotonicity)
            })
            .all(identity),
        None => false,
    }
}

fn get_number_of_safe_reports(reports: &Vec<Vec<u32>>) -> usize {
    reports
        .iter()
        .filter(|report| report_is_safe(report))
        .count()
}

fn main() {
    let input = read_input(2);
    let reports: Vec<Vec<u32>> = split_lines(&input)
        .iter()
        .map(|raw_report| process_raw_report(raw_report))
        .collect();

    let ans_part_one = get_number_of_safe_reports(&reports);

    println!("The number of safe reports is: {}", ans_part_one)
}
