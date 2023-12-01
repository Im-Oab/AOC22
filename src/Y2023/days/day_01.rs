use std::time::Instant;

use crate::file_handler::FileHandler;

pub struct Day01 {}

impl Day01 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_01_1.txt");

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
        let mut total_sum = 0;
        for line in lines.iter() {
            total_sum += Day01::get_digit_input(*line).0;
        }

        total_sum
    }

    fn get_digit_input(line: &str) -> (i32, [i32; 4]) {
        let mut result = 0;
        let mut digit_indexes = [i32::MAX, 0, -1, 0];
        // first digit
        for (index, c) in line.chars().enumerate() {
            if let Ok(value) = String::from(c).parse::<i32>() {
                digit_indexes[0] = index as i32;
                digit_indexes[1] = value;
                break;
            }
        }

        for (index, c) in line.chars().enumerate() {
            if let Ok(value) = String::from(c).parse::<i32>() {
                digit_indexes[2] = index as i32;
                digit_indexes[3] = value;
            }
        }

        result = (digit_indexes[1] * 10 + digit_indexes[3]) as i32;
        (result, digit_indexes)
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let mut total_sum = 0;
        for line in lines.iter() {
            total_sum += Day01::get_digit_and_letter(*line);
        }

        total_sum
    }

    fn get_digit_and_letter(line: &str) -> i32 {
        let letters_list = [
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        let (_, indexes) = Day01::get_digit_input(line);
        let mut left_index = indexes[0];
        let mut left_value = indexes[1] as i32;
        let mut right_index = indexes[2];
        let mut right_value = indexes[3] as i32;

        let text = line.to_owned();
        for (letters, value) in letters_list.iter() {
            let list: Vec<(usize, &str)> = text.match_indices(letters).collect();
            if let Some((start_index, _)) = list.first() {
                if (*start_index as i32) < left_index {
                    left_index = *start_index as i32;
                    left_value = *value;
                }
            }

            if let Some((start_index, _)) = list.last() {
                if (*start_index as i32) > right_index {
                    right_index = *start_index as i32;
                    right_value = *value;
                }
            }
        }

        let result = left_value * 10 + right_value;
        // println!("Lines: {}\nResult:{}", line, result);
        result
    }
}
const TEST_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

const TEST_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_input() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let expected_result = [12, 38, 15, 77];
        for index in 0..input.len() {
            assert_eq!(
                expected_result[index],
                Day01::get_digit_input(input[index]).0
            );
        }
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day01::part_01(&lines);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_digit_and_letter() {
        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "oneight",
        ];

        let expected_output = [29, 83, 13, 24, 42, 14, 76, 18];

        for index in 0..input.len() {
            assert_eq!(
                expected_output[index],
                Day01::get_digit_and_letter(input[index])
            );
        }
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT_2.lines().collect();
        let result = Day01::part_02(&lines);
        assert_eq!(result, 281);
    }
}
