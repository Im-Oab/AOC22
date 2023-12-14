use crate::file_handler::FileHandler;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day12 {}

impl Day12 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_12_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day12::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day12::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_12".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i32 {
        lines
            .par_iter()
            .map(|input| {
                let (conditions, hints) = parse_line(input);

                let value = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());

                value
            })
            .sum()
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        lines
            .par_iter()
            .map(|input| {
                let (conditions, hints) = parse_line(input);
                let (conditions, hints) = unfold(&conditions, &hints);

                let value = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());

                value
            })
            .sum()
    }
}

fn unfold(conditions: &Vec<ConditionType>, hints: &Vec<usize>) -> (Vec<ConditionType>, Vec<usize>) {
    let mut new_conditions = vec![];
    (0..5).for_each(|index| {
        if index > 0 {
            new_conditions.append(&mut vec![ConditionType::Unknown]);
        }

        new_conditions.append(&mut conditions.clone());
    });

    let mut new_hints = vec![];
    (0..5).for_each(|_| new_hints.append(&mut hints.clone()));

    (new_conditions, new_hints)
}

fn unfold_text(input: &str) {}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConditionType {
    Operational,
    Damaged,
    Unknown,
}

impl ConditionType {
    fn from(input: &str) -> ConditionType {
        match input {
            "." => ConditionType::Operational,
            "#" => ConditionType::Damaged,
            "?" => ConditionType::Unknown,
            _ => panic!("ConditionType::Incorrect data: {}", input),
        }
    }

    fn to_string(&self) -> String {
        match self {
            ConditionType::Damaged => "#".to_owned(),
            ConditionType::Operational => ".".to_owned(),
            ConditionType::Unknown => "?".to_owned(),
        }
    }
}

const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_minimum_length() {
        let input = parse_hint("1,1,3");
        assert_eq!(get_minimum_length_of_conditions(&input, 0), 7);

        let input = parse_hint("1");
        assert_eq!(get_minimum_length_of_conditions(&input, 0), 1);

        let input = parse_hint("1,3");
        assert_eq!(get_minimum_length_of_conditions(&input, 0), 5);

        let input = parse_hint("1,3,1,6");
        assert_eq!(get_minimum_length_of_conditions(&input, 0), 14);
    }

    #[test]
    fn test_look_for_filling_index_ex() {
        let (conditions, _) = parse_line("???.### 1,1,3");
        let result = look_for_filling_index_ex(1, &conditions, 0, conditions.len());
        assert_eq!(result, Some(0));

        let result = look_for_filling_index_ex(1, &conditions, 1, conditions.len());
        assert_eq!(result, Some(1));

        let result = look_for_filling_index_ex(1, &conditions, 2, conditions.len());
        assert_eq!(result, Some(2));

        let result = look_for_filling_index_ex(1, &conditions, 3, conditions.len());
        assert_eq!(result, None);

        let (conditions, _) = parse_line(".??..??...?##. 1,1,3");
        let result = look_for_filling_index_ex(1, &conditions, 0, conditions.len());
        assert_eq!(result, Some(1));

        let result = look_for_filling_index_ex(1, &conditions, 1, conditions.len());
        assert_eq!(result, Some(1));

        let result = look_for_filling_index_ex(1, &conditions, 2, conditions.len());
        assert_eq!(result, Some(2));

        let result = look_for_filling_index_ex(1, &conditions, 3, conditions.len());
        assert_eq!(result, Some(5));

        let (conditions, _) = parse_line("?###???????? 3,2,1");
        let result = look_for_filling_index_ex(2, &conditions, 0, conditions.len());
        assert_eq!(result, Some(5));

        let result = look_for_filling_index_ex(2, &conditions, 6, conditions.len());
        assert_eq!(result, Some(6));

        let result = look_for_filling_index_ex(2, &conditions, 7, conditions.len());
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_filling_example() {
        let (conditions, hints) = parse_line("???.### 1,1,3");
        let result = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());
        assert_eq!(result, 1);

        let (conditions, hints) = parse_line(".??..??...?##. 1,1,3");
        let result = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());
        assert_eq!(result, 4);

        let (conditions, hints) = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        let result = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());
        assert_eq!(result, 1);

        let (conditions, hints) = parse_line("????.#...#... 4,1,1");
        let result = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());
        assert_eq!(result, 1);

        let (conditions, hints) = parse_line("????.######..#####. 1,6,5");
        let result = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());
        assert_eq!(result, 4);

        let (conditions, hints) = parse_line("?###???????? 3,2,1");
        let result = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part_2_example() {
        let (conditions, hints) =
            parse_line("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3");
        let result = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());
        assert_eq!(result, 1);
    }
    #[test]
    fn test_filling() {
        // This example has issue.
        let (conditions, hints) = parse_line("?##?##??#?.#?#?? 8,3");
        let result = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());
        assert_eq!(result, 1);

        //
        let (conditions, hints) = parse_line("??#?.#??.?? 2,1,1");
        let result = filling_ex(conditions.clone(), &hints, 0, 0, conditions.len());
        assert_eq!(result, 6);
    }
    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day12::part_01(&lines);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day12::part_02(&lines);
        assert_eq!(result, 525152);
    }
}

