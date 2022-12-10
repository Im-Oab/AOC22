use std::time::Instant;

use crate::file_handler::FileHandler;

enum Instructions {
    ADDX(i32, usize),
    NOOP,
}
pub struct Day10 {}

impl Day10 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/inputs/day_10_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day10::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day10::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_10".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// This part should been easy,
    /// but It took me much time to get the result. I felt like today's puzzle was playing with a word to trick me.
    /// So, I got the correct result after I read it carefully.
    fn part_01(lines: &Vec<&str>) -> i32 {
        let instructions = parsing_input(&lines);
        let (_, registered_values) = process(&instructions);

        return registered_values.iter().take(registered_values.len()).sum();
    }

    /// For part 2, while I implemented the code, The result of the example input did not match exactly with the example after struggling with it for some time.
    /// I desperately tried with my puzzle input and got the correct result. :)
    /// After I rechecked the code again, I found out that I messed up when converting the cycle's value and sprite position.
    fn part_02(lines: &Vec<&str>) -> i32 {
        let instructions = parsing_input(&lines);
        println!("day_10_part2:\n{}\n", rendering(&instructions));

        return 0;
    }
}

/// process instructions and output result in string.
fn rendering(instructions: &Vec<Instructions>) -> String {
    let mut sprite_position = 0;
    let mut cycle = 1;
    let mut canvas = String::new();
    for instruction in instructions.iter() {
        match instruction {
            Instructions::ADDX(value, total_cycle) => {
                for _ in 0..*total_cycle {
                    let render_value = get_render_value(sprite_position, cycle);
                    canvas.push_str(render_value);

                    increase_cycle(&mut canvas, &mut cycle);
                }

                sprite_position += value;
            }
            Instructions::NOOP => {
                let render_value = get_render_value(sprite_position, cycle);
                canvas.push_str(render_value);

                increase_cycle(&mut canvas, &mut cycle);
            }
        }
    }

    canvas
}

fn increase_cycle(canvas: &mut String, cycle: &mut i32) {
    *cycle += 1;

    add_new_line_on_canvas(canvas, *cycle);
}

fn add_new_line_on_canvas(canvas: &mut String, cycle: i32) {
    // Canvas width is 40.
    // If cycle reach the first pixel. It should add new line
    // and It should avoid the first row.
    if cycle > 40 && cycle % 40 == 1 {
        canvas.push_str("\n");
    }
}

fn get_render_value(sprite_position: i32, cycle: i32) -> &'static str {
    let cycle = (cycle - 1) % 40;
    if cycle >= sprite_position && cycle < sprite_position + 3 {
        "#"
    } else {
        "."
    }
}

fn process(instructions: &Vec<Instructions>) -> (i32, Vec<i32>) {
    let mut registered_values: Vec<i32> = vec![];
    let mut cycle = 1;
    let mut cpu_value = 1;

    for instruction in instructions.iter() {
        match instruction {
            Instructions::ADDX(value, total_cycles) => {
                for _ in 0..*total_cycles {
                    if is_cycle_registered(cycle) == true {
                        registered_values.push(cpu_value * cycle as i32);
                    }
                    cycle += 1;
                }

                cpu_value += value;
            }
            Instructions::NOOP => {
                if is_cycle_registered(cycle) == true {
                    registered_values.push(cpu_value * cycle as i32);
                }
                cycle += 1;
            }
        }

        // It should not calculate cycle after 220th, because we need only result until cycle 220th
        if cycle > 220 {
            break;
        }
    }

    (cpu_value, registered_values)
}

fn is_cycle_registered(cycle: usize) -> bool {
    let registered_cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    registered_cycles.contains(&cycle)
}

fn parsing_input(lines: &Vec<&str>) -> Vec<Instructions> {
    let mut result = vec![];
    for line in lines.iter() {
        let splited: Vec<&str> = line.split(" ").collect();
        if splited[0] == "addx" {
            let value = splited[1].parse::<i32>().unwrap();
            result.push(Instructions::ADDX(value, 2))
        } else if splited[0] == "noop" {
            result.push(Instructions::NOOP);
        } else {
            panic!("Unknown instruction appear");
        }
    }

    result
}

const TEST_PROCESS: &str = "noop
addx 3
addx -5";

#[test]
fn test_processing() {
    let lines: Vec<&str> = TEST_PROCESS.lines().collect();
    let instructions = parsing_input(&lines);
    let (result, _) = process(&instructions);
    assert_eq!(result, -1);
}

const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day10::part_01(&lines);
    assert_eq!(result, 13140);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let instructions = parsing_input(&lines);
    let expected_result = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n";
    let result = rendering(&instructions);
    println!("Canvas:\n{}\nexpected:\n{}\n\n", result, expected_result);
    assert_eq!(result.as_str(), expected_result);
}
