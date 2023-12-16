use crate::file_handler::FileHandler;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{stdout, Write};
use std::thread;
use std::time::{Duration, Instant};

pub struct Day16 {}

impl Day16 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_16_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day16::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day16::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_16".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> usize {
        let (data, width, height) = parse_input(&lines);

        let result = process_light(&data, width, height, (0, 0), BeamDirection::Right);

        result.len()
    }

    fn part_02(lines: &Vec<&str>) -> usize {
        let (data, width, height) = parse_input(&lines);
        let mut top: Vec<((usize, usize), BeamDirection)> = (0..width)
            .map(|column| ((column, 0), BeamDirection::Down))
            .collect();

        let mut bottom = (0..width)
            .map(|column| ((column, height - 1), BeamDirection::Up))
            .collect();

        let mut left = (0..height)
            .map(|row| ((0, row), BeamDirection::Right))
            .collect();

        let mut right = (0..height)
            .map(|row| ((width - 1, row), BeamDirection::Left))
            .collect();

        let mut all_start_node: Vec<((usize, usize), BeamDirection)> = vec![];
        all_start_node.append(&mut top);
        all_start_node.append(&mut bottom);
        all_start_node.append(&mut left);
        all_start_node.append(&mut right);

        all_start_node
            .par_iter()
            .map(|(start_coord, start_beam)| {
                process_light(&data, width, height, *start_coord, *start_beam).len()
            })
            .max()
            .unwrap()
    }
}

fn process_light(
    data: &HashMap<(usize, usize), String>,
    width: usize,
    height: usize,
    start_coord: (usize, usize),
    start_beam: BeamDirection,
) -> HashMap<(usize, usize), Vec<BeamDirection>> {
    let mut queue = VecDeque::new();
    let mut duplicated_light = HashSet::new();
    queue.push_back((start_coord, start_beam));
    let mut energized: HashMap<(usize, usize), Vec<BeamDirection>> = HashMap::new();
    loop {
        if let Some(node) = queue.pop_back() {
            let current_coord = node.0;
            let current_beam = node.1;

            if let Some(list) = energized.get_mut(&current_coord) {
                if list.contains(&current_beam) == false {
                    list.push(current_beam);
                }
            } else {
                energized.insert(current_coord, vec![current_beam]);
            }

            if let Some(mirror) = data.get(&current_coord) {
                let new_lights = mirror_reflect(current_beam, mirror);
                new_lights.iter().for_each(|beam| {
                    let (can_travel, next_coord) =
                        can_travel_next(width, height, current_coord, *beam);
                    if can_travel == true {
                        if duplicated_light.contains(&(next_coord, *beam)) == false {
                            // same beam continue
                            queue.push_back((next_coord, *beam));
                            duplicated_light.insert((next_coord, *beam));
                        }
                    }
                });
            }
            // pass through
            else {
                let (can_travel, next_coord) =
                    can_travel_next(width, height, current_coord, current_beam);
                if can_travel == true {
                    if duplicated_light.contains(&(next_coord, current_beam)) == false {
                        // same beam continue
                        queue.push_back((next_coord, current_beam));
                        duplicated_light.insert((next_coord, current_beam));
                    }
                }
            }

            // print!("{}[2J", 27 as char);
            // print_direction_table(width, height, data, &energized);
            // thread::sleep(Duration::from_millis(10));
        } else {
            break;
        }
    }

    // print_table(width, height, data, &energized);

    // print_direction_table(width, height, data, &energized);
    energized
}

