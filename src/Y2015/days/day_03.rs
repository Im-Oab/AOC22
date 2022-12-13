use std::{collections::HashMap, time::Instant};

use itertools::Itertools;

use crate::file_handler::FileHandler;

pub struct Day03 {}

impl Day03 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2015/inputs/day_03_1.txt");

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

    fn part_01(lines: &Vec<&str>) -> usize {
        let directions = parsing(lines);

        return delivery(&directions).0;
    }

    /// It is still easy, but I have learnt how to merge two hashmaps using .extend()
    fn part_02(lines: &Vec<&str>) -> usize {
        let directions = parsing(lines);
        let mut first_directions: Vec<Direction> = vec![];
        let mut second_directions: Vec<Direction> = vec![];
        for (index, dir) in directions.iter().enumerate() {
            if index % 2 == 0 {
                first_directions.push(dir.clone());
            } else {
                second_directions.push(dir.clone());
            }
        }

        let mut first_result = delivery(&first_directions).1;
        let second_result = delivery(&second_directions).1;
        first_result.extend(second_result);
        return first_result.len();
    }
}

const TEST_INPUT: &str = "^>v<";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day03::part_01(&lines);
    assert_eq!(result, 4);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day03::part_02(&lines);
    assert_eq!(result, 3);
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn delivery(directions: &Vec<Direction>) -> (usize, HashMap<(i32, i32), i32>) {
    let mut presents_count = HashMap::new();
    let mut position = (0, 0);
    presents_count.insert(position.clone(), 1);

    for dir in directions.iter() {
        match dir {
            Direction::Up => position.1 -= 1,
            Direction::Down => position.1 += 1,
            Direction::Left => position.0 -= 1,
            Direction::Right => position.0 += 1,
            _ => {}
        }

        if let Some(count) = presents_count.get_mut(&position) {
            *count += 1;
        } else {
            presents_count.insert(position.clone(), 1);
        }
    }

    (presents_count.len(), presents_count)
}

fn parsing(lines: &Vec<&str>) -> Vec<Direction> {
    lines[0]
        .clone()
        .chars()
        .map(|direction| match direction {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => Direction::None,
        })
        .collect_vec()
}
