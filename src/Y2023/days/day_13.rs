use crate::file_handler::FileHandler;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day13 {}

impl Day13 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_13_1.txt");

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

    fn part_01(lines: &Vec<&str>) -> u128 {
        let data = parse_input(lines);
        let patterns: Vec<Pattern> = data
            .par_iter()
            .map(|group| Pattern::new(group.clone()))
            .collect();

        patterns.iter().map(|node| node.process()).sum()
    }

    fn part_02(lines: &Vec<&str>) -> u128 {
        let data = parse_input(lines);
        let mut patterns: Vec<Pattern> = data
            .par_iter()
            .map(|group| Pattern::new(group.clone()))
            .collect();

        patterns.iter_mut().map(|node| node.process_smudge()).sum()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PatternType {
    Rock,
    Ash,
}

impl PatternType {
    fn from(value: &str) -> PatternType {
        match value {
            "." => PatternType::Ash,
            "#" => PatternType::Rock,
            _ => panic!("PatternType::incorrect input: {}", value),
        }
    }

    fn to_string(&self) -> String {
        match self {
            PatternType::Ash => ".".to_owned(),
            PatternType::Rock => "#".to_owned(),
        }
    }

    fn opposite(&self) -> PatternType {
        match self {
            PatternType::Ash => PatternType::Rock,
            PatternType::Rock => PatternType::Ash,
        }
    }
}

struct Pattern {
    /// [row][column]
    horizontal: Vec<Vec<PatternType>>,
    /// [column][row]
    vertical: Vec<Vec<PatternType>>,
}

impl Pattern {
    fn new(data: Vec<Vec<PatternType>>) -> Self {
        let width = data.first().unwrap().len();
        let vertical: Vec<Vec<PatternType>> = (0..width)
            .map(|column| {
                (0..data.len())
                    .map(|row| data[row][column].clone())
                    .collect()
            })
            .collect();

        Self {
            horizontal: data,
            vertical: vertical,
        }
    }

    fn process(&self) -> u128 {
        if let Some((column, row)) = self.find_perfect_reflection(vec![]) {
            Pattern::calculate_score_from_reflection(column, row)
        } else {
            self.print_pattern();

            panic!("Pattern::process()::Something wrong:");
        }
    }

    fn process_smudge(&mut self) -> u128 {
        let original_reflection = self.find_perfect_reflection(vec![]);
        let original_vertical_data = self.vertical.clone();
        let original_horizontal_data = self.horizontal.clone();
        let skipped = vec![original_reflection.unwrap_or((0, 0))];
        let mut index = 0;
        loop {
            let column = index % original_vertical_data.len();
            let row = index / original_vertical_data.len();

            self.invert_data(column, row);

            // if row == 8 && column == 10
            // {
            //     self.print_pattern();
            //     println!("{} {}\n--", column, row);
            // }

            let new_reflection = self.find_perfect_reflection(skipped.clone());
            if (original_reflection.is_some()
                && new_reflection.is_some()
                && new_reflection != original_reflection)
                || (original_reflection.is_none() && new_reflection.is_some())
            {
                // found it
                // self.print_pattern();
                if let Some((r_column, r_row)) = new_reflection {
                    return Pattern::calculate_score_from_reflection(r_column, r_row);
                }

                self.print_pattern();
                println!("Original: {:?}", original_reflection);
                println!("New: {:?}", new_reflection);
                panic!("It should not reach here: {} {}", column, row);
            } else {
                self.horizontal = original_horizontal_data.clone();
                self.vertical = original_vertical_data.clone();
            }

            index += 1;

            if index > original_horizontal_data.len() * original_vertical_data.len() {
                self.print_pattern();
                panic!(
                    "It should not reach here:\n{} {}\n{} {}",
                    original_horizontal_data.len(),
                    original_horizontal_data[0].len(),
                    original_vertical_data[0].len(),
                    original_vertical_data.len()
                );
            }
        }
    }

    fn invert_data(&mut self, column: usize, row: usize) {
        if row >= self.horizontal.len() || column >= self.vertical.len() {
            self.print_pattern();
            panic!(
                "Index out of bound:\nColumn: {} / {}\nRow: {} / {}\n--",
                column,
                self.vertical.len(),
                row,
                self.horizontal.len()
            );
        }
        self.horizontal[row][column] = self.horizontal[row][column].opposite();
        self.vertical[column][row] = self.vertical[column][row].opposite();
    }

    // Column, row
    fn find_perfect_reflection(&self, skipped: Vec<(u128, u128)>) -> Option<(u128, u128)> {
        let horizontal = self.find_start_reflection_horizontal();
        for start_row in horizontal.iter() {
            let perfect = self.is_perfect_horizontal_reflection(*start_row);
            if perfect == true {
                let result = (0, *start_row as u128);
                if skipped.contains(&result) == false {
                    return Some(result);
                }
            }
        }

        let vertical = self.find_start_reflection_vertical();
        for start_column in vertical.iter() {
            let perfect = self.is_perfect_vertical_reflection(*start_column);
            if perfect == true {
                let result = (*start_column as u128, 0);
                if skipped.contains(&result) == false {
                    return Some(result);
                }
            }
        }

        None
    }

    fn calculate_score_from_reflection(column: u128, row: u128) -> u128 {
        row * 100 + column
    }

    fn print_pattern(&self) {
        self.horizontal.iter().for_each(|row| {
            row.iter().for_each(|column| {
                print!("{}", column.to_string());
            });
            println!("");
        });

        println!("---\n");
    }

    fn is_perfect_horizontal_reflection(&self, start_index: usize) -> bool {
        (1..=start_index).rev().all(|index| {
            let actual_index = index - 1;
            let reflect_index = start_index + (start_index - index);
            if reflect_index >= self.horizontal.len() {
                true
            } else {
                self.horizontal[actual_index] == self.horizontal[reflect_index]
            }
        })
    }

    fn is_perfect_vertical_reflection(&self, start_index: usize) -> bool {
        (1..=start_index).rev().all(|index| {
            let actual_index = index - 1;
            let reflect_index = start_index + (start_index - index);
            if reflect_index >= self.vertical.len() {
                true
            } else {
                self.vertical[actual_index] == self.vertical[reflect_index]
            }
        })
    }

    /// row start at 1
    fn find_start_reflection_horizontal(&self) -> Vec<usize> {
        self.horizontal
            .par_windows(2)
            .enumerate()
            .filter_map(|(index, values)| {
                if values[0] == values[1] {
                    Some(index + 1)
                } else {
                    None
                }
            })
            .collect()
    }

    fn find_start_reflection_vertical(&self) -> Vec<usize> {
        self.vertical
            .par_windows(2)
            .enumerate()
            .filter_map(|(index, values)| {
                if values[0] == values[1] {
                    Some(index + 1)
                } else {
                    None
                }
            })
            .collect()
    }

    fn find_smudge_horizontal(&self) {
        let difference: Vec<(usize, Vec<usize>)> = self
            .horizontal
            .par_windows(2)
            .enumerate()
            .filter_map(|(index, values)| {
                let result: Vec<usize> = values[0]
                    .iter()
                    .zip(values[1].iter())
                    .enumerate()
                    .filter_map(|(column_index, (data_1, data_2))| {
                        if data_1 == data_2 {
                            None
                        } else {
                            Some(column_index)
                        }
                    })
                    .collect();

                if result.len() == 1 {
                    Some((index, result))
                } else {
                    None
                }
            })
            .collect();

        difference.iter().for_each(|v| {
            println!("{} :: {:?}", v.0, v.1);
        });
    }
}

fn parse_input(input: &Vec<&str>) -> Vec<Vec<Vec<PatternType>>> {
    let mut patterns: Vec<Vec<Vec<PatternType>>> = vec![];
    let mut group = vec![];
    input.iter().for_each(|data| {
        if data.len() > 0 {
            let row: Vec<PatternType> = (*data)
                .par_chars()
                .map(|c| PatternType::from(&String::from(c)))
                .collect();
            group.push(row);
        } else {
            patterns.push(group.clone());
            group.clear();
        }
    });

    if group.len() > 0 {
        patterns.push(group.clone());
        group.clear();
    }

    patterns
}

const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

const SPECIFIC_TEST: &str = "#......#....##.
####.###.#...#.
####.###.#...#.
#......#....##.
..##...#...#...
..#...#####.#..
#####.#####.###
...####...##.#.
...####...#..#.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_smudge() {
        // Issue It sill look for previous reflection after smudge instead of new one.
        let input: Vec<&str> = SPECIFIC_TEST.lines().collect();
        let data = parse_input(&input);
        let mut pattern = Pattern::new(data[0].clone());
        // let old_result = pattern.find_perfect_reflection(vec![]);
        // println!("{:?}", old_result);
        // let skipped = vec![old_result.unwrap()];
        // pattern.invert_data(10, 8);
        // let result = pattern.find_perfect_reflection(skipped);
        // println!("{:?}", result);
        let result = pattern.process_smudge();
        assert_eq!(result, 800);
    }

    #[test]
    fn test_find_smudge() {
        let input: Vec<&str> = TEST_INPUT.lines().collect();
        let data = parse_input(&input);

        let mut pattern = Pattern::new(data[0].clone());
        let result = pattern.process_smudge();
        assert_eq!(result, 300);

        let mut pattern = Pattern::new(data[1].clone());
        let result = pattern.process_smudge();
        assert_eq!(result, 100);
    }
    #[test]
    fn test_perfect_horizontal_reflection() {
        let input: Vec<&str> = TEST_INPUT.lines().collect();
        let data = parse_input(&input);

        let pattern = Pattern::new(data[0].clone());
        let reflection = pattern.find_start_reflection_vertical();
        assert_eq!(true, pattern.is_perfect_vertical_reflection(reflection[0]));

        let pattern = Pattern::new(data[1].clone());
        let reflection = pattern.find_start_reflection_horizontal();
        assert_eq!(
            true,
            pattern.is_perfect_horizontal_reflection(reflection[0])
        );
    }

    #[test]
    fn test_parse_input() {
        let input: Vec<&str> = TEST_INPUT.lines().collect();
        let data = parse_input(&input);
        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_find_reflection() {
        let input: Vec<&str> = TEST_INPUT.lines().collect();
        let data = parse_input(&input);

        let pattern = Pattern::new(data[0].clone());
        let reflection = pattern.find_start_reflection_vertical();
        assert_eq!(reflection.len(), 1);
        assert_eq!(reflection[0], 5);

        let pattern = Pattern::new(data[1].clone());
        let reflection = pattern.find_start_reflection_horizontal();
        assert_eq!(reflection.len(), 1);
        assert_eq!(reflection[0], 4);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day13::part_01(&lines);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();

        let result = Day13::part_02(&lines);
        assert_eq!(result, 400);
    }
}
