use std::time::Instant;

use std::collections::{HashMap, HashSet};

use num::Integer;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

use crate::file_handler::FileHandler;

pub struct Day08 {}

impl Day08 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_08_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day08::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day08::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_08".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> u128 {
        let (commands, references) = parse_input(lines);

        find_total_steps(&commands, &references, "AAA")
    }

    fn part_02(lines: &Vec<&str>) -> u128 {
        let (commands, references) = parse_input(lines);
        let start_values = find_all_start_values(&references);
        let total_steps: Vec<u128> = start_values
            .par_iter()
            .map(|start_value| find_total_steps(&commands, &references, start_value))
            .collect();
        println!("{:?}", total_steps);

        let mut first = total_steps[0];
        for index in 1..total_steps.len() {
            first = first.lcm(&total_steps[index]);
        }

        first
    }
}

fn find_total_steps(
    commands: &Vec<i32>,
    references: &HashMap<String, (String, String)>,
    start_value: &str,
) -> u128 {
    let mut total_step = 0;
    let mut current_navigation = references.get(start_value).clone().unwrap();
    let mut index = 0;
    loop {
        let next_command = if let Some(command) = commands.get(index) {
            total_step += 1;
            if *command == LEFT {
                current_navigation.0.to_owned()
            } else if *command == RIGHT {
                current_navigation.1.to_owned()
            } else {
                panic!("It should not reach here");
            }
        } else {
            panic!("It should not reach here");
        };

        if next_command.chars().last().unwrap() == 'Z' {
            break;
        } else {
            current_navigation = references.get(&next_command).clone().unwrap();
            index = (index + 1) % commands.len();
        }
    }

    return total_step;
}

fn parse_input(input: &Vec<&str>) -> (Vec<i32>, HashMap<String, (String, String)>) {
    let commands = input[0];
    let references: Vec<&str> = input
        .par_iter()
        .enumerate()
        .filter_map(|(index, value)| if index < 2 { None } else { Some(*value) })
        .collect();

    (parse_commands(commands), parse_references(&references))
}

const LEFT: i32 = -1;
const RIGHT: i32 = -3;
fn parse_commands(input: &str) -> Vec<i32> {
    input
        .par_chars()
        .map(|c| match c {
            'L' => LEFT,
            'R' => RIGHT,
            _ => 0,
        })
        .collect()
}

fn parse_references(input: &Vec<&str>) -> HashMap<String, (String, String)> {
    input
        .par_iter()
        .map(|text| {
            let values: Vec<&str> = (*text).split(" = ").collect();
            let key = values[0].to_owned();

            let right = values[1].replace("(", "").replace(")", "").replace(" ", "");
            let navigation: Vec<&str> = right.split(",").collect();

            (key, (navigation[0].to_owned(), navigation[1].to_owned()))
        })
        .collect()
}

fn find_all_start_values(references: &HashMap<String, (String, String)>) -> HashSet<String> {
    references
        .par_iter()
        .filter_map(|(key, _)| {
            if key.chars().last().unwrap() == 'A' {
                Some(key.to_owned())
            } else {
                None
            }
        })
        .collect()
}

fn find_all_end_values(references: &HashMap<String, (String, String)>) -> HashSet<String> {
    references
        .par_iter()
        .filter_map(|(key, _)| {
            if key.chars().last().unwrap() == 'Z' {
                Some(key.to_owned())
            } else {
                None
            }
        })
        .collect()
}

const TEST_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const TEST_INPUT_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start_values() {
        let lines: Vec<&str> = TEST_INPUT_2.lines().collect();
        let (_, references) = parse_input(&lines);

        let result = find_all_start_values(&references);
        println!("{:?}", result);
        assert!(result.contains("11A"));
        assert!(result.contains("22A"));
    }

    #[test]
    fn test_parse_commands() {
        let result = parse_commands("RL");
        assert_eq!(result, vec![RIGHT, LEFT]);
    }

    #[test]
    fn test_parse_references() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let references: Vec<&str> = lines
            .par_iter()
            .enumerate()
            .filter_map(|(index, value)| if index < 2 { None } else { Some(*value) })
            .collect();

        let result = parse_references(&references);
        assert_eq!(result["AAA"], (String::from("BBB"), String::from("CCC")));
    }
    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day08::part_01(&lines);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT_2.lines().collect();
        let result = Day08::part_02(&lines);
        assert_eq!(result, 6);
    }
}
