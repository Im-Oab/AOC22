use std::time::Instant;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashMap, HashSet};

use crate::file_handler::FileHandler;

pub struct Day11 {}

impl Day11 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_11_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day11::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day11::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_11".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i128 {
        let mut universe = Universe::new(lines);
        universe.expand();

        let pairs = universe.get_galaxy_pairs();
        pairs
            .par_iter()
            .map(|(s, t)| {
                if let Some(source) = universe.get_galaxy_coordinate(*s) {
                    if let Some(target) = universe.get_galaxy_coordinate(*t) {
                        return find_shortest_path(source, target);
                    }
                }
                0
            })
            .sum()
    }

    fn part_02(lines: &Vec<&str>) -> i128 {
        let mut universe = Universe::new(lines);
        universe.expand_in_name(1000000);

        let pairs = universe.get_galaxy_pairs();
        pairs
            .par_iter()
            .map(|(s, t)| {
                if let Some(source) = universe.get_galaxy_coordinate(*s) {
                    if let Some(target) = universe.get_galaxy_coordinate(*t) {
                        return find_shortest_path(source, target);
                    }
                }
                0
            })
            .sum()
    }
}

struct Universe {
    width: usize,
    height: usize,
    grids: HashMap<(i128, i128), String>,
    array: Vec<Vec<String>>,
    galaxies: HashMap<i32, (i128, i128)>,
}

impl Universe {
    fn new(input: &Vec<&str>) -> Self {
        let (grids, array, width, height) = parse_input(input);

        Self {
            width: width,
            height: height,
            grids: grids,
            array: array,
            galaxies: HashMap::new(),
        }
    }

    fn update_galaxies(&mut self) {
        self.galaxies.clear();

        self.array.iter().enumerate().for_each(|(row, list)| {
            list.iter().enumerate().for_each(|(column, value)| {
                if value == "#" {
                    self.galaxies.insert(
                        self.galaxies.len() as i32 + 1,
                        (column as i128, row as i128),
                    );
                }
            });
        });
    }

    fn get_galaxy_coordinate(&self, number: i32) -> Option<(i128, i128)> {
        self.galaxies.get(&number).clone().copied()
    }

    fn get_galaxy_pairs(&self) -> Vec<(i32, i32)> {
        let mut result = vec![];
        for source_index in 0..self.galaxies.len() {
            for target_index in (source_index + 1)..self.galaxies.len() {
                result.push((source_index as i32 + 1, target_index as i32 + 1));
            }
        }

        result
    }

    fn expand_in_name(&mut self, expand_times: i128) {
        self.update_galaxies();
        let empty_columns = self.find_empty_columns();
        let empty_rows = self.find_empty_rows();

        let new_galaxies: HashMap<i32, (i128, i128)> = self
            .galaxies
            .iter()
            .map(|(number, coord)| {
                let new_coord = calculate_new_coord_after_expand(
                    *coord,
                    &empty_columns,
                    &empty_rows,
                    expand_times,
                );
                (*number, new_coord)
            })
            .collect();

        self.galaxies = new_galaxies;
    }

    fn expand(&mut self) {
        let empty_columns = self.find_empty_columns();
        let total_columns = self.width + empty_columns.len();
        for exp_column in empty_columns.iter().rev() {
            let index = *exp_column as usize;
            for columns in self.array.iter_mut() {
                columns.insert(index, ".".to_owned());
            }
        }

        let empty_rows = self.find_empty_rows();
        let total_rows = self.height + empty_rows.len();
        for exp_row in empty_rows.iter().rev() {
            let index = *exp_row as usize;
            let new_row = (0..total_columns).map(|_| ".".to_owned()).collect();
            self.array.insert(index, new_row);
        }

        self.width = total_columns;
        self.height = total_rows;

        self.array.iter().enumerate().for_each(|(row, list)| {
            list.iter().enumerate().for_each(|(column, value)| {
                self.grids
                    .insert((column as i128, row as i128), value.to_owned());
            });
        });

        self.update_galaxies();
    }

    fn find_empty_columns(&self) -> Vec<i128> {
        let mut result = vec![];
        'next_column: for column in 0..self.width {
            for row in 0..self.height {
                let coord = (column as i128, row as i128);
                if let Some(value) = self.grids.get(&coord) {
                    if value == "#" {
                        continue 'next_column;
                    }
                }
            }

            result.push(column as i128);
        }

        result
    }

    fn find_empty_rows(&self) -> Vec<i128> {
        let mut result = vec![];
        'next_row: for row in 0..self.height {
            for column in 0..self.width {
                let coord = (column as i128, row as i128);
                if let Some(value) = self.grids.get(&coord) {
                    if value == "#" {
                        continue 'next_row;
                    }
                }
            }

            result.push(row as i128);
        }

        result
    }

    fn is_galaxy(&self, coord: (i128, i128)) -> bool {
        if let Some(value) = self.grids.get(&coord) {
            value == "#"
        } else {
            false
        }
    }
}

