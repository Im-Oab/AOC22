use crate::file_handler::FileHandler;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day14 {}

impl Day14 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_14_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day14::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day14::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_14".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i32 {
        let (blocks, mut rocks, width, height) = parse_input(&lines);

        tilt_platform(&blocks, &mut rocks, width, height, TiltDirection::North);

        let score = calculate_load(&rocks, height);
        score as i32
    }

    fn part_02(lines: &Vec<&str>) -> usize {
        let (blocks, mut rocks, width, height) = parse_input(&lines);
        let mut cache: HashMap<usize, Vec<i32>> = HashMap::new();
        for index in 0..1000 {
            process_one_cycle(&blocks, &mut rocks, width, height);

            let score = calculate_load(&rocks, height);
            if cache.contains_key(&score) == true {
                if let Some(list) = cache.get_mut(&score) {
                    list.push(index);
                }
            } else {
                cache.insert(score, vec![index]);
            }

            if index > 0 {
                println!("Loop: {}", index);
                cache.iter().for_each(|(score, indexes)| {
                    if indexes.len() >= 10 {
                        println!("Score: {} Indexes: {}", score, indexes.len());
                    }
                });
                println!("---\n");
            }
        }

        // cache.iter().for_each(|(score, indexes)|{
        //     println!("Score: {} Indexes: {}", score, indexes.len());
        // });

        let score = calculate_load(&rocks, height);
        score
    }
}

#[derive(Debug, Clone)]
enum TiltDirection {
    North,
    South,
    East,
    West,
}

fn calculate_load(rocks: &HashSet<(usize, usize)>, height: usize) -> usize {
    rocks
        .par_iter()
        .map(|(_, row)| {
            let score = height - *row;

            score
        })
        .sum()
}

fn is_empty(
    blocks: &HashSet<(usize, usize)>,
    rocks: &HashSet<(usize, usize)>,
    coord: (usize, usize),
    width: usize,
    height: usize,
) -> bool {
    if coord.0 >= width || coord.1 >= height {
        return false;
    }

    blocks.contains(&coord) == false && rocks.contains(&coord) == false
}

fn parse_input(
    input: &Vec<&str>,
) -> (
    HashSet<(usize, usize)>,
    HashSet<(usize, usize)>,
    usize,
    usize,
) {
    let mut width = 0;
    let mut height = input.len();

    let mut blocks = HashSet::new();
    let mut rocks = HashSet::new();

    input.iter().enumerate().for_each(|(row, columns_data)| {
        width = columns_data.len();
        columns_data.chars().enumerate().for_each(|(column, c)| {
            let coord = (column, row);
            match c {
                'O' => {
                    rocks.insert(coord);
                }
                '#' => {
                    blocks.insert(coord);
                }
                _ => {}
            }
        });
    });

    (blocks, rocks, width, height)
}

const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tilt_one_cycle() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (blocks, mut rocks, width, height) = parse_input(&lines);
        process_one_cycle(&blocks, &mut rocks, width, height);
        print_board(&blocks, &rocks, width, height);

        process_one_cycle(&blocks, &mut rocks, width, height);
        print_board(&blocks, &rocks, width, height);

        process_one_cycle(&blocks, &mut rocks, width, height);
        print_board(&blocks, &rocks, width, height);
    }

    #[test]
    fn test_move_rocks() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (blocks, mut rocks, width, height) = parse_input(&lines);

        tilt_platform(&blocks, &mut rocks, width, height, TiltDirection::North);

        let score = calculate_load(&rocks, height);
        assert_eq!(score, 136);
    }

    #[test]
    fn test_parse_input() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (blocks, rocks, width, height) = parse_input(&lines);
        assert_eq!(18, rocks.len());
        assert_eq!(17, blocks.len());
        assert_eq!(10, height);
        assert_eq!(10, width);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day14::part_01(&lines);
        assert_eq!(result, 136);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day14::part_02(&lines);
        assert_eq!(result, 64);
    }
}

