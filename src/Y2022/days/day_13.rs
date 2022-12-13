use std::{collections::VecDeque, time::Instant};

use hashbrown::HashMap;
use itertools::Itertools;

use crate::file_handler::FileHandler;

pub struct Day13 {}

impl Day13 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_13_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day13::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day13::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_13".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// Part 1, I took a very long time to implement parsing and support nesting of the list. 
    /// Also, I still get confused with the detail of the puzzle again.
    fn part_01(lines: &Vec<&str>) -> usize {
        let packets = parsing(lines);
        let packets = packets
            .iter()
            .map(|p| converting_raw_data(p.clone()))
            .collect_vec();

        return find_total_right_orders(&packets).1;
    }

    /// Part 2, After reading the detail, I learned how to implement it when someone guided me that sorting is the key. 
    /// I used the function that compares two packets as a condition in sorting.
    fn part_02(lines: &Vec<&str>) -> usize {
        let mut packets = parsing(lines);
        packets.push(VecDeque::from([
            "[".to_owned(),
            "[".to_owned(),
            "2".to_owned(),
            "]".to_owned(),
            "]".to_owned(),
        ]));
        packets.push(VecDeque::from([
            "[".to_owned(),
            "[".to_owned(),
            "6".to_owned(),
            "]".to_owned(),
            "]".to_owned(),
        ]));
        let mut packets = packets
            .iter()
            .map(|p| converting_raw_data(p.clone()))
            .collect_vec();

        packets.sort_by(|a, b| match compare_data(a, b) {
            CompareDataResult::TRUE => std::cmp::Ordering::Less,
            CompareDataResult::FALSE => std::cmp::Ordering::Greater,
            CompareDataResult::SKIP => std::cmp::Ordering::Equal,
        });

        return find_decode_key(&packets);
    }
}

const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day13::part_01(&lines);
    assert_eq!(result, 13);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day13::part_02(&lines);
    assert_eq!(result, 140);
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Data {
    Number(i32),
    List(VecDeque<Data>),
}

#[derive(Debug, Clone)]
enum CompareDataResult {
    SKIP = 0,
    TRUE = 1,
    FALSE = -1,
}

fn find_decode_key(packets: &Vec<Data>) -> usize {
    let mut result = vec![];
    for (index, data) in packets.iter().enumerate() {
        if let Data::List(list_1) = data {
            if list_1.len() == 1 {
                if let Data::List(list_2) = list_1.front().unwrap() {
                    if list_2.len() == 1 {
                        if let Data::Number(value) = list_2.front().unwrap() {
                            if *value == 2 || *value == 6 {
                                result.push(index + 1);
                            }
                        }
                    }
                }
            }
        }
    }

    result.iter().product()
}

fn find_total_right_orders(packets: &Vec<Data>) -> (usize, usize) {
    let mut total_right_order = 0;
    let mut sum_of_indices = 0;
    for index in (0..packets.len()).step_by(2) {
        let result = compare_data(&packets[index], &packets[index + 1]);
        if matches!(result, CompareDataResult::TRUE) {
            total_right_order += 1;
            sum_of_indices += index / 2 + 1;
        }
    }

    (total_right_order, sum_of_indices)
}

