use crate::file_handler::FileHandler;
use geo::polygon;
use geo::Area;
use geo::{Contains, LineString, Point, Polygon};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day18 {}

impl Day18 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_18_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day18::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day18::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_18".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i64 {
        let data = parse_input(&lines);
        let (top, left, bottom, right, digs) = dig(&data);
        let filled = fill(top, left, bottom, right, &digs);
        filled.len() as i64
    }

    fn part_02(lines: &Vec<&str>) -> i64 {
        let data = parse_input_hex(&lines);
        let (top, left, bottom, right, digs) = dig(&data);
        // let filled = fill(top, left, bottom, right, &digs);

        magic_shovel(&data)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DigDirection {
    Up,
    Down,
    Left,
    Right,
}

impl DigDirection {
    fn from_str(input: &str) -> Self {
        match input {
            "R" => DigDirection::Right,
            "L" => DigDirection::Left,
            "U" => DigDirection::Up,
            "D" => DigDirection::Down,
            _ => unreachable!("It should not be here"),
        }
    }

    fn from_i64(input: i64) -> Self {
        match input {
            0 => DigDirection::Right,
            1 => DigDirection::Down,
            2 => DigDirection::Left,
            3 => DigDirection::Up,

            _ => unreachable!("It should not be here"),
        }
    }
}

/// Ideas from Advent of rust discord channel.
/// Note: Dig to Left/right is define width.Dig up and down is paint or undo the paint.

fn magic_shovel(commands: &Vec<(DigDirection, i64)>) -> i64 {
    let mut column = 0;
    let mut total_hole = 1;
    commands
        .iter()
        .for_each(|(direction, total_meter)| match direction {
            // Increase width of the paint
            DigDirection::Right => {
                column += total_meter;
                total_hole += total_meter;
            }
            // Paint with the same width
            DigDirection::Down => {
                total_hole += total_meter * (column + 1);
            }
            // Adjust width
            DigDirection::Left => {
                column -= total_meter;
            }
            // Undo the paint.
            DigDirection::Up => {
                total_hole -= column * total_meter;
            }
        });

    total_hole
}

fn dig(commands: &Vec<(DigDirection, i64)>) -> (i64, i64, i64, i64, Vec<(i64, i64)>) {
    let (mut top, mut left) = (0, 0);
    let (mut bottom, mut right) = (0, 0);
    let mut start_coord = (0, 0);

    let mut digs: Vec<(i64, i64)> = vec![];
    digs.push(start_coord);
    commands.iter().for_each(|(direction, total_meters)| {
        (1..=*total_meters).for_each(|index| {
            let dif_offset = match direction {
                DigDirection::Up => (0, -index),
                DigDirection::Down => (0, index),
                DigDirection::Left => (-index, 0),
                DigDirection::Right => (index, 0),
            };

            let actual_dig_coord = (start_coord.0 + dif_offset.0, start_coord.1 + dif_offset.1);
            digs.push(actual_dig_coord);
            if actual_dig_coord.0 < left {
                left = actual_dig_coord.0;
            } else if actual_dig_coord.0 > right {
                right = actual_dig_coord.0;
            }
            if actual_dig_coord.1 < top {
                top = actual_dig_coord.1;
            } else if actual_dig_coord.1 > bottom {
                bottom = actual_dig_coord.1;
            }
        });
        start_coord = digs.last().copied().unwrap();
    });

    (top, left, bottom, right, digs)
}

fn fill(top: i64, left: i64, bottom: i64, right: i64, digs: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut filled: HashSet<(i64, i64)> = digs.iter().map(|coord| *coord).collect();
    let polygon = Polygon::new(LineString::from(digs.clone()), vec![]);
    for row in top..=bottom {
        for column in left..=right {
            let coord = (column, row);
            if polygon.contains(&Point::new(coord.0, coord.1)) == true {
                if filled.contains(&coord) == false {
                    filled.insert(coord);
                }
            }
        }
    }

    filled.iter().map(|coord| *coord).collect()
}

fn print_table(top: i64, left: i64, bottom: i64, right: i64, digs: &Vec<(i64, i64)>) {
    for row in top..=bottom {
        for column in left..=right {
            let coord = (column, row);
            if digs.contains(&coord) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("---");
}

fn parse_input(input: &Vec<&str>) -> Vec<(DigDirection, i64)> {
    input
        .iter()
        .map(|text| {
            let values: Vec<&str> = text.split(" ").collect();
            (
                DigDirection::from_str(values[0]),
                values[1].parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn parse_input_hex(input: &Vec<&str>) -> Vec<(DigDirection, i64)> {
    input
        .iter()
        .map(|text| {
            let values: Vec<&str> = text.split(" ").collect();
            let raw_hex = values[2].replace("(", "").replace(")", "").replace("#", "");
            let digit = i64::from_str_radix(&raw_hex[0..5].to_owned(), 16).unwrap();
            let direction = raw_hex[5..].to_owned().parse::<i64>().unwrap();

            (DigDirection::from_i64(direction), digit)
        })
        .collect()
}

const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_hex() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = parse_input_hex(&lines);
        assert_eq!(result[0].0, DigDirection::Right);
        assert_eq!(result[0].1, 461937);

        assert_eq!(result[1].0, DigDirection::Down);
        assert_eq!(result[1].1, 56407);
    }
    #[test]
    fn test_parse_input() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = parse_input(&lines);
        assert_eq!(result.len(), 14);

        let (top, left, bottom, right, digs) = dig(&result);
        print_table(top, left, bottom, right, &digs);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day18::part_01(&lines);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day18::part_02(&lines);
        assert_eq!(result, 952408144115);
    }
}
