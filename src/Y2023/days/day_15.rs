use crate::file_handler::FileHandler;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day15 {}

impl Day15 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_15_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day15::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day15::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_15".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i32 {
        let data = parse_input(lines);

        data.iter().map(|list| process_hash(list)).sum()
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let data = do_lense_boxing(lines[0]);
        calculate_box_scores(&data)
    }
}

fn do_lense_boxing(input: &str) -> HashMap<i32, (Vec<String>, HashMap<String, i32>)> {
    let mut boxes: HashMap<i32, (Vec<String>, HashMap<String, i32>)> = HashMap::new();
    let data = parse_input_with_commands(input);
    data.iter()
        .for_each(|(operation, label, label_value, focal_length)| {
            let box_number = process_hash(label_value);
            // =
            if *operation > 0 {
                // box exist
                if let Some((slots, lens)) = boxes.get_mut(&box_number) {
                    // lens exist then replace with new focal_length
                    if slots.contains(label) == true {
                        lens.insert(label.to_owned(), *focal_length);
                    }
                    // new lens
                    else {
                        slots.push(label.to_owned());
                        lens.insert(label.to_owned(), *focal_length);
                    }
                }
                // new box
                else {
                    let slots = vec![label.to_owned()];
                    let mut lens = HashMap::new();
                    lens.insert(label.to_owned(), *focal_length);
                    boxes.insert(box_number, (slots, lens));
                }
            } else if *operation < 0 {
                if let Some((slots, lens)) = boxes.get_mut(&box_number) {
                    if let Some(remove_index) = slots.iter().position(|v| v == label) {
                        slots.remove(remove_index);
                        lens.remove(label);
                    }
                }
            }
        });

    boxes
}

fn calculate_box_scores(boxes: &HashMap<i32, (Vec<String>, HashMap<String, i32>)>) -> i32 {
    boxes
        .par_iter()
        .map(|(box_number, (slots, lens))| {
            let sum: i32 = slots
                .iter()
                .enumerate()
                .map(|(index, label)| {
                    if let Some(focal_length) = lens.get(label) {
                        (*box_number + 1) * (index + 1) as i32 * (*focal_length)
                    } else {
                        unreachable!("It should not be here");
                    }
                })
                .sum();

            sum
        })
        .sum()
}

fn parse_input(input: &Vec<&str>) -> Vec<Vec<i32>> {
    input[0]
        .split(",")
        .map(|input| parse_string(input))
        .collect()
}

fn parse_input_with_commands(input: &str) -> Vec<(i32, String, Vec<i32>, i32)> {
    input
        .split(",")
        .map(|value| {
            if value.contains("=") {
                let sub_values: Vec<&str> = value.split("=").collect();
                let label = sub_values[0].to_owned();
                let label_values = parse_string(sub_values[0]);
                let focal_length = sub_values[1].parse::<i32>().unwrap();
                (1, label, label_values, focal_length)
            } else if value.contains("-") {
                let sub_values: Vec<&str> = value.split("-").collect();
                let label = sub_values[0].to_owned();
                let label_values = parse_string(sub_values[0]);
                (-1, label, label_values, 0)
            } else {
                unreachable!("It should not be here: {}", value);
            }
        })
        .collect()
}

fn parse_string(input: &str) -> Vec<i32> {
    input.chars().map(|c| c as i32).collect()
}

fn process_hash(list: &Vec<i32>) -> i32 {
    let mut current_value = 0;
    list.iter().for_each(|value| {
        current_value = calculate_hash_score(*value, current_value);
    });

    current_value
}

fn calculate_hash_score(input: i32, start_value: i32) -> i32 {
    let mut current_value = start_value + input;
    current_value *= 17;
    current_value % 256
}

const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_with_commands() {
        let (operation, label, label_value, focal_length) =
            parse_input_with_commands("rn=1")[0].clone();
        assert_eq!(operation, 1);
        assert_eq!(label, "rn".to_owned());
        assert_eq!(label_value, vec![114, 110]);
        assert_eq!(focal_length, 1);
    }

    #[test]
    fn test_hash() {
        let lines: Vec<&str> = "HASH".lines().collect();
        let result = Day15::part_01(&lines);
        assert_eq!(result, 52);

        let lines: Vec<&str> = "rn".lines().collect();
        let data = parse_input(&lines);
        let result = process_hash(&data[0]);
        assert_eq!(result, 0);

        let lines: Vec<&str> = "qp".lines().collect();
        let data = parse_input(&lines);
        let result = process_hash(&data[0]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_process() {
        let result = calculate_hash_score(72, 0);
        assert_eq!(result, 200);
    }

    #[test]
    fn test_parse_string_to_ascii() {
        let result = parse_string("HASH");
        assert_eq!(result, vec![72, 65, 83, 72])
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day15::part_01(&lines);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day15::part_02(&lines);
        assert_eq!(result, 145);
    }
}