fn process_one_cycle(
    blocks: &HashSet<(usize, usize)>,
    rocks: &mut HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) {
    let cycle_direction = [
        TiltDirection::North,
        TiltDirection::West,
        TiltDirection::South,
        TiltDirection::East,
    ];

    cycle_direction.iter().for_each(|direction| {
        tilt_platform(blocks, rocks, width, height, direction.clone());
    });
}

fn print_board(
    blocks: &HashSet<(usize, usize)>,
    rocks: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) {
    for row in 0..height {
        for column in 0..width {
            let coord = (column, row);
            if blocks.contains(&coord) == true {
                print!("#");
            } else if rocks.contains(&coord) == true {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    println!("\n---\n");
}

fn tilt_platform(
    blocks: &HashSet<(usize, usize)>,
    rocks: &mut HashSet<(usize, usize)>,
    width: usize,
    height: usize,
    direction: TiltDirection,
) {
    loop {
        let mut rocks_list: Vec<(usize, usize)> = rocks.iter().map(|v| *v).collect();

        rocks_list.sort_by(|a, b| a.1.cmp(&b.1));

        let mut not_move = true;
        rocks_list.iter().for_each(|coord| {
            match direction {
                TiltDirection::North => {
                    if move_rock_north(blocks, rocks, *coord, width, height) == true {
                        // rock moved
                        not_move = false;
                    }
                }
                TiltDirection::South => {
                    if move_rock_south(blocks, rocks, *coord, width, height) == true {
                        // rock moved
                        not_move = false;
                    }
                }
                TiltDirection::East => {
                    if move_rock_east(blocks, rocks, *coord, width, height) == true {
                        // rock moved
                        not_move = false;
                    }
                }
                TiltDirection::West => {
                    if move_rock_west(blocks, rocks, *coord, width, height) == true {
                        // rock moved
                        not_move = false;
                    }
                }
            }
        });

        if not_move == true {
            break;
        }
    }
}

/// move rock from current coord up one step without checking that it available or not
fn move_rock_north(
    blocks: &HashSet<(usize, usize)>,
    rocks: &mut HashSet<(usize, usize)>,
    coord: (usize, usize),
    width: usize,
    height: usize,
) -> bool {
    let next_coord = (coord.0, coord.1.saturating_sub(1));
    if is_empty(blocks, rocks, next_coord, width, height) == false {
        return false;
    }

    if coord != next_coord && rocks.remove(&coord) == true {
        rocks.insert(next_coord);

        true
    } else {
        false
    }
}

fn move_rock_south(
    blocks: &HashSet<(usize, usize)>,
    rocks: &mut HashSet<(usize, usize)>,
    coord: (usize, usize),
    width: usize,
    height: usize,
) -> bool {
    let next_coord = (coord.0, coord.1.saturating_add(1));
    if is_empty(blocks, rocks, next_coord, width, height) == false {
        return false;
    }

    if coord != next_coord && rocks.remove(&coord) == true {
        rocks.insert(next_coord);

        true
    } else {
        false
    }
}

fn move_rock_west(
    blocks: &HashSet<(usize, usize)>,
    rocks: &mut HashSet<(usize, usize)>,
    coord: (usize, usize),
    width: usize,
    height: usize,
) -> bool {
    let next_coord = (coord.0.saturating_sub(1), coord.1);
    if is_empty(blocks, rocks, next_coord, width, height) == false {
        return false;
    }

    if coord != next_coord && rocks.remove(&coord) == true {
        rocks.insert(next_coord);

        true
    } else {
        false
    }
}

fn move_rock_east(
    blocks: &HashSet<(usize, usize)>,
    rocks: &mut HashSet<(usize, usize)>,
    coord: (usize, usize),
    width: usize,
    height: usize,
) -> bool {
    let next_coord = (coord.0.saturating_add(1), coord.1);
    if is_empty(blocks, rocks, next_coord, width, height) == false {
        return false;
    }

    if coord != next_coord && rocks.remove(&coord) == true {
        rocks.insert(next_coord);

        true
    } else {
        false
    }
}
