use std::time::Instant;

use crate::file_handler::FileHandler;

use super::day_03::Day03;

#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

pub struct Day02 {}

impl Day02 {
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

    /// I started the part where it needed to compare two inputs with nested of "match".
    fn part_01(lines: &Vec<&str>) -> i32 {
        let mut total_score = 0;
        for line in lines.iter() {
            let values: Vec<&str> = line.split(' ').collect();
            let opponent = Day02::convert_input(values[0]);
            let you = Day02::convert_input(values[1]);

            let comparing_score = Day02::comparing(&you, &opponent);
            let your_command_score = Day02::command_score(&you);

            let match_score = comparing_score + your_command_score;
            total_score += match_score;
        }

        return total_score;
    }

    // 1 for Rock, 2 for Paper, and 3 for Scissors
    fn command_score(input: &self::RPS) -> i32 {
        match input {
            RPS::Paper => 2,
            RPS::Rock => 1,
            _ => 3,
        }
    }

    fn comparing(left: &self::RPS, right: &self::RPS) -> i32 {
        match left {
            RPS::Paper => match right {
                RPS::Paper => 3,
                RPS::Rock => 6,
                _ => 0,
            },
            RPS::Rock => match right {
                RPS::Paper => 0,
                RPS::Rock => 3,
                _ => 6,
            },
            RPS::Scissor => match right {
                RPS::Paper => 6,
                RPS::Rock => 0,
                _ => 3,
            },
        }
    }

    //  A for Rock, B for Paper, and C for Scissors.
    // X for Rock, Y for Paper, and Z for Scissors
    fn convert_input(input: &str) -> self::RPS {
        match input {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            _ => RPS::Scissor,
        }
    }

    /// I continued this part with the code from the previous part and added the logic for choosing the command to have the expected match result.
    /// I continue using "if" and "match" for comparing.
    /// At the end. I have some ideas sparking; I could use a 2d array for comparing instead. I will get back to it if I have free time.
    fn part_02(lines: &Vec<&str>) -> i32 {
        let mut total_score = 0;
        for line in lines.iter() {
            let values: Vec<&str> = line.split(' ').collect();
            let opponent = Day02::convert_input(values[0]);
            let expected = Day02::convert_expected(values[1]);
            let you = Day02::choose(&opponent, expected);

            let comparing_score = Day02::comparing(&you, &opponent);
            let your_command_score = Day02::command_score(&you);

            let match_score = comparing_score + your_command_score;
            total_score += match_score;
        }
        return total_score;
    }

    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
    fn convert_expected(input: &str) -> i32 {
        match input {
            "X" => -1,
            "Y" => 0,
            _ => 1,
        }
    }

    fn choose(opponent: &self::RPS, expect_result: i32) -> self::RPS {
        if expect_result == 0 {
            (*opponent).clone()
        } else if expect_result == 1 {
            match opponent {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissor,
                RPS::Scissor => RPS::Rock,
            }
        } else {
            match opponent {
                RPS::Rock => RPS::Scissor,
                RPS::Paper => RPS::Rock,
                RPS::Scissor => RPS::Paper,
            }
        }
    }
}

const TEST_INPUT: &str = "A Y
B X
C Z";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day02::part_01(&lines);
    assert_eq!(result, 15);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day02::part_02(&lines);
    assert_eq!(result, 12);
}
