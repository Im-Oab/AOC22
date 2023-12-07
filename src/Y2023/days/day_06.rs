use std::time::Instant;

use crate::file_handler::FileHandler;
use itertools::Itertools;
use rayon::prelude::*;

pub struct Day06 {}

impl Day06 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_06_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day06::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day06::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_06".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> u128 {
        let data = parsing_input(lines);

        let mut total_multiply = 1;
        for index in (0..data.0.len()) {
            let duration = data.0[index] as u128;
            let distance = data.1[index] as u128;

            let count = find_win_solutions(duration, distance);
            total_multiply *= count;
        }

        return total_multiply;
    }

    fn part_02(lines: &Vec<&str>) -> u128 {
        let (duration, distance) = parsing_input_2(lines);

        let count = find_win_solutions(duration, distance);

        return count;
    }
}

fn find_win_solutions(total_duration: u128, minimum_distance: u128) -> u128 {
    let v: Vec<_> = (0..total_duration).collect();
    let total_solutions = v
        .par_iter()
        .filter_map(|hold_duration| {
            let speed = hold_duration;
            let moving_duration = total_duration - hold_duration;
            let distance = speed * moving_duration;

            if distance > minimum_distance {
                Some(distance)
            } else {
                None
            }
        })
        .count();

    total_solutions as u128
}

fn parsing_input(lines: &Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let times = parsing_values(lines[0], "Time: ");
    let distance = parsing_values(lines[1], "Distance: ");

    (times, distance)
}

fn parsing_input_2(lines: &Vec<&str>) -> (u128, u128) {
    let times = parsing_values(lines[0], "Time: ");
    let distance = parsing_values(lines[1], "Distance: ");

    let mut text = String::new();
    for value in times.iter() {
        text.push_str(&format!("{}", value));
    }
    let duration = text.parse::<u128>().unwrap();

    let mut text = String::new();
    for value in distance.iter() {
        text.push_str(&format!("{}", value));
    }

    let distance = text.parse::<u128>().unwrap();

    (duration, distance)
}

fn parsing_values(input: &str, removing_prefix: &str) -> Vec<i32> {
    let input = input.replace(removing_prefix, "");
    let values: Vec<&str> = input.split(" ").collect();
    let mut result = vec![];
    for value in values.iter() {
        if let Ok(number) = value.parse::<i32>() {
            result.push(number);
        }
    }

    result
}

const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parsing_input() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (times, distances) = parsing_input(&lines);
        assert_eq!(times, vec![7, 15, 30]);
        assert_eq!(distances, vec![9, 40, 200]);
    }
    #[test]
    fn test_find_win_solutions() {
        let count = find_win_solutions(7, 9);
        assert_eq!(count, 4);

        let count = find_win_solutions(15, 40);
        assert_eq!(count, 8);

        let count = find_win_solutions(30, 200);
        assert_eq!(count, 9);
    }

    #[test]
    fn test_parsing_input_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (times, distances) = parsing_input_2(&lines);
        assert_eq!(times, 71530);
        assert_eq!(distances, 940200);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day06::part_01(&lines);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day06::part_02(&lines);
        assert_eq!(result, 71503);
    }
}
