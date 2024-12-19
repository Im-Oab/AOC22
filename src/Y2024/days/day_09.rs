use crate::file_handler::FileHandler;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day09 {}

impl Day09 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2024/inputs/day_09_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day09::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day09::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_09".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> u64 {
        let data = parse_input(&lines);
        let processed_data = process_data(&data);
        let check_sum = check_sum(&processed_data);

        check_sum
    }

    fn part_02(lines: &Vec<&str>) -> u64 {
        let (data, data_map, space_map) = parse_input_for_mapping_data(&lines);
        let processed_data = process_data_with_mapping_data(&data, data_map, space_map);
        let check_sum = check_sum(&processed_data);

        check_sum
    }
}

const TEST_INPUT: &str = "2333133121414131402";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day09::part_01(&lines);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day09::part_02(&lines);
        assert_eq!(result, 2858);
    }

    #[test]
    fn test_parse_input() {
        let lines = vec!["12345"];
        let data = parse_input(&lines);
        assert_eq!("0..111....22222", &data_to_string(&data));

        let lines = vec!["2333133121414131402"];
        let data = parse_input(&lines);
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899",
            &data_to_string(&data)
        );
    }

    #[test]
    fn test_process_data_1() {
        let lines = vec!["12345"];
        let data = parse_input(&lines);
        let result = process_data(&data);
        assert_eq!("022111222......", &data_to_string(&result));
    }

    #[test]
    fn test_process_data_2() {
        let lines = vec!["2333133121414131402"];
        let data = parse_input(&lines);
        let result = process_data(&data);
        assert_eq!(
            "0099811188827773336446555566..............",
            &data_to_string(&result)
        );
    }

    #[test]
    fn test_check_sum() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let data = parse_input(&lines);
        let processed_data = process_data(&data);
        let check_sum = check_sum(&processed_data);
        assert_eq!(1928, check_sum)
    }

    #[test]
    fn test_parse_input_for_mapping_data() {
        let lines = vec!["12345"];
        let (data, data_map, space_map) = parse_input_for_mapping_data(&lines);
        assert_eq!("0..111....22222", &data_to_string(&data));
        assert_eq!(data_map.len(), 3);
        assert_eq!((0, 1), data_map[0]);
        assert_eq!((3, 3), data_map[1]);
        assert_eq!((10, 5), data_map[2]);

        assert_eq!(space_map.len(), 2);
        assert_eq!((1, 2), space_map[0]);
        assert_eq!((6, 4), space_map[1]);
    }

    #[test]
    fn test_process_data_with_mapping_data() {
        let lines = vec!["2333133121414131402"];
        let (data, data_map, space_map) = parse_input_for_mapping_data(&lines);
        let result = process_data_with_mapping_data(&data, data_map, space_map);
        assert_eq!(
            "00992111777.44.333....5555.6666.....8888..",
            &data_to_string(&result)
        );
    }
}

enum DataOperators {
    None,
    Remove(usize),
    // move index by total_blocks
    Update(usize, usize),
}

fn process_data_with_mapping_data(
    data: &Vec<DataBlock>,
    mut data_map: Vec<(usize, usize)>,
    mut space_map: Vec<(usize, usize)>,
) -> Vec<DataBlock> {
    let mut result = data.clone();
    let mut current_data_index = data_map.len().saturating_sub(1);

    loop {
        if let Some((data_index, total_data_blocks)) = data_map.get(current_data_index) {
            let mut operator = DataOperators::None;
            // look for free space
            space_map.iter().enumerate().all(
                |(space_map_index, (space_index, total_space_blocks))| {
                    if total_data_blocks <= total_space_blocks {
                        // copy data to the free space
                        (*space_index..(*space_index + *total_data_blocks)).for_each(
                            |target_index| {
                                if let Some(DataBlock::Data(data_number_id)) =
                                    result.get(*data_index)
                                {
                                    result[target_index] = DataBlock::Data(*data_number_id);
                                }
                            },
                        );

                        // remove data from previous locations
                        (*data_index..(*data_index + *total_data_blocks)).for_each(
                            |target_index| {
                                result[target_index] = DataBlock::Space;
                            },
                        );

                        // update space_map
                        if total_data_blocks == total_space_blocks {
                            operator = DataOperators::Remove(space_map_index);
                        } else {
                            operator = DataOperators::Update(space_map_index, *total_data_blocks);
                        }

                        // print_data(&result);

                        return false;
                    }

                    *space_index < *data_index
                },
            );

            match operator {
                DataOperators::Update(space_index, move_blocks) => {
                    if let Some((index, total_blocks)) = space_map.get(space_index) {
                        space_map[space_index] = (
                            *index + move_blocks,
                            total_blocks.saturating_sub(move_blocks),
                        );
                    }
                }
                DataOperators::Remove(index) => {
                    space_map.remove(index);
                }
                DataOperators::None => {}
            };
        } else {
            break;
        }
        // print_data(&result);
        current_data_index = match current_data_index.checked_sub(1) {
            Some(v) => v,
            None => break,
        };
    }

    result
}

