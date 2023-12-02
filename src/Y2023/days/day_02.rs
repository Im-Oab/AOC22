use std::time::Instant;

use std::collections::HashMap;

use crate::file_handler::FileHandler;

pub struct Day02 {}

impl Day02 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_02_1.txt");

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

    //only 12 red cubes, 13 green cubes, and 14 blue cubes?
    fn part_01(lines: &Vec<&str>) -> i32 {
        let mut maximum_values = HashMap::new();
        maximum_values.insert(CubeType::Red, 12);
        maximum_values.insert(CubeType::Green, 13);
        maximum_values.insert(CubeType::Blue, 14);

        let mut total_sum = 0;
        for line in lines.iter() {
            let (game_id, game_sets) = parsing_input(*line);
            let mut possible = true;
            for game in game_sets.iter() {
                if validate_game(&maximum_values, game) == false {
                    possible = false;
                    break;
                }
            }

            if possible == true {
                total_sum += game_id;
            }
        }

        total_sum
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let mut total_sum = 0;
        for line in lines.iter() {
            let (_, game_sets) = parsing_input(*line);
            let power = get_game_power(&game_sets);

            total_sum += power;
        }

        total_sum
    }
}

fn get_game_power(game_sets: &Vec<HashMap<CubeType, i32>>) -> i32 {
    let mut power = 1;
    let mut maximum_cubes: HashMap<CubeType, i32> = HashMap::new();
    for game in game_sets.iter() {
        for (cube, value) in game.iter() {
            if let Some(maximum_value) = maximum_cubes.get(cube) {
                if *maximum_value < *value {
                    maximum_cubes.insert(cube.clone(), *value);
                }
            } else {
                maximum_cubes.insert(cube.clone(), *value);
            }
        }
    }

    for (_, value) in maximum_cubes.iter() {
        power *= *value;
    }

    power
}

fn validate_game(maximum_values: &HashMap<CubeType, i32>, game: &HashMap<CubeType, i32>) -> bool {
    for (key, value) in game.iter() {
        if let Some(maximum_values) = maximum_values.get(key) {
            if *value > *maximum_values {
                return false;
            }
        }
    }

    true
}

fn parsing_input(line: &str) -> (i32, Vec<HashMap<CubeType, i32>>) {
    let text = line.to_owned();
    let values: Vec<&str> = text.split(": ").collect();
    let game_id = values[0].replace("Game ", "").parse::<i32>().unwrap();
    let game_sets_text: Vec<&str> = values[1].split("; ").collect();

    let mut game_sets = vec![];
    for set in game_sets_text.iter() {
        let cube_set = parsing_set(*set);
        game_sets.push(cube_set);
    }

    (game_id, game_sets)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum CubeType {
    Blue,
    Red,
    Green,
}

impl CubeType {
    fn from_str(text: &str) -> CubeType {
        match text.to_lowercase().as_str() {
            "blue" => CubeType::Blue,
            "green" => CubeType::Green,
            "red" => CubeType::Red,
            _ => panic!("Unknown cube type: {}", text),
        }
    }
}

fn parsing_set(text: &str) -> HashMap<CubeType, i32> {
    let cube_rolls: Vec<&str> = text.split(", ").collect();

    let mut result = HashMap::new();
    for cubes in cube_rolls.iter() {
        let values: Vec<&str> = (*cubes).split(" ").collect();
        if values.len() != 2 {
            continue;
        }
        let total_cubes = values[0].parse::<i32>().unwrap();
        let cube_type = CubeType::from_str(values[1]);

        if let Some(value) = result.get(&cube_type) {
            result.insert(cube_type, *value + total_cubes);
        } else {
            result.insert(cube_type, total_cubes);
        }
    }

    result
}

const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

const TEST_INPUT_2: &str = "";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let (game_id, game_sets) =
            parsing_input("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game_id, 1);
        assert_eq!(game_sets[0][&CubeType::Blue], 3);
        assert_eq!(game_sets[0][&CubeType::Red], 4);
        assert_eq!(game_sets[1][&CubeType::Blue], 6);
        assert_eq!(game_sets[1][&CubeType::Red], 1);
        assert_eq!(game_sets[1][&CubeType::Green], 2);
        assert_eq!(game_sets[2][&CubeType::Green], 2);
    }

    #[test]
    fn test_get_game_power() {
        let (game_id, game_sets) =
            parsing_input("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let power = get_game_power(&game_sets);
        assert_eq!(power, 48);
    }
    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day02::part_01(&lines);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day02::part_02(&lines);
        assert_eq!(result, 2286);
    }
}
