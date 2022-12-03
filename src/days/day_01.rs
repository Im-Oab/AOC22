use std::time::Instant;

use crate::file_handler::FileHandler;

pub struct Day01 {}

impl Day01 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/inputs/day_01_1.txt");

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

    ///  I wrote something to get the result as soon as possible without thinking.
    fn part_01(lines: &Vec<&str>) -> i32 {
        let mut total_calories = 0;
        let mut max_calories = 0;

        for line in lines.iter() {
            let value = (*line).to_owned();
            if value.len() > 0 {
                total_calories += value.parse::<i32>().unwrap_or(0);
            } else {
                if total_calories > max_calories {
                    max_calories = total_calories;
                }

                total_calories = 0;
            }
        }

        return max_calories;
    }

    /// It needs to use Vec because it will quickly find three maximum values by sorting.
    fn part_02(lines: &Vec<&str>) -> i32 {
        let mut total_calories = 0;
        let mut calories = vec![];

        for line in lines.iter() {
            let value = (*line).to_owned();
            if value.len() > 0 {
                total_calories += value.parse::<i32>().unwrap_or(0);
            } else {
                calories.push(total_calories);
                total_calories = 0;
            }
        }
        calories.push(total_calories);
        calories.sort();

        // take last 3 values of sorted list and sum.
        let total_sum_of_calories = calories.iter().rev().take(3).sum();

        return total_sum_of_calories;
    }
}

const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day01::part_01(&lines);
    assert_eq!(result, 24000);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day01::part_02(&lines);
    assert_eq!(result, 45000);
}
