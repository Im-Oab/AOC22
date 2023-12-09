use std::{collections::VecDeque, time::Instant};

use rayon::{
    iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator},
    str::ParallelString,
};

use crate::file_handler::FileHandler;

pub struct Day09 {}

impl Day09 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_09_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day09::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day09::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_09".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i128 {
        let reports = parse_input(lines);

        reports
            .iter()
            .map(|input| process_extrapolated(input))
            .sum()
    }

    fn part_02(lines: &Vec<&str>) -> i128 {
        let reports = parse_input(lines);

        reports
            .iter()
            .map(|input| process_extrapolated_backward(input))
            .sum()
    }
}

fn process_extrapolated_backward(input: &Vec<i128>) -> i128 {
    let start_values = input.clone();
    let mut sequences = vec![];
    sequences.push(start_values.clone());
    loop {
        let current_values = find_different_step(sequences.last().clone().unwrap());
        sequences.push(current_values.clone());

        if current_values.par_iter().all(|v| *v == 0) {
            break;
        }
    }

    let mut sequences: Vec<VecDeque<i128>> = sequences
        .par_iter()
        .map(|v| VecDeque::from(v.clone()))
        .collect();

    for index in (0..sequences.len()).rev() {
        let prev = sequences.get(index + 1);
        find_extrapolated_backward(prev.cloned(), sequences.get_mut(index).unwrap());
    }

    sequences.first().unwrap().front().unwrap().clone()
}

fn find_extrapolated_backward(prev: Option<VecDeque<i128>>, current: &mut VecDeque<i128>) {
    if prev.is_none() {
        current.push_front(0);
    } else {
        let prev_first_value = prev.unwrap().front().unwrap().clone();
        let first_value = current.front().unwrap();
        current.push_front(*first_value - prev_first_value);
    }
}

fn process_extrapolated(input: &Vec<i128>) -> i128 {
    let start_values = input.clone();
    let mut sequences = vec![];
    sequences.push(start_values.clone());
    loop {
        let current_values = find_different_step(sequences.last().clone().unwrap());
        sequences.push(current_values.clone());

        if current_values.par_iter().all(|v| *v == 0) {
            break;
        }
    }

    for index in (0..sequences.len()).rev() {
        let prev = sequences.get(index + 1);
        find_extrapolated(prev.cloned(), sequences.get_mut(index).unwrap());
    }

    sequences.first().unwrap().last().unwrap().clone()
}

fn find_extrapolated(prev: Option<Vec<i128>>, current: &mut Vec<i128>) {
    if prev.is_none() {
        current.push(0);
    } else {
        let prev_last_value = prev.unwrap().last().unwrap().clone();
        let last_value = current.last().unwrap();
        current.push(*last_value + prev_last_value);
    }
}

fn parse_input(input: &Vec<&str>) -> Vec<Vec<i128>> {
    input.par_iter().map(|v| parse_line(*v)).collect()
}

fn find_different_step(numbers: &Vec<i128>) -> Vec<i128> {
    numbers
        .windows(2)
        .map(|values| values[1] - values[0])
        .collect()
}

fn parse_line(input: &str) -> Vec<i128> {
    input
        .split(" ")
        .filter_map(|c| {
            if let Ok(v) = c.parse::<i128>() {
                Some(v)
            } else {
                None
            }
        })
        .collect()
}

const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_prediction() {
        let prev = None;
        let mut current = vec![0, 0, 0, 0];
        find_extrapolated(prev, &mut current);
        assert_eq!(current.len(), 5);
        assert_eq!(current, vec![0, 0, 0, 0, 0]);

        let prev = Some(current.clone());
        let mut current = vec![3, 3, 3, 3, 3];
        find_extrapolated(prev, &mut current);
        assert_eq!(current.len(), 6);
        assert_eq!(current, vec![3, 3, 3, 3, 3, 3]);

        let prev = Some(current.clone());
        let mut current = vec![0, 3, 6, 9, 12, 15];
        find_extrapolated(prev, &mut current);
        assert_eq!(current.len(), 7);
        assert_eq!(current, vec![0, 3, 6, 9, 12, 15, 18]);
    }

    #[test]
    fn test_parser() {
        let result = parse_line("0 3 6 9 12 15");
        assert_eq!(result, vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_different_steps() {
        let input = parse_line("0 3 6 9 12 15");
        let result = find_different_step(&input);
        assert_eq!(result, vec![3, 3, 3, 3, 3]);

        let input = parse_line("1 3 6 10 15 21");
        let result = find_different_step(&input);
        assert_eq!(result, vec![2, 3, 4, 5, 6]);

        let input = parse_line("10 13 16 21 30 45");
        let result = find_different_step(&input);
        assert_eq!(result, vec![3, 3, 5, 9, 15]);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day09::part_01(&lines);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day09::part_02(&lines);
        assert_eq!(result, 2);
    }
}
