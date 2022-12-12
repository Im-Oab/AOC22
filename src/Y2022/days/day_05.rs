use std::time::Instant;

use itertools::Itertools;
use std::collections::HashMap;

use crate::file_handler::FileHandler;

pub struct Day05 {}

impl Day05 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_05_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day05::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day05::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_05".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// The difficult part of this puzzle is parsing the input and separate into initial create stacks
    /// and crane moving commands after finishing parsing the input. The rest is very straight forward.
    fn part_01(lines: &Vec<&str>) -> String {
        let (mut stacks, commands) = Day05::parsing_input(lines);

        for command in commands.iter() {
            for _ in 0..(command.0) {
                Day05::move_one_crate(&mut stacks, command.1, command.2);
            }
        }

        let mut result = String::new();
        for index in 0..stacks.len() {
            let stack = stacks.get(&(index + 1)).unwrap();
            let crate_name = Day05::get_top_crate(stack);
            result.push_str(crate_name.as_str());
        }

        return result;
    }

    /// For the part 2, I have to modify function for moving one crate at a time into multiple crates.
    fn part_02(lines: &Vec<&str>) -> String {
        let (mut stacks, commands) = Day05::parsing_input(lines);

        for command in commands.iter() {
            Day05::move_crates(&mut stacks, command.0, command.1, command.2);
        }

        let mut result = String::new();
        for index in 0..stacks.len() {
            let stack = stacks.get(&(index + 1)).unwrap();
            let crate_name = Day05::get_top_crate(stack);
            result.push_str(crate_name.as_str());
        }

        return result;
    }

    fn move_one_crate(stacks: &mut HashMap<usize, Vec<String>>, start: usize, end: usize) {
        Day05::move_crates(stacks, 1, start, end);
    }

    fn move_crates(
        stacks: &mut HashMap<usize, Vec<String>>,
        total_moving_crate: usize,
        start: usize,
        end: usize,
    ) {
        let source_stack = stacks.get_mut(&start).unwrap();
        let total_crates = source_stack.len();
        let mut top_crates = source_stack
            .drain((total_crates - total_moving_crate)..)
            .collect();
        let destination_stack = stacks.get_mut(&end).unwrap();
        destination_stack.append(&mut top_crates);
    }

    fn get_top_crate(stack: &Vec<String>) -> String {
        if let Some(letter) = stack.last() {
            letter.to_owned()
        } else {
            "".to_owned()
        }
    }

    fn parsing_input(
        lines: &Vec<&str>,
    ) -> (HashMap<usize, Vec<String>>, Vec<(usize, usize, usize)>) {
        let total_stacks = Day05::get_total_stacks(lines[0]);
        let split_index = Day05::get_split_part_index(lines);

        let stack_input: Vec<&str> = lines.clone().drain(..(split_index - 1)).collect();

        let mut stacks = HashMap::new();
        for index in 0..total_stacks {
            let stack = Day05::get_stack(&stack_input, index);
            stacks.insert(index + 1, stack);
        }

        let mut commands = vec![];
        let mut command_input = lines.clone();
        command_input.drain(..(split_index + 1));
        for line in command_input.iter() {
            let command = Day05::parsing_command(line);
            commands.push(command);
        }

        (stacks, commands)
    }

    fn parsing_command(line: &str) -> (usize, usize, usize) {
        let splited_parts: Vec<&str> = line.split(" from ").collect();
        let total_crates = splited_parts[0]
            .clone()
            .replace("move ", "")
            .parse::<usize>()
            .unwrap();
        let positions: Vec<&str> = splited_parts[1].clone().split(" to ").collect();
        let start = positions[0].parse::<usize>().unwrap();
        let end = positions[1].parse::<usize>().unwrap();
        (total_crates, start, end)
    }

    fn get_stack(input: &Vec<&str>, stack_index: usize) -> Vec<String> {
        let mut result = vec![];
        let index = (stack_index * 4) + 1;
        for line in input.iter().rev() {
            let letter = line.chars().nth(index).unwrap();
            if letter != ' ' {
                result.push(String::from(letter));
            }
        }

        result
    }

    fn get_total_stacks(line: &str) -> usize {
        (line.len() + 1) / 4
    }

    fn get_split_part_index(lines: &Vec<&str>) -> usize {
        let (index, _) = lines.iter().find_position(|line| line.len() == 0).unwrap();
        index
    }
}

const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day05::part_01(&lines);
    assert_eq!(result, "CMZ");
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day05::part_02(&lines);
    assert_eq!(result, "MCD");
}
