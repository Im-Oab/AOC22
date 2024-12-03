use crate::file_handler::FileHandler;
use rayon::prelude::*;

use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::usize;

pub struct Day03 {}

impl Day03 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2024/inputs/day_03_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day03::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day03::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_03".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i32 {
        lines
            .iter()
            .map(|text| {
                let operators = split_operators(*text);
                operators
                    .iter()
                    .map(|(_, (left, right))| left * right)
                    .sum::<i32>()
            })
            .sum()
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let mut combined_lines = String::new();
        lines.iter().for_each(|text| {
            combined_lines = format!("{}{}", combined_lines, text);
        });
        vec![&combined_lines]
            .iter()
            .map(|text| {
                let operators = split_operators(*text);
                let enable_ranges = find_all_stop_operator(&text);
                operators
                    .iter()
                    .map(|(start_index, (left, right))| {
                        if enable_ranges
                            .iter()
                            .any(|(start, end)| start_index >= start && start_index < end)
                            == true
                        {
                            left * right
                        } else {
                            0
                        }
                    })
                    .sum::<i32>()
            })
            .sum()
    }
}

fn split_operators(line: &str) -> Vec<(usize, (i32, i32))> {
    let value = line.to_owned();
    let all_indexes: Vec<_> = value.match_indices("mul(").collect();

    all_indexes
        .iter()
        .filter_map(|(index, _)| is_complete_operator(&value, *index))
        .collect()
}

fn is_complete_operator(text: &String, start_index: usize) -> Option<(usize, (i32, i32))> {
    let last_index = (start_index + 12).min(text.len());
    let raw = text[start_index..last_index].to_owned();

    let comma_index = raw.find(",").unwrap_or(raw.len());
    let close_index = raw.find(")").unwrap_or(raw.len());

    if comma_index < raw.len() && close_index < raw.len() {
        let left = raw[4..comma_index].to_owned();
        let right = raw[comma_index + 1..close_index].to_owned();

        Some((
            start_index,
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()),
        ))
    } else {
        None
    }
}

fn find_all_stop_operator(line: &str) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut value = line.to_owned();
    let mut started_text_index = 0;

    loop {
        let (mut start, end) = find_range(&value);
        if start == usize::MAX {
            break;
        } else if end == usize::MAX {
            result.push((started_text_index + start, line.len()));
            break;
        }

        if result.len() == 0 {
            start = 0;
        }

        if start < end {
            result.push((started_text_index + start, started_text_index + end));
        }
        started_text_index += end + 7;
        if end + 7 < value.len() {
            value = value[(end + 7)..].to_owned();
        } else {
            break;
        }
    }

    result.sort_by(|a, b| a.0.cmp(&b.0));

    result
}

fn find_range(text: &str) -> (usize, usize) {
    let do_index = text.find("do()").unwrap_or(usize::MAX);
    let dont_index = text.find("don't()").unwrap_or(usize::MAX);

    (do_index, dont_index)
}

const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST_INPUT_2: &str = "xmul(2,4)&don't()mul[3,7]!^don't()_mul(5,5)+do()don't()do()don't()do()don't()mul(32,64]don't()don't()don't()don't()(mul(11,8)do()do()do()?undo()?mul(8,5)do()?)";
#[cfg(test)]
mod tests {
    use rayon::result;

    use super::*;

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day03::part_01(&lines);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT_2.lines().collect();
        let result = Day03::part_02(&lines);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_find_all_stop_operator() {
        let result = find_all_stop_operator(
            "xmul(2,4)&don't()mul[3,7]!^don't()_mul(5,5)+do()don't()do()don't()do()don't()",
        );
        assert!(result.contains(&(0, 10)));

        let result = find_all_stop_operator("xdo()don't()xdo()don't()");
        assert!(result.contains(&(0, 5)));
        assert!(result.contains(&(13, 17)));

        let result = find_all_stop_operator("xdo()don't()");
        assert!(result.contains(&(0, 5)));

        let result = find_all_stop_operator("xdo()do()don't()don't()");
        assert!(result.contains(&(0, 9)));

        let result = find_all_stop_operator("xgo()do()don't()don't()");
        assert!(result.contains(&(0, 9)))
    }

    #[test]
    fn test_find_range() {
        let (start, end) = find_range("xdo()don't()");
        assert_eq!(start, 1);
        assert_eq!(end, 5);

        let (start, end) = find_range("xdo()xxxx");
        assert_eq!(start, 1);
        assert_eq!(end, usize::MAX);

        let (start, end) = find_range("xgo()don't()");
        assert_eq!(start, usize::MAX);
        assert_eq!(end, 5);

        let (start, end) = find_range("xdo()do()do()don't()");
        assert_eq!(start, 1);
        assert_eq!(end, 13);

        let (start, end) = find_range("xgo()do()do()don't()");
        assert_eq!(start, 5);
        assert_eq!(end, 13);

        let (start, end) = find_range("xdo()don't()don't()don't()don't()");
        assert_eq!(start, 1);
        assert_eq!(end, 5);
    }
}