fn find_shortest_path(source: (i128, i128), target: (i128, i128)) -> i128 {
    let (x1, y1) = source;
    let (x2, y2) = target;
    let diff_x = (x2 - x1) - 0;
    let diff_y = (y2 - y1) - 0;
    diff_x.abs() + diff_y.abs()
}

fn calculate_new_coord_after_expand(
    source: (i128, i128),
    empty_columns: &Vec<i128>,
    empty_rows: &Vec<i128>,
    expand_times: i128,
) -> (i128, i128) {
    let multiply_columns: i128 = empty_columns
        .par_iter()
        .map(|column| if *column < source.0 { 1 } else { 0 })
        .sum();

    let multiply_rows: i128 = empty_rows
        .par_iter()
        .map(|row| if *row < source.1 { 1 } else { 0 })
        .sum();

    let replace_x = multiply_columns;

    let replace_y = multiply_rows;

    (
        source.0 - replace_x + multiply_columns * expand_times,
        source.1 - replace_y + multiply_rows * expand_times,
    )
}

fn parse_input(
    input: &Vec<&str>,
) -> (
    HashMap<(i128, i128), String>,
    Vec<Vec<String>>,
    usize,
    usize,
) {
    let height = input.len();
    let mut width = 0;

    let data: Vec<Vec<((i128, i128), String)>> = input
        .par_iter()
        .enumerate()
        .map(|(row, data)| {
            let data: Vec<((i128, i128), String)> = (*data)
                .chars()
                .enumerate()
                .map(|(column, v)| ((column as i128, row as i128), String::from(v)))
                .collect();

            data
        })
        .collect();

    let mut grids = HashMap::new();
    let array = data
        .iter()
        .map(|columns| {
            width = columns.len();
            let values: Vec<String> = columns
                .iter()
                .map(|((x, y), value)| {
                    grids.insert((*x, *y), value.clone());
                    value.clone()
                })
                .collect();

            values
        })
        .collect();

    (grids, array, width, height)
}

const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expand_2_times() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let mut universe = Universe::new(&lines);
        universe.update_galaxies();
        universe.galaxies.iter().for_each(|(number, coord)| {
            println!("{}| {},{}", number, coord.0, coord.1);
        });
        universe.expand_in_name(2);
        println!("---");
        universe.galaxies.iter().for_each(|(number, coord)| {
            println!("{}| {},{}", number, coord.0, coord.1);
        });
        let pairs = universe.get_galaxy_pairs();
        let result: i128 = pairs
            .par_iter()
            .map(|(s, t)| {
                if let Some(source) = universe.get_galaxy_coordinate(*s) {
                    if let Some(target) = universe.get_galaxy_coordinate(*t) {
                        return find_shortest_path(source, target);
                    }
                }
                0
            })
            .sum();
        assert_eq!(result, 374);
    }

    #[test]
    fn test_expand_10_times() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let mut universe = Universe::new(&lines);
        universe.update_galaxies();

        universe.expand_in_name(10);

        let pairs = universe.get_galaxy_pairs();
        let result: i128 = pairs
            .par_iter()
            .map(|(s, t)| {
                if let Some(source) = universe.get_galaxy_coordinate(*s) {
                    if let Some(target) = universe.get_galaxy_coordinate(*t) {
                        return find_shortest_path(source, target);
                    }
                }
                0
            })
            .sum();
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_shortest_path() {
        let source = (3, 0);
        let target = (7, 1);
        let distance = find_shortest_path(source, target);
        assert_eq!(distance, 5);

        let source = (3, 0);
        let target = (0, 2);
        let distance = find_shortest_path(source, target);
        assert_eq!(distance, 5);

        let source = (3, 0);
        let target = (6, 5);
        let distance = find_shortest_path(source, target);
        assert_eq!(distance, 8);
    }

    #[test]
    fn test_expand_universe() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let mut universe = Universe::new(&lines);
        assert_eq!(universe.width, 10);
        assert_eq!(universe.height, 10);
        assert_eq!(true, universe.is_galaxy((3, 0)));
        universe.expand();
        assert_eq!(universe.width, 13);
        assert_eq!(universe.height, 12);
        assert_eq!(true, universe.is_galaxy((4, 0)));
    }
    #[test]
    fn test_universe_find_empty_spaces() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let universe = Universe::new(&lines);
        let empty_columns = universe.find_empty_columns();
        assert_eq!(empty_columns.len(), 3);
        assert_eq!(empty_columns, vec![2, 5, 8]);

        let empty_rows = universe.find_empty_rows();
        assert_eq!(empty_rows.len(), 2);
        assert_eq!(empty_rows, vec![3, 7]);
    }

    #[test]
    fn test_parse_input() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (grids, _, width, height) = parse_input(&lines);
        assert_eq!(grids.get(&(3, 0)), Some(&String::from("#")));
        assert_eq!(width, 10);
        assert_eq!(height, 10);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day11::part_01(&lines);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day11::part_02(&lines);
        assert_eq!(result, 2286);
    }
}
