use std::{collections::VecDeque, time::Instant};

use hashbrown::HashMap;

use crate::file_handler::FileHandler;

pub struct Day04 {}

impl Day04 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_04_1.txt");

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

    fn part_01(lines: &Vec<&str>) -> u128 {
        let mut total_points = 0;
        for input in lines.iter() {
            let (_, winning_numbers, numbers) = parsing_card(input);
            let points = validate_card_winning(&winning_numbers, &numbers);

            total_points += points;
        }

        return total_points;
    }

    fn part_02(lines: &Vec<&str>) -> u128 {
        let mut card_references = HashMap::new();
        let mut process_queue: VecDeque<i32> = VecDeque::new();

        let last_card_number = lines.len() as i32;
        for input in lines.iter() {
            let (card_number, winning_numbers, numbers) = parsing_card(input);
            if let Some(copy_list) =
                get_copy_list(card_number, last_card_number, &winning_numbers, &numbers)
            {
                card_references.insert(card_number, copy_list);
            }

            process_queue.push_back(card_number);
        }

        let mut total_all_cards = last_card_number as u128;
        loop {
            if let Some(card_number) = process_queue.pop_front() {
                if let Some(list) = card_references.get(&card_number) {
                    total_all_cards += list.len() as u128;
                    process_queue.append(&mut list.clone());
                }
            }

            if process_queue.len() == 0 {
                break;
            }
        }

        return total_all_cards;
    }
}

fn parsing_card(input: &str) -> (i32, Vec<i32>, Vec<i32>) {
    let (card_number, input) = parsing_card_number(input);
    let list_of_numbers: Vec<&str> = input.split(" | ").collect();
    let winning_numbers = parsing_numbers(list_of_numbers[0]);
    let numbers = parsing_numbers(list_of_numbers[1]);

    (card_number, winning_numbers, numbers)
}

fn parsing_card_number(line: &str) -> (i32, String) {
    let values: Vec<&str> = line.split(": ").collect();
    let card_number = values[0]
        .replace("Card", "")
        .replace(" ", "")
        .parse::<i32>()
        .unwrap();

    (card_number, values[1].to_owned())
}

fn parsing_numbers(line: &str) -> Vec<i32> {
    let mut result = vec![];

    let values: Vec<&str> = line.split(" ").collect();
    for v in values.iter() {
        if let Ok(number) = (*v).parse::<i32>() {
            result.push(number);
        }
    }
    result
}

fn validate_card_winning(winning_numbers: &Vec<i32>, numbers: &Vec<i32>) -> u128 {
    let total_matching = count_matching_numbers(winning_numbers, numbers);
    calculate_winning_points(total_matching)
}

fn count_matching_numbers(winning_numbers: &Vec<i32>, numbers: &Vec<i32>) -> i32 {
    let mut total_matching: i32 = 0;
    for v in numbers.iter() {
        if winning_numbers.contains(v) {
            total_matching += 1;
        }
    }

    total_matching
}

fn calculate_winning_points(total_matching: i32) -> u128 {
    if total_matching > 0 {
        2u128.pow(total_matching as u32 - 1)
    } else {
        0
    }
}

fn get_copy_list(
    card_number: i32,
    last_card_number: i32,
    winning_numbers: &Vec<i32>,
    numbers: &Vec<i32>,
) -> Option<VecDeque<i32>> {
    let total_matching = count_matching_numbers(winning_numbers, numbers);
    if total_matching <= 0 {
        return None;
    }
    let mut result = VecDeque::new();
    for count in 0..total_matching {
        let copy_card_number = card_number + (count + 1);
        if copy_card_number > last_card_number {
            break;
        }

        result.push_back(copy_card_number);
    }

    Some(result)
}

const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_number() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (card_number, remain_input) = parsing_card_number(input);
        assert_eq!(card_number, 1);
        assert_eq!(remain_input, "41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    }

    #[test]
    fn test_parsing_numbers() {
        let input = "41 48 83 86 17";
        let result = parsing_numbers(input);
        assert_eq!(result.len(), 5);
        assert_eq!(result, vec![41, 48, 83, 86, 17]);

        let input = "83 86  6 31 17  9 48 53";
        let result = parsing_numbers(input);
        assert_eq!(result.len(), 8);
        assert_eq!(result, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_validating_winning_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (card_number, winning_numbers, numbers) = parsing_card(input);
        let points = validate_card_winning(&winning_numbers, &numbers);
        assert_eq!(points, 8);
    }

    #[test]
    fn test_get_copy_card_numbers() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (card_number, winning_numbers, numbers) = parsing_card(input);
        let result = get_copy_list(card_number, 6, &winning_numbers, &numbers);
        assert!(result.is_some());
        assert_eq!(result, Some(VecDeque::from([2, 3, 4, 5])));

        let result = get_copy_list(card_number, 3, &winning_numbers, &numbers);
        assert!(result.is_some());
        assert_eq!(result, Some(VecDeque::from([2, 3])));
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day04::part_01(&lines);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day04::part_02(&lines);
        assert_eq!(result, 30);
    }
}
