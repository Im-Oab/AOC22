use crate::file_handler::FileHandler;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day01 {}

impl Day01 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2024/inputs/day_01_1.txt");
        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day01::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day01::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_01".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i32 {
        let (left_list, right_list) = split_list_and_sorted(lines);

        (0..left_list.len())
            .map(|index| find_distance(left_list[index], right_list[index]))
            .sum()
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let (left_list, right_list) = split_list_and_sorted(lines);

        left_list
            .iter()
            .map(|checked_value| calculate_similarity_score(*checked_value, &right_list))
            .sum()
    }
}

fn calculate_similarity_score(checked_value: i32, list: &Vec<i32>) -> i32 {
    list.iter().filter(|value| checked_value == **value).count() as i32 * checked_value
}

fn split_list_and_sorted(list: &Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let list: Vec<(i32, i32)> = list
        .iter()
        .map(|text| {
            let values: Vec<&str> = text.split_whitespace().collect();

            return (
                values.first().unwrap().to_owned().parse::<i32>().unwrap(),
                values.last().unwrap().to_owned().parse::<i32>().unwrap(),
            );
        })
        .collect();

    let mut left_list: Vec<i32> = list.iter().map(|(left, _)| *left).collect();
    left_list.sort();

    let mut right_list: Vec<i32> = list.iter().map(|(_, right)| *right).collect();
    right_list.sort();

    (left_list, right_list)
}

fn find_distance(left: i32, right: i32) -> i32 {
    (right - left).abs()
}

const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day01::part_01(&lines);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day01::part_02(&lines);
        assert_eq!(result, 31);
    }
}
