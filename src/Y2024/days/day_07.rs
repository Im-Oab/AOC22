use crate::file_handler::FileHandler;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day07 {}

impl Day07 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2024/inputs/day_07_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day07::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day07::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_07".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> u64 {
        let input = parsing_input(lines);

        process_two_operators(input)
    }

    fn part_02(lines: &Vec<&str>) -> u64 {
        let input = parsing_input(lines);

        process_three_operators(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day07::part_01(&lines);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day07::part_02(&lines);
        assert_eq!(result, 11387);
    }

    #[test]
    fn test_parse_input() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = parsing_input(&lines);
        assert_eq!(190, result[0].0);
        assert_eq!(vec![10, 19], result[0].1);
    }

    #[test]
    fn test_total_operators() {
        assert_eq!(1, total_operators(2));
        assert_eq!(2, total_operators(3));
        assert_eq!(3, total_operators(4));
    }

    #[test]
    fn test_make_operators() {
        let mut result = vec![];
        make_two_operators(&mut result, vec![], 2);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_process_combination() {
        let result = process_combination(&vec![10, 19], &vec![Operators::Mul]);
        assert_eq!(result, 190);

        let result = process_combination(&vec![81, 40, 27], &vec![Operators::Add, Operators::Mul]);
        assert_eq!(result, 3267);

        let result = process_combination(&vec![81, 40, 27], &vec![Operators::Mul, Operators::Add]);
        assert_eq!(result, 3267);

        let result = process_combination(&vec![15, 6], &vec![Operators::Concat]);
        assert_eq!(result, 156);

        let result = process_combination(
            &vec![6, 8, 6, 15],
            &vec![Operators::Mul, Operators::Concat, Operators::Mul],
        );
        assert_eq!(result, 7290);
    }
}

fn process_two_operators(input: Vec<(u64, Vec<u64>)>) -> u64 {
    input
        .iter()
        .map(|(test_value, numbers)| {
            let total_operators = total_operators(numbers.len());
            let mut operator_combinations = vec![];
            make_two_operators(&mut operator_combinations, vec![], total_operators);

            // println!("Numbers: {:?}", numbers);
            let equal_count = operator_combinations
                .iter()
                .map(|operators| {
                    // println!("operators: {:?}", operators);
                    let processed_result: u64 = process_combination(numbers, operators);
                    // println!("{}", processed_result);
                    if *test_value == processed_result {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u64>();

            // println!("==");

            if equal_count > 0 {
                *test_value
            } else {
                0
            }
        })
        .sum()
}

fn process_three_operators(input: Vec<(u64, Vec<u64>)>) -> u64 {
    input
        .iter()
        .map(|(test_value, numbers)| {
            let total_operators = total_operators(numbers.len());
            let mut operator_combinations = vec![];
            make_three_operators(&mut operator_combinations, vec![], total_operators);

            // println!("Numbers: {:?}", numbers);
            let equal_count = operator_combinations
                .iter()
                .map(|operators| {
                    // println!("operators: {:?}", operators);
                    let processed_result: u64 = process_combination(numbers, operators);
                    // println!("{}", processed_result);
                    if *test_value == processed_result {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u64>();

            // println!("==");

            if equal_count > 0 {
                *test_value
            } else {
                0
            }
        })
        .sum()
}

fn process_combination(numbers: &Vec<u64>, operators: &Vec<Operators>) -> u64 {
    let mut left = numbers[0];
    operators.iter().enumerate().for_each(|(index, op)| {
        let right = numbers[index + 1];

        left = op.process(left, right)
    });

    left
}

#[derive(Debug, Clone)]
enum Operators {
    Add,
    Mul,
    Concat,
}

impl Operators {
    fn process(&self, a: u64, b: u64) -> u64 {
        match self {
            Operators::Add => a + b,
            Operators::Mul => a * b,
            Operators::Concat => {
                let text = format!("{}{}", a, b);
                text.parse::<u64>().unwrap()
            }
        }
    }
}

fn make_two_operators(
    result: &mut Vec<Vec<Operators>>,
    list: Vec<Operators>,
    total_operators: usize,
) {
    if total_operators == 0 {
        result.push(list);
    } else {
        let mut add_list = list.clone();
        add_list.push(Operators::Add);
        make_two_operators(result, add_list, total_operators.saturating_sub(1));

        let mut mul_list = list.clone();
        mul_list.push(Operators::Mul);
        make_two_operators(result, mul_list, total_operators.saturating_sub(1));
    }
}

fn make_three_operators(
    result: &mut Vec<Vec<Operators>>,
    list: Vec<Operators>,
    total_operators: usize,
) {
    if total_operators == 0 {
        result.push(list);
    } else {
        let mut add_list = list.clone();
        add_list.push(Operators::Add);
        make_three_operators(result, add_list, total_operators.saturating_sub(1));

        let mut mul_list = list.clone();
        mul_list.push(Operators::Mul);
        make_three_operators(result, mul_list, total_operators.saturating_sub(1));

        let mut concat_list = list.clone();
        concat_list.push(Operators::Concat);
        make_three_operators(result, concat_list, total_operators.saturating_sub(1));
    }
}

fn get_combination(total_numbers: usize) {
    let total_operators = total_operators(total_numbers);
    let posibilities = total_posibilities(total_numbers);
}

fn total_posibilities(total_numbers: usize) -> usize {
    let total_operators = total_operators(total_numbers);
    2i32.pow(total_operators as u32) as usize
}

fn total_operators(total_numbers: usize) -> usize {
    total_numbers - 1
}

fn parsing_input(lines: &Vec<&str>) -> Vec<(u64, Vec<u64>)> {
    lines
        .iter()
        .map(|text| {
            let splited: Vec<&str> = text.split(": ").collect();
            let left = splited[0].parse::<u64>().unwrap();
            let values: Vec<&str> = splited[1].split(" ").collect();
            let right = values
                .iter()
                .map(|value| value.parse::<u64>().unwrap())
                .collect();

            (left, right)
        })
        .collect()
}

const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
