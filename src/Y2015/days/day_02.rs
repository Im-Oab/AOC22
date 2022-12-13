use std::time::Instant;

use itertools::Itertools;

use crate::file_handler::FileHandler;

pub struct Day02 {}

impl Day02 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2015/inputs/day_02_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day02::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day02::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_02".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// The puzzle is nothing much but still fun to do it.
    fn part_01(lines: &Vec<&str>) -> i32 {
        let boxes = parsing(lines);
        let mut total_areas = 0;
        for dimensions in boxes.iter() {
            total_areas += surface_area(dimensions.0, dimensions.1, dimensions.2);
        }
        return total_areas;
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let boxes = parsing(lines);
        let mut total_ribbon_length = 0;
        for dimensions in boxes.iter() {
            total_ribbon_length += ribbon_length(dimensions.0, dimensions.1, dimensions.2)
        }
        return total_ribbon_length;
    }
}

const TEST_INPUT: &str = "2x3x4";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day02::part_01(&lines);
    assert_eq!(result, 58);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day02::part_02(&lines);
    assert_eq!(result, 34);
}

fn ribbon_length(w: i32, h: i32, l: i32) -> i32 {
    let mut dimensions = vec![w, h, l];
    dimensions.sort();
    let half_wrapping: i32 = dimensions.iter().take(2).sum();
    let bow: i32 = dimensions.iter().product();
    2 * half_wrapping + bow
}

fn surface_area(w: i32, h: i32, l: i32) -> i32 {
    let mut areas = vec![w * h, h * l, l * w];

    areas.sort();
    let total_areas: i32 = areas.iter().sum();

    (2 * total_areas) + (*areas.first().unwrap())
}

fn parsing(lines: &Vec<&str>) -> Vec<(i32, i32, i32)> {
    lines
        .iter()
        .map(|line| {
            let splited = (*line).split("x").collect::<Vec<&str>>();
            return (
                splited[0].parse::<i32>().unwrap(),
                splited[1].parse::<i32>().unwrap(),
                splited[2].parse::<i32>().unwrap(),
            );
        })
        .collect_vec()
}
