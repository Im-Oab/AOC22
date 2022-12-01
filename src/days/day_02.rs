use std::time::Instant;

use crate::file_handler::FileHandler;

pub struct Day02 {}

impl Day02{
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/inputs/day_02_1.txt");

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

    fn part_01(lines: &Vec<&str>) -> i32 {

        return 0;
    }

    fn part_02(lines: &Vec<&str>) -> i32 {

        return 0;
    }
}