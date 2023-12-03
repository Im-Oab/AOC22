use std::time::Instant;

use std::collections::HashMap;

use hashbrown::HashSet;

use crate::file_handler::FileHandler;

pub struct Day03 {}

impl Day03 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_03_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day03::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day03::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_03".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    //only 12 red cubes, 13 green cubes, and 14 blue cubes?
    fn part_01(lines: &Vec<&str>) -> i32 {
        let (grids, numbers) = parsing_input(lines);

        let mut total_sum = 0;
        for (value, coordinates) in numbers.iter() {
            let mut found = false;
            'next_value: for (column, row) in coordinates.iter() {
                let neighbours = get_neighbour(*column, *row);
                for coord in neighbours.iter() {
                    if let Some(schm_type) = grids.get(coord) {
                        if matches!(schm_type, ScehmaticType::Symbol(_)) == true {
                            // found symbol
                            total_sum += value;
                            found = true;
                            break 'next_value;
                        }
                    }
                }
            }

            if found == false {
                println!(
                    "value not use: [{}] coordinates: \n{:?}\n",
                    value, coordinates
                );
            }
        }

        total_sum
    }

    fn part_02(lines: &Vec<&str>) -> u128 {
        let (grids, numbers) = parsing_input(lines);

        let mut reverse_numbers: HashMap<(i32, i32), i32> = HashMap::new();
        for (value, coordinates) in numbers.iter() {
            for coord in coordinates.iter() {
                if let Some(value) = reverse_numbers.get_mut(coord) {
                } else {
                    reverse_numbers.insert(coord.clone(), *value);
                }
            }
        }

        let mut total_sum_gear_ratio: u128 = 0;
        for ((column, row), schm_type) in grids.iter() {
            if let ScehmaticType::Symbol(symbol) = schm_type {
                if symbol == "*" {
                    /// Warning, This has a bug when gears has same value.
                    let mut gear_values = HashSet::new();
                    let neighbour = get_neighbour(*column, *row);
                    for coord in neighbour.iter() {
                        if let Some(value) = reverse_numbers.get(coord) {
                            gear_values.insert(*value);
                        }
                    }

                    if gear_values.len() == 2 {
                        let mut gear_ratio = 1;
                        for value in gear_values.iter() {
                            gear_ratio *= value;
                        }

                        total_sum_gear_ratio += gear_ratio as u128;
                    }
                }
            }
        }

        total_sum_gear_ratio
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ScehmaticType {
    Digit(i32),
    Symbol(String),
    None,
}

fn parsing_input(
    lines: &Vec<&str>,
) -> (
    HashMap<(i32, i32), ScehmaticType>,
    HashMap<i32, Vec<(i32, i32)>>,
) {
    let mut grids = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        // grids
        for (column, c) in line.chars().enumerate() {
            if let Ok(digit) = String::from(c).parse::<i32>() {
                grids.insert((column as i32, row as i32), ScehmaticType::Digit(digit));
            } else {
                match c {
                    '.' => grids.insert((column as i32, row as i32), ScehmaticType::None),
                    _ => grids.insert(
                        (column as i32, row as i32),
                        ScehmaticType::Symbol(String::from(c)),
                    ),
                };
            }
        }
    }

    let numbers = get_numbers_ex(lines);

    (grids, numbers)
}

fn get_numbers(line: &str, row: i32) -> HashMap<(i32, i32, i32), Vec<(i32, i32)>> {
    let mut result = HashMap::new();
    let mut content = String::new();
    let mut start_index = 0;

    for (index, c) in line.chars().enumerate() {
        if let Ok(_) = String::from(c).parse::<i32>() {
            if content.len() == 0 {
                start_index = index;
            }

            content.push_str(String::from(c).as_str());
        } else {
            // Has value
            if content.len() > 0 {
                if let Ok(value) = content.parse::<i32>() {
                    let mut list = vec![];
                    for column in start_index..index {
                        list.push((column as i32, row));
                    }

                    result.insert((start_index as i32, row, value), list);

                    content.clear();
                }
            }
        }
    }

    // Has value
    if content.len() > 0 {
        if let Ok(value) = content.parse::<i32>() {
            let mut list = vec![];
            for column in start_index..line.len() {
                list.push((column as i32, row));
            }

            result.insert((start_index as i32, row, value), list);

            content.clear();
        }
    }
    result
}

fn get_numbers_ex(lines: &Vec<&str>) -> HashMap<i32, Vec<(i32, i32)>> {
    let mut result: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut content = String::new();
    let mut start_index = 0;

    for (row, line) in lines.iter().enumerate() {
        for (index, c) in line.chars().enumerate() {
            if let Ok(_) = String::from(c).parse::<i32>() {
                if content.len() == 0 {
                    start_index = index;
                }

                content.push_str(String::from(c).as_str());
            } else {
                // Has value
                if content.len() > 0 {
                    if let Ok(value) = content.parse::<i32>() {
                        let mut list = vec![];
                        for column in start_index..index {
                            list.push((column as i32, row as i32));
                        }

                        if let Some(current_list) = result.get_mut(&value) {
                            current_list.append(&mut list);
                        } else {
                            result.insert(value, list);
                        }

                        content.clear();
                    }
                }
            }
        }

        // Has value
        if content.len() > 0 {
            if let Ok(value) = content.parse::<i32>() {
                let mut list = vec![];
                for column in start_index..line.len() {
                    list.push((column as i32, row as i32));
                }

                if let Some(current_list) = result.get_mut(&value) {
                    current_list.append(&mut list);
                } else {
                    result.insert(value, list);
                }

                content.clear();
            }
        }
    }
    result
}

fn get_neighbour(column: i32, row: i32) -> Vec<(i32, i32)> {
    let mut result = vec![];
    result.push((column - 1, row - 1));
    result.push((column, row - 1));
    result.push((column + 1, row - 1));

    result.push((column - 1, row));
    result.push((column + 1, row));

    result.push((column - 1, row + 1));
    result.push((column, row + 1));
    result.push((column + 1, row + 1));

    result
}

const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
..........
.....114..
.....114..
.....114..";

const TEST_INPUT_2: &str = "";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number() {
        let mut result = get_numbers_ex(&vec!["467..114..", "...467..114.5"]);
        assert_eq!(result.len(), 3);

        println!("{:?}", result);
    }

    #[test]
    fn test_parsing() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (grids, numbers) = parsing_input(&lines);
    }
    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day03::part_01(&lines);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day03::part_02(&lines);
        assert_eq!(result, 467835);
    }
}
