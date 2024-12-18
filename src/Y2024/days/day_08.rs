use crate::file_handler::FileHandler;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day08 {}

impl Day08 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2024/inputs/day_08_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day08::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day08::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_08".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i32 {
        let (w, h, data) = parse_input(lines);
        let result = process_data(w, h, data);
        result.len() as i32
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let (w, h, data) = parse_input(lines);
        let result = process_data_with_resonant(w, h, data);
        result.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day08::part_01(&lines);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day08::part_02(&lines);
        assert_eq!(result, 34);
    }

    #[test]
    fn test_parse_input() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (width, height, data) = parse_input(&lines);
        assert_eq!(width, 12);
        assert_eq!(height, 12);
        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_find_signed_distance() {
        let (w, h) = find_signed_distance((0, 0), (1, 2));
        assert_eq!(w, 1);
        assert_eq!(h, 2);

        let (w, h) = find_signed_distance((1, 2), (0, 0));
        assert_eq!(w, -1);
        assert_eq!(h, -2);

        let (w, h) = find_signed_distance((5, 2), (8, 1));
        assert_eq!(w, 3);
        assert_eq!(h, -1);
    }

    #[test]
    fn test_find_anti_nodes_2() {
        let result = find_antinodes((4, 3), (5, 5));
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, 3);
        assert_eq!(result[0].1, 1);

        assert_eq!(result[1].0, 6);
        assert_eq!(result[1].1, 7);

        let result = find_antinodes((5, 2), (8, 1));
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].0, 2);
        assert_eq!(result[0].1, 3);

        assert_eq!(result[1].0, 11);
        assert_eq!(result[1].1, 0);

        let result = find_antinodes((3, 2), (5, 2));
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].0, 1);
        assert_eq!(result[0].1, 2);

        assert_eq!(result[1].0, 7);
        assert_eq!(result[1].1, 2);

        let result = find_antinodes((5, 2), (3, 2));
        assert_eq!(result.len(), 2);

        assert_eq!(result[1].0, 1);
        assert_eq!(result[1].1, 2);

        assert_eq!(result[0].0, 7);
        assert_eq!(result[0].1, 2);

        let result = find_antinodes((5, 2), (4, 4));
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].0, 6);
        assert_eq!(result[0].1, 0);

        assert_eq!(result[1].0, 3);
        assert_eq!(result[1].1, 6);
    }

    #[test]
    fn test_find_antinodes_with_resonant() {
        let result = find_antinodes_with_resonant((0, 0), (3, 1), 10, 10);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], (0, 0));
        assert_eq!(result[1], (3, 1));
        assert_eq!(result[2], (6, 2));
        assert_eq!(result[3], (9, 3));

        let result = find_antinodes_with_resonant((0, 0), (1, 2), 10, 10);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], (0, 0));
        assert_eq!(result[1], (1, 2));
        assert_eq!(result[2], (2, 4));
        assert_eq!(result[3], (3, 6));
        assert_eq!(result[4], (4, 8));

        let result = find_antinodes_with_resonant((3, 1), (1, 2), 10, 10);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], (3, 1));
        assert_eq!(result[1], (5, 0));
        assert_eq!(result[2], (1, 2));
    }
}

fn process_data(
    width: usize,
    height: usize,
    data: HashMap<String, HashSet<(i32, i32)>>,
) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();

    data.iter().for_each(|(frequency, antennas)| {
        antennas.clone().iter().for_each(|first| {
            antennas.iter().for_each(|second| {
                if first != second {
                    let nodes = find_antinodes(*first, *second);

                    nodes.iter().for_each(|location| {
                        if location.0 >= 0
                            && location.0 < width as i32
                            && location.1 >= 0
                            && location.1 < height as i32
                        {
                            result.insert((location.0, location.1));
                        }
                    });
                }
            });
        });
    });

    result
}

fn process_data_with_resonant(
    width: usize,
    height: usize,
    data: HashMap<String, HashSet<(i32, i32)>>,
) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();

    data.iter().for_each(|(frequency, antennas)| {
        antennas.clone().iter().for_each(|first| {
            antennas.iter().for_each(|second| {
                if first != second {
                    let nodes = find_antinodes_with_resonant(*first, *second, width, height);

                    nodes.iter().for_each(|location| {
                        if location.0 >= 0
                            && location.0 < width as i32
                            && location.1 >= 0
                            && location.1 < height as i32
                        {
                            result.insert((location.0, location.1));
                        }
                    });
                }
            });
        });
    });

    result
}

fn find_antinodes(first: (i32, i32), second: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = vec![];

    let (w, h) = find_signed_distance(first, second);

    result.push((first.0 - w, first.1 - h));
    result.push((second.0 + w, second.1 + h));

    result
}

fn find_antinodes_with_resonant(
    first: (i32, i32),
    second: (i32, i32),
    width: usize,
    height: usize,
) -> Vec<(i32, i32)> {
    let mut result = vec![];

    let (w, h) = find_signed_distance(first, second);

    let mut index = 0;
    // left
    loop {
        if first.0 - (w * index) < 0
            || first.1 - (h * index) < 0
            || first.0 - (w * index) >= width as i32
            || first.1 - (h * index) >= height as i32
        {
            break;
        }

        result.push((first.0 - w * index, first.1 - h * index));

        index += 1;
    }

    index = 0;
    // right
    loop {
        if second.0 + (w * index) < 0
            || second.1 + (h * index) < 0
            || second.0 + (w * index) >= width as i32
            || second.1 + (h * index) >= height as i32
        {
            break;
        }
        result.push((second.0 + w * index, second.1 + h * index));
        index += 1;
    }

    result
}

fn find_signed_distance(first: (i32, i32), second: (i32, i32)) -> (i32, i32) {
    ((second.0 - first.0), (second.1 - first.1))
}

fn parse_input(lines: &Vec<&str>) -> (usize, usize, HashMap<String, HashSet<(i32, i32)>>) {
    let height = lines.len();
    let width = lines.first().unwrap().len();
    let mut data = HashMap::new();

    lines.iter().enumerate().for_each(|(row, text)| {
        text.chars().enumerate().for_each(|(column, c)| match c {
            '.' => {}
            _ => {
                let key = c.to_string();
                if data.contains_key(&key) == false {
                    data.insert(key.to_owned(), HashSet::new());
                }

                if let Some(list) = data.get_mut(&key) {
                    list.insert((column as i32, row as i32));
                }
            }
        });
    });

    (width, height, data)
}

const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

const TEST_INPUT_2: &str = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";
