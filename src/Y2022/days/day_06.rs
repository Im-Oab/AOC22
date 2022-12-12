use std::time::Instant;

use itertools::Itertools;

use crate::file_handler::FileHandler;

pub struct Day06 {}

impl Day06 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_06_1.txt");

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
    /// check common character on 4 chracters window on long string.
    fn part_01(lines: &Vec<&str>) -> i32 {
        let input = lines[0];

        let window_size = 4;
        let mut buffer: Vec<char> = vec![];
        let mut index = 0;
        for c in input.chars() {
            index += 1;

            if let Some(pos) = Day06::check_common_character_on_buffer(&buffer, &c) {
                Day06::remove_incorrect_buffer(&mut buffer, pos);
            } else if buffer.len() == window_size - 1 {
                break;
            }

            Day06::add_to_buffer(&mut buffer, c.clone());
        }
        return index as i32;
    }

    /// I reuse the same code but change from 4 to 14 characters.
    fn part_02(lines: &Vec<&str>) -> i32 {
        let input = lines[0];

        let window_size = 14;
        let mut buffer: Vec<char> = vec![];
        let mut index = 0;
        for c in input.chars() {
            index += 1;

            if let Some(pos) = Day06::check_common_character_on_buffer(&buffer, &c) {
                Day06::remove_incorrect_buffer(&mut buffer, pos);
            } else if buffer.len() == window_size - 1 {
                break;
            }

            Day06::add_to_buffer(&mut buffer, c.clone());
        }
        return index as i32;
    }

    fn add_to_buffer(buffer: &mut Vec<char>, c: char) {
        buffer.push(c);
    }

    fn remove_incorrect_buffer(buffer: &mut Vec<char>, incorrect_index: usize) {
        buffer.drain(..=incorrect_index);
    }

    fn check_common_character_on_buffer(buffer: &Vec<char>, interested: &char) -> Option<usize> {
        if let Some((pos, _)) = buffer.iter().find_position(|value| **value == *interested) {
            Some(pos)
        } else {
            None
        }
    }
}

const TEST_INPUT: [(&str, i32); 5] = [
    ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
    ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
    ("nppdvjthqldpwncqszvftbrmjlhg", 6),
    ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
    ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
];

#[test]
fn test_part_1() {
    for (line, expected_result) in TEST_INPUT.iter() {
        let lines: Vec<&str> = line.lines().collect();
        let result = Day06::part_01(&lines);
        assert_eq!(result, *expected_result);
    }
}

const TEST_INPUT_2: [(&str, i32); 5] = [
    ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
    ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
    ("nppdvjthqldpwncqszvftbrmjlhg", 23),
    ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
    ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
];

#[test]
fn test_part_2() {
    for (line, expected_result) in TEST_INPUT_2.iter() {
        let lines: Vec<&str> = line.lines().collect();
        let result = Day06::part_02(&lines);
        assert_eq!(result, *expected_result);
    }
}
