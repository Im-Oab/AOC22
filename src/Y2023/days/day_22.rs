use std::time::Instant;
use std::collections::{HashMap, HashSet};
use rayon::prelude::*;
use crate::file_handler::FileHandler;

pub struct Day22 {}

impl Day22 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_22_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day22::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day22::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_22".to_owned(),
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

const TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day22::part_01(&lines);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day22::part_02(&lines);
        assert_eq!(result, 2286);
    }
}