fn filling_ex(
    conditions: Vec<ConditionType>,
    reference_hints: &Vec<usize>,
    hints_index: usize,
    left_border: usize,
    right_border: usize,
) -> i32 {
    let minimum_length = get_minimum_length_of_conditions(reference_hints, hints_index);
    let length = right_border - left_border;
    if minimum_length > length {
        // println!("Rejected: {:?}\n{:?}\n--", conditions, hints);
        return 0;
    }

    if follow_conditions(&conditions, reference_hints, hints_index, left_border) == false {
        return 0;
    }

    // handle predicted all damaged parts.
    if hints_index >= reference_hints.len() {
        let total_damaged_parts: i32 = conditions[left_border..right_border]
            .par_iter()
            .map(|v| {
                if matches!(v, ConditionType::Damaged) {
                    1
                } else {
                    0
                }
            })
            .sum();

        // It should not have any damage part in the conditions. to assume that it's correct record
        if total_damaged_parts == 0 {
            // let printed_text = print_debug(&String::new(), &conditions, 0, right_border, false);
            // if self_check_result(&printed_text, reference_hints) == false
            // {
            //     panic!("Something wrong: {}\n{:?}\n\n{:?}",printed_text, conditions, reference_hints);
            // }
            return 1;
        } else {
            return 0;
        }
    } else {
        let window_size = reference_hints[hints_index];
        // find the index to split the predicted damaged part.
        if let Some(start_index) =
            look_for_filling_index_ex(window_size, &conditions, left_border, right_border)
        {
            let end_index = start_index + window_size;
            // split until the last element of conditions
            if end_index > conditions.len() {
                // Not enough elements to match condition. This case is not possible.
                panic!(
                    "Impossible case: {} {}\n{:?}",
                    end_index,
                    conditions.len(),
                    conditions
                )
            } else {
                // Scenario: use this location as damaged part.
                let marked_conditions = conditions
                    .iter()
                    .enumerate()
                    .map(|(index, v)| {
                        if index < start_index {
                            assume_value(*v, false)
                        } else if index >= start_index && index < end_index {
                            assume_value(*v, true)
                        } else {
                            v.clone()
                        }
                    })
                    .collect();
                let result_2 = filling_ex(
                    marked_conditions,
                    reference_hints,
                    hints_index + 1,
                    end_index,
                    right_border,
                );

                // Scenario: skip this location and try next one. It should check that it possible to try next one or not.
                let unmarked_conditions = conditions
                    .iter()
                    .enumerate()
                    .map(|(index, v)| {
                        if index <= start_index {
                            assume_value(*v, false)
                        } else {
                            v.clone()
                        }
                    })
                    .collect();

                let result_1 = filling_ex(
                    unmarked_conditions,
                    reference_hints,
                    hints_index,
                    start_index + 1,
                    right_border,
                );

                return result_1 + result_2;
            }
        } else {
            // No potential damage parts in the conditions. Reject the process.
            return 0;
        }
    }

    0
}

fn self_check_result(text: &String, reference_hints: &Vec<usize>) -> bool {
    let current_data: Vec<usize> = text
        .split(".")
        .filter_map(|v| if v.len() == 0 { None } else { Some(v.len()) })
        .collect();

    current_data == *reference_hints
}

