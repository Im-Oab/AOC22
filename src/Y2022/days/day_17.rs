use std::{collections::VecDeque, time::Instant, vec};

use itertools::Itertools;

use crate::file_handler::FileHandler;

pub struct Day17 {}

impl Day17 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_17_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day17::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day17::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_17".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i32 {
        let pattern = parsing(lines);

        let chamber = VerticalChamber::new(&pattern);
        chamber.print();

        return 0;
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        return 0;
    }
}

const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day17::part_01(&lines);
    assert_eq!(result, 0);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day17::part_02(&lines);
    assert_eq!(result, 0);
}

enum Blocks {
    Horizontal = 1,
    Plus = 2,
    InverseL = 3,
    Vertical = 4,
    Square = 5,
}

struct VerticalChamber {
    width: i32,
    rows: VecDeque<[bool; 7]>,
    current_block: Option<(Blocks, i32, i32)>,
    jet_pattern: Vec<bool>,
    current_pattern_index: usize,
}

impl VerticalChamber {
    fn new(pattern: &Vec<bool>) -> Self {
        Self {
            width: 7,
            rows: VecDeque::new(),
            current_block: None,
            jet_pattern: pattern.clone(),
            current_pattern_index: 0,
        }
    }

    fn spawn_point(&self, block_type: &Blocks) -> (i32, i32) {
        let (_, h) = block_size(block_type);
        (2, (self.rows.len() as i32) + h - 1)
    }

    fn process(&mut self) {}

    fn print(&self) {
        let mut top = self.rows.len() as i32 + 3 - 1;
        // if let Some((x1, y1, x2, y2)) = self.current_bounding_block() {
        //     if top < y1 {
        //         top = y1;
        //     }
        // }

        // for y in (0..=top).rev()
        // {
        //     print!("|");
        //     if y < self.rows.len() as i32
        //     {
        //         // draw existing area
        //         if let Some(row) = self.rows.get(y as usize)
        //         {

        //         }
        //     }
        //     println!("|");
        // }

        // for (_, row) in self.rows.iter().rev().enumerate() {
            
        //     for v in row.iter() {
        //         if *v == true {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
            
        // }

        println!("+-------+");
    }
}

fn parsing(lines: &Vec<&str>) -> Vec<bool> {
    lines
        .first()
        .unwrap()
        .chars()
        .map(|c| {
            if c == '>' {
                return true;
            } else {
                return false;
            }
        })
        .collect_vec()
}

fn block_size(block: &Blocks) -> (i32, i32) {
    match block {
        Blocks::Horizontal => (4, 1),
        Blocks::Plus => (3, 3),
        Blocks::InverseL => (3, 3),
        Blocks::Vertical => (1, 4),
        Blocks::Square => (2, 2),
    }
}