fn can_travel_next(
    width: usize,
    height: usize,
    coord: (usize, usize),
    beam: BeamDirection,
) -> (bool, (usize, usize)) {
    let column = coord.0;
    let row = coord.1;
    match beam {
        BeamDirection::Left => {
            if column == 0 {
                (false, coord)
            } else {
                (true, (column.saturating_sub(1), row))
            }
        }
        BeamDirection::Right => {
            if column == width - 1 {
                (false, coord)
            } else {
                (true, (column.saturating_add(1), row))
            }
        }
        BeamDirection::Up => {
            if row == 0 {
                (false, coord)
            } else {
                (true, (column, row.saturating_sub(1)))
            }
        }
        BeamDirection::Down => {
            if row == height - 1 {
                (false, coord)
            } else {
                (true, (column, row.saturating_add(1)))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BeamDirection {
    Up,
    Down,
    Right,
    Left,
}

impl BeamDirection {
    fn to_string(&self) -> String {
        match self {
            BeamDirection::Down => "V".to_owned(),
            BeamDirection::Up => "^".to_owned(),
            BeamDirection::Left => "<".to_owned(),
            BeamDirection::Right => ">".to_owned(),
        }
    }
}

fn mirror_reflect(beam: BeamDirection, mirror: &str) -> Vec<BeamDirection> {
    match beam {
        BeamDirection::Right => match mirror {
            "/" => vec![BeamDirection::Up],
            "\\" => vec![BeamDirection::Down],
            "|" => vec![BeamDirection::Up, BeamDirection::Down],
            "." | "-" => vec![beam],
            _ => unreachable!("It should not be here: {}", mirror),
        },

        BeamDirection::Left => match mirror {
            "/" => vec![BeamDirection::Down],
            "\\" => vec![BeamDirection::Up],
            "|" => vec![BeamDirection::Up, BeamDirection::Down],
            "." | "-" => vec![beam],
            _ => unreachable!("It should not be here: {}", mirror),
        },

        BeamDirection::Up => match mirror {
            "/" => vec![BeamDirection::Right],
            "\\" => vec![BeamDirection::Left],
            "-" => vec![BeamDirection::Left, BeamDirection::Right],
            "." | "|" => vec![beam],
            _ => unreachable!("It should not be here: {}", mirror),
        },

        BeamDirection::Down => match mirror {
            "/" => vec![BeamDirection::Left],
            "\\" => vec![BeamDirection::Right],
            "-" => vec![BeamDirection::Left, BeamDirection::Right],
            "." | "|" => vec![beam],
            _ => unreachable!("It should not be here: {}", mirror),
        },
    }
}

fn parse_input(input: &Vec<&str>) -> (HashMap<(usize, usize), String>, usize, usize) {
    let mut width = 0;
    let height = input.len();
    let mut data = HashMap::new();
    input.iter().enumerate().for_each(|(row, list)| {
        width = list.len();
        list.chars().enumerate().for_each(|(column, c)| {
            let coord = (column, row);
            if c == '\\' || c == '/' || c == '-' || c == '|' {
                data.insert(coord, String::from(c));
            }
        });
    });

    (data, width, height)
}

const TEST_INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_travel() {
        let coord = (0, 0);
        let (can, next_coord) = can_travel_next(10, 10, coord, BeamDirection::Left);
        assert_eq!(can, false);
        assert_eq!(next_coord, coord);

        let (can, next_coord) = can_travel_next(10, 10, coord, BeamDirection::Up);
        assert_eq!(can, false);
        assert_eq!(next_coord, coord);

        let (can, next_coord) = can_travel_next(10, 10, coord, BeamDirection::Right);
        assert_eq!(can, true);
        assert_eq!(next_coord, (1, 0));

        let (can, next_coord) = can_travel_next(10, 10, coord, BeamDirection::Down);
        assert_eq!(can, true);
        assert_eq!(next_coord, (0, 1));
    }

    #[test]
    fn test_reflected() {
        let start_beam = BeamDirection::Right;
        let result = mirror_reflect(start_beam, "-");
        assert_eq!(result, vec![BeamDirection::Right]);
        let result = mirror_reflect(start_beam, "/");
        assert_eq!(result, vec![BeamDirection::Up]);
        let result = mirror_reflect(start_beam, "\\");
        assert_eq!(result, vec![BeamDirection::Down]);
        let result = mirror_reflect(start_beam, "|");
        assert_eq!(result, vec![BeamDirection::Up, BeamDirection::Down]);

        let start_beam = BeamDirection::Left;
        let result = mirror_reflect(start_beam, "-");
        assert_eq!(result, vec![BeamDirection::Left]);
        let result = mirror_reflect(start_beam, "/");
        assert_eq!(result, vec![BeamDirection::Down]);
        let result = mirror_reflect(start_beam, "\\");
        assert_eq!(result, vec![BeamDirection::Up]);
        let result = mirror_reflect(start_beam, "|");
        assert_eq!(result, vec![BeamDirection::Up, BeamDirection::Down]);

        let start_beam = BeamDirection::Up;
        let result = mirror_reflect(start_beam, "-");
        assert_eq!(result, vec![BeamDirection::Left, BeamDirection::Right]);
        let result = mirror_reflect(start_beam, "/");
        assert_eq!(result, vec![BeamDirection::Right]);
        let result = mirror_reflect(start_beam, "\\");
        assert_eq!(result, vec![BeamDirection::Left]);
        let result = mirror_reflect(start_beam, "|");
        assert_eq!(result, vec![BeamDirection::Up]);

        let start_beam = BeamDirection::Down;
        let result = mirror_reflect(start_beam, "-");
        assert_eq!(result, vec![BeamDirection::Left, BeamDirection::Right]);
        let result = mirror_reflect(start_beam, "/");
        assert_eq!(result, vec![BeamDirection::Left]);
        let result = mirror_reflect(start_beam, "\\");
        assert_eq!(result, vec![BeamDirection::Right]);
        let result = mirror_reflect(start_beam, "|");
        assert_eq!(result, vec![BeamDirection::Down]);
    }

    #[test]
    fn test_light_travel() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (data, width, height) = parse_input(&lines);

        let result = process_light(&data, width, height, (0,0), BeamDirection::Right);
        assert_eq!(result.len(), 46);

        let lines: Vec<&str> = ".|...\\....".lines().collect();
        let (data, width, height) = parse_input(&lines);

        let result = process_light(&data, width, height, (0,0), BeamDirection::Right);
        assert_eq!(result.len(), 2);

        let lines: Vec<&str> = "..........".lines().collect();
        let (data, width, height) = parse_input(&lines);

        let result = process_light(&data, width, height, (0,0), BeamDirection::Right);
        assert_eq!(result.len(), width);
    }

    #[test]
    fn test_parse_input() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (data, width, height) = parse_input(&lines);
        let coord = (4, 1);
        assert_eq!(data.get(&coord).unwrap(), &"\\".to_owned());
        let coord = (5, 1);
        assert_eq!(data.get(&coord), None);

        assert_eq!(width, 10);
        assert_eq!(height, 10);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day16::part_01(&lines);
        assert_eq!(result, 46);

    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day16::part_02(&lines);
        assert_eq!(result, 51);
    }
}

fn print_direction_table(
    width: usize,
    height: usize,
    data: &HashMap<(usize, usize), String>,
    energized: &HashMap<(usize, usize), Vec<BeamDirection>>,
) {
    for row in 0..height {
        for column in 0..width {
            let coord = (column, row);

            if let Some(mirror) = data.get(&coord) {
                print!("{}", mirror);
            } else {
                if let Some(directions) = energized.get(&coord) {
                    if directions.len() == 1 {
                        if let Some(value) = directions.get(0) {
                            print!("{}", value.to_string());
                        }
                    } else {
                        print!("{}", directions.len());
                    }
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }

    // println!("---\n");
}

fn print_table(
    width: usize,
    height: usize,
    data: &HashMap<(usize, usize), String>,
    energized: &HashMap<(usize, usize), Vec<BeamDirection>>,
) {
    for row in 0..height {
        for column in 0..width {
            let coord = (column, row);
            if energized.contains_key(&coord) {
                print!("#");
            } else {
                if let Some(mirror) = data.get(&coord) {
                    print!("{}", mirror);
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }

    println!("---\n");
}
