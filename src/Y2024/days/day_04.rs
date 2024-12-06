use crate::file_handler::FileHandler;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day04 {}

impl Day04 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2024/inputs/day_04_1.txt");

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

    fn part_01(lines: &Vec<&str>) -> i32 {
        find_matching_keyword(lines, "XMAS") as i32
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        find_matching_keyword_cross(lines) as i32
    }
}

fn find_starting_characters(lines: &Vec<&str>, start_character: &str) -> Vec<(usize, usize)> {
    let mut result = vec![];

    lines.iter().enumerate().for_each(|(row, text)| {
        let indexes: Vec<_> = (*text).match_indices(start_character).collect();
        indexes.iter().for_each(|(column, _)| {
            result.push((row, *column));
        });
    });

    result
}

fn get_character_at_coordinate(lines: &Vec<&str>, row: usize, column: usize) -> Option<String> {
    if row < lines.len() {
        let text = lines[row].to_owned();
        if column < text.len() {
            return Some(text.chars().nth(column).unwrap().to_string());
        }
    }

    None
}

fn find_matching_keyword_cross(lines: &Vec<&str>) -> usize {
    let coordinate = find_starting_characters(lines, "A");
    let matching_words = ["MAS", "SAM"];
    let mut counter = 0;
    coordinate.iter().for_each(|(row, column)| {
        if let Some(words) = get_words_cross_reading(lines, *row, *column) {
            if words
                .iter()
                .all(|word| matching_words.iter().any(|matching| word == *matching))
                == true
            {
                counter += 1;
            }
        }
    });

    counter
}

fn get_words_cross_reading(lines: &Vec<&str>, row: usize, column: usize) -> Option<Vec<String>> {
    let mut result = vec![];
    // top-left
    if row.checked_sub(1).is_some() && column.checked_sub(1).is_some() {
        let mut word = String::new();
        (-1..=1i32).for_each(|offset| {
            if let Some(character) = get_character_at_coordinate(
                lines,
                (row as i32 + offset) as usize,
                (column as i32 + offset) as usize,
            ) {
                word.push_str(&character);
            }
        });

        if word.len() == 3 {
            result.push(word);
        }
    }
    // top-right
    if row.checked_sub(1).is_some() && column.checked_sub(1).is_some() {
        let mut word = String::new();
        (-1..=1i32).for_each(|offset| {
            if let Some(character) = get_character_at_coordinate(
                lines,
                (row as i32 + offset) as usize,
                (column as i32 - offset) as usize,
            ) {
                word.push_str(&character);
            }
        });

        if word.len() == 3 {
            result.push(word);
        }
    }

    if result.len() > 0 {
        Some(result)
    } else {
        None
    }
}

fn find_matching_keyword(lines: &Vec<&str>, keyword: &str) -> usize {
    let start_character = keyword.chars().nth(0).unwrap().to_string();
    let total_characters = keyword.len();
    let directions = [
        ReadDirection::Horizontal { backward: false },
        ReadDirection::Horizontal { backward: true },
        ReadDirection::Vertical { backward: false },
        ReadDirection::Vertical { backward: true },
        ReadDirection::DiagonalDown { left: false },
        ReadDirection::DiagonalDown { left: true },
        ReadDirection::DiagonalUp { left: false },
        ReadDirection::DiagonalUp { left: true },
    ];

    let mut counter = 0;

    let coordinate = find_starting_characters(lines, &start_character);
    coordinate.iter().for_each(|(row, column)| {
        directions.iter().for_each(|direction| {
            if let Some(word) =
                get_word_at_coordinate(lines, *row, *column, *direction, total_characters)
            {
                if &word == keyword {
                    counter += 1;
                }
            }
        });
    });

    counter
}

