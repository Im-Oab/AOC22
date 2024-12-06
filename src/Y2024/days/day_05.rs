use crate::file_handler::FileHandler;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day05 {}

impl Day05 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2024/inputs/day_05_1.txt");

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

    fn part_01(lines: &Vec<&str>) -> i32 {
        let (rules, updates) = split_input(lines);
        updates
            .iter()
            .map(|update| {
                if validate_update(&rules, update) == true {
                    find_middle_page(update)
                } else {
                    0
                }
            })
            .sum()
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let (rules, updates) = split_input(lines);
        updates
            .iter()
            .map(|update| {
                if validate_update(&rules, update) == true {
                    0
                } else {
                    let new_update = correcting_update(&rules, update);
                    find_middle_page(&new_update)
                }
            })
            .sum()
    }
}

fn correcting_update(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut new_update = update.clone();
    new_update.sort_by(|a, b| {
        if let Some(after_list) = rules.get(a) {
            if after_list.contains(b) == true {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Greater
        }
    });

    new_update
}

fn find_middle_page(update: &Vec<i32>) -> i32 {
    let middle_index = update.len() / 2;
    update[middle_index]
}

fn validate_update(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    let mut printed_pages: Vec<i32> = vec![];
    update.iter().all(|page_number| {
        if let Some(after_list) = rules.get(page_number) {
            if after_list
                .iter()
                .any(|after_page| printed_pages.contains(after_page))
                == true
            {
                return false;
            } else {
                printed_pages.push(*page_number);
            }
        } else {
            printed_pages.push(*page_number);
        }

        true
    })
}

fn split_input(lines: &Vec<&str>) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = vec![];
    let mut end_of_rules = false;
    lines.iter().for_each(|text| {
        if end_of_rules == false {
            if text.len() == 0 {
                end_of_rules = true;
            } else {
                let values: Vec<&str> = text.split("|").collect();
                let key = values[0].parse::<i32>().unwrap();
                let page_number = values[1].parse::<i32>().unwrap();

                if let Some(list) = rules.get_mut(&key) {
                    list.push(page_number);
                } else {
                    rules.insert(key, vec![page_number]);
                }
            }
        } else {
            let values: Vec<&str> = text.split(",").collect();
            let update_list = values
                .iter()
                .map(|text| text.parse::<i32>().unwrap())
                .collect();

            updates.push(update_list);
        }
    });

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correcting_order() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (rules, updates) = split_input(&lines);

        let result = correcting_update(&rules, &vec![75, 97, 47, 61, 53]);
        assert_eq!(result, vec![97, 75, 47, 61, 53]);

        let result = correcting_update(&rules, &vec![61, 13, 29]);
        assert_eq!(result, vec![61, 29, 13]);

        let result = correcting_update(&rules, &vec![97, 13, 75, 29, 47]);
        assert_eq!(result, vec![97, 75, 47, 29, 13])
    }

    #[test]
    fn test_validate_rule() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (rules, updates) = split_input(&lines);
        let result = validate_update(&rules, &updates[0]);
        assert!(result);

        let result = validate_update(&rules, &updates[1]);
        assert!(result);

        let result = validate_update(&rules, &updates[2]);
        assert!(result);

        let result = validate_update(&rules, &updates[3]);
        assert!(result == false);

        let result = validate_update(&rules, &updates[4]);
        assert!(result == false);

        let result = validate_update(&rules, &updates[5]);
        assert!(result == false);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day05::part_01(&lines);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day05::part_02(&lines);
        assert_eq!(result, 123);
    }
}

const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
