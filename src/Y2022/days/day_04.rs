use std::collections::HashSet;
use std::time::Instant;

use crate::file_handler::FileHandler;

pub struct Day04 {}

impl Day04 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_04_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day04::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day04::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_04".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }
    /// After I read the puzzle, I want to try using all() to run through the list and check each value instead of the solution that I did yesterday.
    fn part_01(lines: &Vec<&str>) -> i32 {
        let mut fully_contain = 0;
        for line in lines.iter() {
            let persons = Day04::split_pair(line);
            let first_assignment = Day04::convert_sections(persons[0]);
            let second_assignment = Day04::convert_sections(persons[1]);

            if first_assignment.len() >= second_assignment.len() {
                if Day04::contain_another(first_assignment, second_assignment) {
                    fully_contain += 1;
                }
            } else {
                if Day04::contain_another(second_assignment, first_assignment) {
                    fully_contain += 1;
                }
            }
        }
        return fully_contain;
    }

    /// This part is easy because, from part 1, I already check whether it contains another. I modified the code to check if some parts overlap or not.
    fn part_02(lines: &Vec<&str>) -> i32 {
        let mut overlap_count = 0;
        for line in lines.iter() {
            let persons = Day04::split_pair(line);
            let first_assignment = Day04::convert_sections(persons[0]);
            let second_assignment = Day04::convert_sections(persons[1]);

            if first_assignment.len() >= second_assignment.len() {
                if Day04::overlap_another(first_assignment, second_assignment) {
                    overlap_count += 1;
                }
            } else {
                if Day04::overlap_another(second_assignment, first_assignment) {
                    overlap_count += 1;
                }
            }
        }
        return overlap_count;
    }

    fn contain_another(long: Vec<usize>, short: Vec<usize>) -> bool {
        let hash: HashSet<usize> = long.iter().copied().collect();
        short.iter().all(|value| hash.contains(value))
    }

    fn overlap_another(long: Vec<usize>, short: Vec<usize>) -> bool {
        let hash: HashSet<usize> = long.iter().copied().collect();
        short.iter().any(|value| hash.contains(value))
    }

    fn split_pair(pair: &str) -> Vec<&str> {
        let persons: Vec<&str> = pair.split(",").collect();
        persons
    }

    fn convert_sections(section: &str) -> Vec<usize> {
        let values: Vec<&str> = section.split("-").collect();
        let first = values[0].parse::<usize>().unwrap();
        let second = values[1].parse::<usize>().unwrap();
        (first..=second).collect()
    }
}

const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day04::part_01(&lines);
    assert_eq!(result, 2);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day04::part_02(&lines);
    assert_eq!(result, 4);
}
