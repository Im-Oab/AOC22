use std::time::Instant;

use crate::file_handler::FileHandler;

enum Direction {
    UP,
    DOWN,
}

pub struct Day01 {}

impl Day01 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2015/inputs/day_01_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day01::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day01::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_01".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// I want to see what the puzzle look like in 2015. So, I start playing previous years puzzle.
    /// The first puzzle quite easy.
    fn part_01(lines: &Vec<&str>) -> i32 {
        let (total_ups, total_downs, _) = parsing(lines);
        return total_ups - total_downs;
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let (_, _, move_orders) = parsing(lines);
        let position = processing(&move_orders);
        return position as i32;
    }
}

const TEST_INPUT: &str = "))(((((";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day01::part_01(&lines);
    assert_eq!(result, 3);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = "()())".lines().collect();
    let result = Day01::part_02(&lines);
    assert_eq!(result, 5);
}

fn processing(move_orders: &Vec<Direction>) -> usize {
    let mut floor = 0;
    for (index, direction) in move_orders.iter().enumerate() {
        match direction {
            Direction::UP => floor += 1,
            Direction::DOWN => {
                floor -= 1;
                if floor < 0 {
                    return index + 1;
                }
            }
        }
    }

    panic!("It should not pass through here")
}

fn parsing(lines: &Vec<&str>) -> (i32, i32, Vec<Direction>) {
    let mut total_up = 0;
    let mut total_down = 0;
    let mut order = vec![];
    for c in lines.first().unwrap().chars() {
        if c == '(' {
            total_up += 1;
            order.push(Direction::UP);
        } else if c == ')' {
            total_down += 1;
            order.push(Direction::DOWN);
        }
    }

    (total_up, total_down, order)
}