fn follow_conditions(
    conditions: &Vec<ConditionType>,
    reference_hints: &Vec<usize>,
    hints_index: usize,
    maximum_left: usize,
) -> bool {
    let mut left_border = 0;
    for index in 0..hints_index {
        if let Some(window_size) = reference_hints.get(index).copied() {
            if let Some(start_index) =
                look_for_filling_index_ex(window_size, conditions, left_border, maximum_left)
            {
                let non_damage_parts = (left_border..start_index).all(|checking_index| {
                    if let Some(value) = conditions.get(checking_index) {
                        matches!(value, ConditionType::Operational | ConditionType::Unknown)
                    } else {
                        false
                    }
                });

                if non_damage_parts == true {
                    left_border = start_index + window_size + 1;
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    let non_damage_parts = (left_border..maximum_left).all(|checking_index| {
        if let Some(value) = conditions.get(checking_index) {
            matches!(value, ConditionType::Operational | ConditionType::Unknown)
        } else {
            false
        }
    });

    non_damage_parts
}

fn assume_value(v: ConditionType, assume_damage: bool) -> ConditionType {
    if assume_damage == true {
        if matches!(v, ConditionType::Unknown) {
            return ConditionType::Damaged;
        }
    } else {
        if matches!(v, ConditionType::Unknown) {
            return ConditionType::Operational;
        }
    }

    v
}

fn print_debug(
    printed: &String,
    conditions: &Vec<ConditionType>,
    left_board: usize,
    right_border: usize,
    assume_damage: bool,
) -> String {
    let new_string: String = conditions[left_board..right_border]
        .iter()
        .map(|v| {
            let v = assume_value(*v, assume_damage);
            match v {
                ConditionType::Damaged => "#".to_owned(),
                ConditionType::Unknown => {
                    if assume_damage == true {
                        "#".to_owned()
                    } else {
                        ".".to_owned()
                    }
                }
                ConditionType::Operational => ".".to_owned(),
            }
        })
        .collect();

    let result = format!("{}{}", printed, new_string);
    println!("{}", result);
    result
}

fn look_for_filling_index_ex(
    window_size: usize,
    conditions: &Vec<ConditionType>,
    left_board: usize,
    right_border: usize,
) -> Option<usize> {
    let length = right_border - left_board;
    if window_size > length {
        None
    } else if window_size == length {
        let matched = conditions[left_board..right_border]
            .par_iter()
            .all(|v| matches!(v, ConditionType::Damaged | ConditionType::Unknown));

        if matched == true {
            // check left
            if left_board > 0 {
                if let Some(v) = conditions.get(left_board - 1) {
                    if matches!(v, ConditionType::Damaged) == true {
                        return None;
                    }
                }
            }
            // check right
            if right_border < conditions.len() {
                if let Some(v) = conditions.get(right_border) {
                    if matches!(v, ConditionType::Damaged) == true {
                        return None;
                    }
                }
            }

            return Some(left_board);
        } else {
            None
        }
    } else if window_size < length {
        let total_checking = length - window_size;
        for window_index in 0..=total_checking {
            let matched = (0..window_size).all(|index| {
                let actual_index = left_board + window_index + index;
                if let Some(value) = conditions.get(actual_index) {
                    matches!(value, ConditionType::Damaged | ConditionType::Unknown)
                } else {
                    false
                }
            });

            if matched == true {
                let actual_index = left_board + window_index;
                // check front
                if actual_index == 0 {
                    // pass
                } else {
                    let front_index = actual_index - 1;
                    if let Some(value) = conditions.get(front_index) {
                        if matches!(value, ConditionType::Damaged) {
                            // not pass
                            continue;
                        }
                    }
                }

                // check back
                if actual_index + window_size == conditions.len() {
                    return Some(actual_index);
                } else if actual_index + window_size < conditions.len() {
                    let last_condition_index = actual_index + window_size;
                    if let Some(value) = conditions.get(last_condition_index) {
                        if matches!(value, ConditionType::Operational | ConditionType::Unknown) {
                            return Some(actual_index);
                        }
                    }
                } else {
                    panic!("Something wrong");
                }
            }
        }

        None
    } else {
        None
    }
}

fn look_for_filling_index(window_size: usize, conditions: &Vec<ConditionType>) -> Option<usize> {
    if window_size > conditions.len() {
        None
    } else if window_size == conditions.len() {
        conditions
            .par_iter()
            .all(|v| matches!(v, ConditionType::Damaged | ConditionType::Unknown))
            .then_some(0)
    } else if window_size < conditions.len() {
        let total_checking = conditions.len() - window_size;
        for window_index in 0..=total_checking {
            let matched = (0..window_size).all(|index| {
                let actual_index = window_index + index;
                if let Some(value) = conditions.get(actual_index) {
                    matches!(value, ConditionType::Damaged | ConditionType::Unknown)
                } else {
                    false
                }
            });

            if matched == true {
                // check front
                if window_index == 0 {
                    // pass
                } else {
                    let front_index = window_index - 1;
                    if let Some(value) = conditions.get(front_index) {
                        if matches!(value, ConditionType::Damaged) {
                            // not pass
                            continue;
                        }
                    }
                }

                // check back
                if window_index + window_size == conditions.len() {
                    return Some(window_index);
                } else {
                    let last_condition_index = window_index + window_size;
                    if let Some(value) = conditions.get(last_condition_index) {
                        if matches!(value, ConditionType::Operational | ConditionType::Unknown) {
                            return Some(window_index);
                        }
                    }
                }
            }
        }

        None
    } else {
        None
    }
}

fn parse_line(input: &str) -> (Vec<ConditionType>, Vec<usize>) {
    let values: Vec<&str> = input.split(" ").collect_vec();
    let conditions = parse_conditions(values[0]);
    let hint = parse_hint(values[1]);

    (conditions, hint)
}

fn parse_conditions(input: &str) -> Vec<ConditionType> {
    input
        .par_chars()
        .map(|c| ConditionType::from(&String::from(c)))
        .collect()
}

fn parse_hint(input: &str) -> Vec<usize> {
    let values: Vec<&str> = input.split(",").collect();

    values
        .par_iter()
        .map(|v| (*v).parse::<usize>().unwrap())
        .collect()
}

fn get_minimum_length_of_conditions(hints: &Vec<usize>, hints_index: usize) -> usize {
    let total_hints = hints.len() - hints_index;
    hints[hints_index..]
        .par_iter()
        .enumerate()
        .map(|(index, value)| {
            if index == total_hints - 1 {
                *value
            } else {
                *value + 1
            }
        })
        .sum()
}