fn compare_data(first: &Data, second: &Data) -> CompareDataResult {
    if matches!(first, Data::List(_)) && matches!(second, Data::List(_)) {
        if let Data::List(mut first_list) = first.clone() {
            if let Data::List(mut second_list) = second.clone() {
                loop {
                    if first_list.len() == 0 && second_list.len() > 0 {
                        // left ran out of items

                        return CompareDataResult::TRUE;
                    } else if first_list.len() > 0 && second_list.len() == 0 {
                        // right ran out of items

                        return CompareDataResult::FALSE;
                    } else if first_list.len() == 0 && second_list.len() == 0 {
                        // both ran out of item

                        return CompareDataResult::SKIP;
                    } else {
                        let first_value = first_list.pop_front().unwrap();
                        let second_value = second_list.pop_front().unwrap();
                        // both are number
                        if let Some(result) = compare_number(&first_value, &second_value) {
                            if matches!(result, CompareDataResult::SKIP) {
                                continue;
                            }

                            return result;
                        } else if matches!(first_value, Data::List(_))
                            && matches!(second_value, Data::List(_))
                        {
                            // both are list. So, recursive
                            let result = compare_data(&first_value, &second_value);
                            if matches!(result, CompareDataResult::SKIP) {
                                continue;
                            }

                            return result;
                        } else {
                            // first value need to put it as a list
                            if matches!(first_value, Data::Number(_)) {
                                let first_value = convert_number_to_list(&first_value);
                                first_list.push_front(first_value);
                                second_list.push_front(second_value);
                            } else {
                                let second_value = convert_number_to_list(&second_value);
                                first_list.push_front(first_value);
                                second_list.push_front(second_value);
                            }
                        }
                    }
                }
            }
        }
    }

    panic!("It should not be here");
}

fn convert_number_to_list(data: &Data) -> Data {
    if let Data::Number(value) = data {
        let mut new_list: VecDeque<Data> = VecDeque::new();
        new_list.push_back(Data::Number(*value));
        // new_list.push_back(Data::End);
        return Data::List(new_list);
    }

    panic!("convert_number_to_list():: data has to be number");
}

fn compare_number(first: &Data, second: &Data) -> Option<CompareDataResult> {
    if let Data::Number(first_value) = first {
        if let Data::Number(second_value) = second {
            if *first_value < *second_value {
                return Some(CompareDataResult::TRUE);
            } else if *first_value > *second_value {
                return Some(CompareDataResult::FALSE);
            } else {
                return Some(CompareDataResult::SKIP);
            }
        }
    }

    return None;
    // panic!("compare_number():: Both data has to be number");
}

fn converting_raw_data(packet: VecDeque<String>) -> Data {
    let mut packet = packet.clone();
    remove_outer_bracket(&mut packet);

    let mut list = VecDeque::new();
    while packet.len() > 0 {
        let first = packet.pop_front().unwrap();
        let data = match first.parse::<i32>() {
            Ok(v) => Data::Number(v),
            Err(_) => {
                if first == "[" {
                    let closed_index = find_closed_list_index(&packet, 0);

                    let mut new_list = packet.drain(..=closed_index).collect::<VecDeque<String>>();
                    new_list.push_front("[".to_owned());

                    converting_raw_data(new_list)
                } else {
                    continue;
                }
            }
        };

        list.push_back(data);
    }
    Data::List(list)
}

fn remove_outer_bracket(packet: &mut VecDeque<String>) {
    packet.pop_back();
    packet.pop_front();
}

fn find_closed_list_index(packet: &VecDeque<String>, start_index: usize) -> usize {
    let mut open = 0;
    for (index, value) in packet.iter().enumerate() {
        if index >= start_index {
            if value == "[" {
                open += 1;
            } else if value == "]" {
                if open > 0 {
                    open -= 1;
                } else {
                    return index;
                }
            }
        }
    }

    panic!("find_group_range: something wrong");
}

fn parsing(lines: &Vec<&str>) -> Vec<VecDeque<String>> {
    let mut result = vec![];
    for line in lines.iter() {
        if line.len() > 0 {
            let splited: Vec<char> = line.chars().map(|c| c.to_owned()).collect_vec();
            let mut packet = VecDeque::new();
            let mut temp = String::new();
            for c in splited.iter() {
                if *c == '[' {
                    packet.push_back(c.to_string());
                } else if *c == ',' || *c == ']' {
                    if temp.len() > 0 {
                        packet.push_back(temp.to_owned());
                        temp.clear();
                    }

                    if *c == ']' {
                        packet.push_back(c.to_string());
                    }
                } else {
                    temp.push_str(c.to_string().as_str());
                }
            }

            result.push(packet)
        }
    }

    result
}
