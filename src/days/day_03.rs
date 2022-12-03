use std::time::Instant;

use crate::file_handler::FileHandler;

///  I woke up early and prepared for today's puzzle.
/// I started working at 4:45PM and finished at 5:30pm
pub struct Day03 {}

impl Day03 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/inputs/day_03_1.txt");

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

    /// I look for how to find common characters in multiple strings. I found the solution about converting `char` into usize and using it as index of array.
    /// And It works very well to solve this issue.
    fn part_01(lines: &Vec<&str>) -> i32 {
        let mut total_value = 0;
        for line in lines.iter() {
            let middle = line.len() / 2;
            let items = (*line).split_at(middle);
            let first = items.0.to_owned();
            let second = items.1.to_owned();

            let mut first_containers = Day03::item_inspect(&first);
            let second_container = Day03::item_inspect(&second);

            Day03::merge_container(&mut first_containers, second_container);

            for index in 0..first_containers.len() {
                let value = first_containers[index];

                if value > 1 {
                    let index_value = Day03::calculate_value(index);
                    total_value += index_value as i32;

                    break;
                }
            }
        }

        return total_value as i32;
    }

    fn item_inspect(bag: &str) -> [u8; 125] {
        let mut container = [0; 125];
        for c in bag.chars() {
            container[c as usize] = 1;
        }

        container
    }

    fn merge_container(first: &mut [u8; 125], second: [u8; 125]) {
        for index in 0..first.len() {
            first[index] += second[index];
        }
    }

    //A: 65
    // Z: 90
    // a: 97
    // z: 122
    // Lowercase item types a through z have priorities 1 through 26.
    //Uppercase item types A through Z have priorities 27 through 52.
    fn calculate_value(index: usize) -> u8 {
        if index >= 65 && index <= 90 {
            (index - 65 + 27) as u8
        } else if index >= 97 && index <= 122 {
            (index - 97 + 1) as u8
        } else {
            0
        }
    }

    /// After I read part 2. The code is very straightforward. I only need to group the bags and repeat the same as part 1 to get the result.
    fn part_02(lines: &Vec<&str>) -> i32 {
        let mut total_value = 0;
        for group in 0..lines.len() / 3 {
            let index = group * 3;
            let first = lines[index];
            let second = lines[index + 1];
            let third = lines[index + 2];

            let mut first_container = Day03::item_inspect(first);
            let second_conatiner = Day03::item_inspect(second);
            let third_conatiner = Day03::item_inspect(third);

            Day03::merge_container(&mut first_container, second_conatiner);
            Day03::merge_container(&mut first_container, third_conatiner);

            for index in 0..first_container.len() {
                let value = first_container[index];

                if value == 3 {
                    let index_value = Day03::calculate_value(index);
                    // println!("{} {} {}", index, index_value, (index as u8) as char);
                    // format!("{} {}", (index as u8) as char, index_value);
                    total_value += index_value as i32;

                    break;
                }
            }
        }
        return total_value;
    }
}
