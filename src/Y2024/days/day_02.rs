use crate::file_handler::FileHandler;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day02 {}

impl Day02 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2024/inputs/day_02_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day02::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day02::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_02".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i32 {
        let reports = split_reports(lines);

        reports
            .iter()
            .map(|report| if safety_check(report) == true { 1 } else { 0 })
            .sum()
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let mut reports = split_reports(lines);

        reports
            .iter_mut()
            .map(|report| {
                if safety_check(report) == true {
                    return 1;
                } else {
                    if (0..report.len()).any(|index| {
                        let new_report = remove_level(report, index);

                        if safety_check(&new_report) == true {
                            return true;
                        } else {
                            false
                        }
                    }) == true
                    {
                        return 1;
                    }
                }

                0
            })
            .sum()
    }
}

fn remove_level(report: &Vec<i32>, start_index: usize) -> Vec<i32> {
    let mut new_report = report.clone();
    new_report.remove(start_index);

    new_report
}

fn is_safe_adjacent(diff: i32) -> bool {
    [1, 2, 3].contains(&diff.abs())
}

fn safety_check(report: &Vec<i32>) -> bool {
    let mut start_direction = None;

    report.windows(2).enumerate().all(|(index, values)| {
        let left = values[0];
        let right = values[1];
        let diff = right - left;

        if start_direction.is_none() == true {
            start_direction = Some(diff.signum());
        }

        if let Some(sign) = start_direction.as_ref() {
            if is_safe_adjacent(diff) == true {
                return *sign == diff.signum();
            }
        }

        false
    })
}

fn split_reports(lines: &Vec<&str>) -> Vec<Vec<i32>> {
    lines.iter().map(|text| make_report(*text)).collect()
}

fn make_report(text: &str) -> Vec<i32> {
    text.split_whitespace()
        .map(|value| (value.to_owned()).parse::<i32>().unwrap())
        .collect()
}

const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day02::part_01(&lines);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day02::part_02(&lines);
        assert_eq!(result, 4);
    }
}
