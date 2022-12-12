use std::time::Instant;

use hashbrown::HashSet;
use itertools::Itertools;

use crate::file_handler::FileHandler;

pub struct Day08 {}

impl Day08 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_08_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day08::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day08::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_08".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    ///  I struggled in Part 1 because I did not read the detail of the puzzle and implemented incorrect logic.
    /// Also, I feel like it will have a better solution other than brute force to get the result.
    fn part_01(lines: &Vec<&str>) -> i32 {
        let grid = self::parsing_input(lines);
        let total_visibles = self::find_visible_spots(&grid);

        return total_visibles;
    }

    /// I still use brute force to solve the part 2
    fn part_02(lines: &Vec<&str>) -> i32 {
        let grid = self::parsing_input(lines);
        let best_score = self::find_best_view_score(&grid);

        return best_score;
    }
}

fn find_best_view_score(grid: &Vec<Vec<i32>>) -> i32 {
    let mut best_score = 0;
    for row in 0..grid.len() as i32 {
        let row_data = &grid[row as usize];
        for column in 0..row_data.len() as i32 {
            let score = self::get_view_score(grid, row, column);

            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}

fn get_view_score(grid: &Vec<Vec<i32>>, row: i32, column: i32) -> i32 {
    if let Some(current_height) = self::find_value(grid, row, column) {
        if self::is_edge(grid, row, column) {
            return 0;
        }

        let mut top_count = 0;
        let mut bottom_count = 0;
        let mut left_count = 0;
        let mut right_count = 0;

        if let Some(row_data) = grid.get(row as usize) {
            for index in (0..column).rev() {
                if let Some(checking_height) = self::find_value(grid, row, index) {
                    left_count += 1;
                    if checking_height >= current_height {
                        break;
                    }
                }
            }
            for index in (column)..row_data.len() as i32 {
                if column == index {
                    continue;
                }
                if let Some(checking_height) = self::find_value(grid, row, index) {
                    right_count += 1;
                    if checking_height >= current_height {
                        break;
                    }
                }
            }
        }

        // check up, down
        for index in (0..row).rev() {
            if let Some(checking_height) = self::find_value(grid, index, column) {
                top_count += 1;
                if checking_height >= current_height {
                    break;
                }
            }
        }
        for index in row..grid.len() as i32 {
            if row == index {
                continue;
            }

            if let Some(checking_height) = self::find_value(grid, index, column) {
                bottom_count += 1;
                if checking_height >= current_height {
                    break;
                }
            }
        }
        let score = left_count.max(1) * right_count.max(1) * top_count.max(1) * bottom_count.max(1);

        return score;
    }

    0
}

fn find_visible_spots(grid: &Vec<Vec<i32>>) -> i32 {
    let mut total_visibles = 0;
    for row in 0..grid.len() as i32 {
        let row_data = &grid[row as usize];
        for column in 0..row_data.len() as i32 {
            if self::is_visible(grid, row, column) {
                total_visibles += 1;
            }
        }
    }

    total_visibles
}

fn is_visible(grid: &Vec<Vec<i32>>, row: i32, column: i32) -> bool {
    if let Some(current_height) = self::find_value(grid, row, column) {
        let mut left_visible = true;
        let mut right_visible = true;
        let mut top_visible = true;
        let mut bottom_visible = true;

        if self::is_edge(grid, row, column) {
            return true;
        }

        // check left, right
        if let Some(row_data) = grid.get(row as usize) {
            for index in (0..column).rev() {
                if let Some(checking_height) = self::find_value(grid, row, index) {
                    if checking_height >= current_height {
                        left_visible = false;
                        break;
                    }
                }
            }
            for index in (column)..row_data.len() as i32 {
                if column == index {
                    continue;
                }
                if let Some(checking_height) = self::find_value(grid, row, index) {
                    if checking_height >= current_height {
                        right_visible = false;
                        break;
                    }
                }
            }
        } else {
            return true;
        }
        // check up, down
        for index in (0..row).rev() {
            if let Some(checking_height) = self::find_value(grid, index, column) {
                if checking_height >= current_height {
                    top_visible = false;
                    break;
                }
            }
        }
        for index in row..grid.len() as i32 {
            if row == index {
                continue;
            }

            if let Some(checking_height) = self::find_value(grid, index, column) {
                if checking_height >= current_height {
                    bottom_visible = false;
                    break;
                }
            }
        }

        return left_visible || right_visible || top_visible || bottom_visible;
    }

    true
}

fn is_edge(grid: &Vec<Vec<i32>>, row: i32, column: i32) -> bool {
    if row <= 0 || row >= grid.len() as i32 - 1 {
        return true;
    } else if column <= 0 {
        return true;
    } else {
        if let Some(row_data) = grid.first() {
            return column >= row_data.len() as i32 - 1;
        }
    }

    false
}

fn parsing_input(lines: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut grid = vec![];
    for line in lines.iter() {
        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect_vec();
        grid.push(row);
    }

    grid
}

fn find_value(grid: &Vec<Vec<i32>>, row: i32, column: i32) -> Option<i32> {
    if row < 0 || column < 0 {
        return None;
    }

    if let Some(row_data) = grid.get(row as usize) {
        if let Some(value) = row_data.get(column as usize) {
            return Some(*value);
        }
    }
    None
}

const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day08::part_01(&lines);
    assert_eq!(result, 21);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day08::part_02(&lines);
    assert_eq!(result, 8);
}