fn parse_input_for_mapping_data(
    lines: &Vec<&str>,
) -> (Vec<DataBlock>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut result = vec![];
    let raw_data = lines.first().unwrap().to_string();

    let mut data_map = vec![];
    let mut free_space_map = vec![];
    let mut data_number_id = 0;
    raw_data.chars().enumerate().for_each(|(index, c)| {
        let total_blocks = c.to_string().parse::<usize>().unwrap();
        // data
        if index % 2 == 0 {
            data_map.push((result.len(), total_blocks));
            (0..total_blocks).for_each(|_| {
                result.push(DataBlock::Data(data_number_id));
            });

            data_number_id += 1;
        }
        // space
        else {
            free_space_map.push((result.len(), total_blocks));

            (0..total_blocks).for_each(|_| {
                result.push(DataBlock::Space);
            });
        }
    });

    (result, data_map, free_space_map)
}

fn check_sum(data: &Vec<DataBlock>) -> u64 {
    data.iter()
        .enumerate()
        .map(|(index, block)| match block {
            DataBlock::Data(data_number_id) => *data_number_id as u64 * index as u64,
            DataBlock::Space => 0u64,
        })
        .sum::<u64>()
}

fn process_data(data: &Vec<DataBlock>) -> Vec<DataBlock> {
    let mut result = data.clone();
    let mut last_index = data.len().saturating_sub(1);

    data.iter().enumerate().all(|(index, block)| {
        match block {
            DataBlock::Data(_) => {}
            DataBlock::Space => loop {
                if last_index <= index {
                    break;
                }

                if let Some(DataBlock::Data(data_number_id)) = data.get(last_index) {
                    result[index] = DataBlock::Data(*data_number_id);
                    result[last_index] = DataBlock::Space;
                    last_index = last_index.saturating_sub(1);

                    break;
                } else {
                    last_index = last_index.saturating_sub(1);
                }
            },
        }
        // print_data(&result);

        index < last_index
    });

    result
}

#[derive(Debug, Clone)]
enum DataBlock {
    Space,
    Data(i32),
}

fn print_data(data: &Vec<DataBlock>) {
    println!("{}", data_to_string(data));
}

fn data_to_string(data: &Vec<DataBlock>) -> String {
    let mut result = String::new();
    data.iter().for_each(|block| match block {
        DataBlock::Data(data_number_id) => {
            result = format!("{}{}", result, data_number_id);
        }
        DataBlock::Space => {
            result = format!("{}.", result);
        }
    });

    result
}

fn parse_input(lines: &Vec<&str>) -> Vec<DataBlock> {
    let mut result = vec![];
    let raw_data = lines.first().unwrap().to_string();

    let mut data_number_id = 0;
    raw_data.chars().enumerate().for_each(|(index, c)| {
        let total_blocks = c.to_string().parse::<usize>().unwrap();
        // data
        if index % 2 == 0 {
            (0..total_blocks).for_each(|_| {
                result.push(DataBlock::Data(data_number_id));
            });

            data_number_id += 1;
        }
        // space
        else {
            (0..total_blocks).for_each(|_| {
                result.push(DataBlock::Space);
            });
        }
    });

    result
}