fn get_word_at_coordinate(
    lines: &Vec<&str>,
    row: usize,
    column: usize,
    direction: ReadDirection,
    total_characters: usize,
) -> Option<String> {
    match direction {
        ReadDirection::Horizontal { backward } => {
            if backward == false {
                let mut result = String::new();
                (column..column + total_characters).for_each(|target_column| {
                    if let Some(character) = get_character_at_coordinate(lines, row, target_column)
                    {
                        result = format!("{}{}", result, character);
                    }
                });

                if result.len() == total_characters {
                    return Some(result);
                } else {
                    None
                }
            } else {
                let mut result = String::new();
                (column.saturating_sub(total_characters.saturating_sub(1))..=column)
                    .rev()
                    .for_each(|target_column| {
                        if let Some(character) =
                            get_character_at_coordinate(lines, row, target_column)
                        {
                            result = format!("{}{}", result, character);
                        }
                    });

                if result.len() == total_characters {
                    return Some(result);
                } else {
                    None
                }
            }
        }
        ReadDirection::Vertical { backward } => {
            if backward == false {
                let mut result = String::new();
                (row..row + total_characters).for_each(|target_row| {
                    if let Some(character) = get_character_at_coordinate(lines, target_row, column)
                    {
                        result = format!("{}{}", result, character);
                    }
                });

                if result.len() == total_characters {
                    return Some(result);
                } else {
                    None
                }
            } else {
                let mut result = String::new();
                (row.saturating_sub(total_characters.saturating_sub(1))..=row)
                    .rev()
                    .for_each(|target_row| {
                        if let Some(character) =
                            get_character_at_coordinate(lines, target_row, column)
                        {
                            result = format!("{}{}", result, character);
                        }
                    });

                if result.len() == total_characters {
                    return Some(result);
                } else {
                    None
                }
            }
        }
        ReadDirection::DiagonalDown { left } => {
            if left == false {
                let mut result = String::new();
                (0..total_characters).for_each(|offset| {
                    if let Some(character) =
                        get_character_at_coordinate(lines, row + offset, column + offset)
                    {
                        result = format!("{}{}", result, character);
                    }
                });

                if result.len() == total_characters {
                    return Some(result);
                } else {
                    None
                }
            } else {
                let mut result = String::new();
                (0..total_characters).for_each(|offset| {
                    if column.checked_sub(offset).is_some() == true {
                        if let Some(character) = get_character_at_coordinate(
                            lines,
                            row + offset,
                            column.saturating_sub(offset),
                        ) {
                            result = format!("{}{}", result, character);
                        }
                    }
                });

                if result.len() == total_characters {
                    return Some(result);
                } else {
                    None
                }
            }
        }
        ReadDirection::DiagonalUp { left } => {
            if left == false {
                let mut result = String::new();
                (0..total_characters).for_each(|offset| {
                    if row.checked_sub(offset).is_some() == true {
                        if let Some(character) =
                            get_character_at_coordinate(lines, row - offset, column + offset)
                        {
                            result = format!("{}{}", result, character);
                        }
                    }
                });

                if result.len() == total_characters {
                    return Some(result);
                } else {
                    None
                }
            } else {
                let mut result = String::new();
                (0..total_characters).for_each(|offset| {
                    if row.checked_sub(offset).is_some() == true
                        && column.checked_sub(offset).is_some() == true
                    {
                        if let Some(character) =
                            get_character_at_coordinate(lines, row - offset, column - offset)
                        {
                            result = format!("{}{}", result, character);
                        }
                    }
                });

                if result.len() == total_characters {
                    return Some(result);
                } else {
                    None
                }
            }
        }
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
enum ReadDirection {
    Horizontal { backward: bool },
    Vertical { backward: bool },
    // read upward
    DiagonalUp { left: bool },
    // read downward
    DiagonalDown { left: bool },
}

const TEST_INPUT: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

const TEST_INPUT_2: &str = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";

const TEST_INPUT_3: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_word_at_coordinate_diagonal_up() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let keyword = "XMAS";
        let total_characters = keyword.len();

        let result = get_word_at_coordinate(
            &lines,
            3,
            0,
            ReadDirection::DiagonalUp { left: false },
            total_characters,
        );
        assert_eq!(&result.unwrap(), "XAA.");

        let result = get_word_at_coordinate(
            &lines,
            3,
            0,
            ReadDirection::DiagonalUp { left: true },
            total_characters,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_get_word_at_coordinate_diagonal_down() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let keyword = "XMAS";
        let total_characters = keyword.len();

        let result = get_word_at_coordinate(
            &lines,
            0,
            2,
            ReadDirection::DiagonalDown { left: false },
            total_characters,
        );
        assert_eq!(&result.unwrap(), "XMAS");

        let result = get_word_at_coordinate(
            &lines,
            0,
            2,
            ReadDirection::DiagonalDown { left: true },
            total_characters,
        );
        assert!(result.is_none());
    }
    #[test]
    fn test_get_word_at_coordinate_vertical() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let keyword = "XMAS";
        let total_characters = keyword.len();

        let result = get_word_at_coordinate(
            &lines,
            0,
            1,
            ReadDirection::Vertical { backward: false },
            total_characters,
        );
        assert_eq!(&result.unwrap(), ".SAM");

        let result = get_word_at_coordinate(
            &lines,
            0,
            1,
            ReadDirection::Vertical { backward: true },
            total_characters,
        );
        assert!(result.is_none());

        let result = get_word_at_coordinate(
            &lines,
            4,
            1,
            ReadDirection::Vertical { backward: true },
            total_characters,
        );
        assert_eq!(&result.unwrap(), "XMAS");

        let result = get_word_at_coordinate(
            &lines,
            4,
            1,
            ReadDirection::Vertical { backward: false },
            total_characters,
        );
        assert!(result.is_none());
    }
    #[test]
    fn test_get_word_at_coordinate_horizontal() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let keyword = "XMAS";
        let total_characters = keyword.len();

        let result = get_word_at_coordinate(
            &lines,
            0,
            0,
            ReadDirection::Horizontal { backward: false },
            total_characters,
        );
        assert_eq!(&result.unwrap(), "..X.");

        let result = get_word_at_coordinate(
            &lines,
            0,
            0,
            ReadDirection::Horizontal { backward: true },
            total_characters,
        );
        assert!(result.is_none());

        let result = get_word_at_coordinate(
            &lines,
            3,
            0,
            ReadDirection::Horizontal { backward: false },
            total_characters,
        );
        assert_eq!(&result.unwrap(), "XMAS");

        let result = get_word_at_coordinate(
            &lines,
            1,
            4,
            ReadDirection::Horizontal { backward: true },
            total_characters,
        );
        assert_eq!(&result.unwrap(), "XMAS");

        let result = get_word_at_coordinate(
            &lines,
            0,
            2,
            ReadDirection::Horizontal { backward: false },
            total_characters,
        );
        assert_eq!(&result.unwrap(), "X...");

        let result = get_word_at_coordinate(
            &lines,
            0,
            2,
            ReadDirection::Horizontal { backward: true },
            total_characters,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day04::part_01(&lines);
        assert_eq!(result, 4);

        let lines: Vec<&str> = TEST_INPUT_2.lines().collect();
        let result = Day04::part_01(&lines);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT_3.lines().collect();
        let result = Day04::part_02(&lines);
        assert_eq!(result, 9);
    }
}
