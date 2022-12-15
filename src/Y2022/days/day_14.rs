use std::{borrow::Borrow, time::Instant};

use hashbrown::HashMap;
use itertools::Itertools;

use crate::file_handler::FileHandler;

pub struct Day14 {}

impl Day14 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_14_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day14::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day14::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_14".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// Today's puzzle is fun. It is a sand-falling simulation. It is one of the games that I would like to make one day.
    fn part_01(lines: &Vec<&str>) -> usize {
        let scan_input = parsing(lines);

        let mut cave = Cave::new(1);
        cave.apply_scan(&scan_input);
        cave.set_sand_pouring(&(500, 0));

        loop {
            let fall_to_abyss = cave.simulate();

            if fall_to_abyss == true {
                break;
            }
        }
        // cave.draw();
        return cave.counting_sand();
    }

    fn part_02(lines: &Vec<&str>) -> usize {
        let scan_input = parsing(lines);

        let mut cave = Cave::new(1);
        cave.apply_scan(&scan_input);
        cave.set_sand_pouring(&(500, 0));

        loop {
            let fall_to_abyss = cave.simulate_with_floor();

            if fall_to_abyss == true {
                break;
            }
        }
        // cave.draw();
        return cave.counting_sand();
    }
}

const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day14::part_01(&lines);
    assert_eq!(result, 24);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day14::part_02(&lines);
    assert_eq!(result, 93);
}

#[derive(Clone, Debug)]
enum SpaceType {
    Air,
    Rock,
    Sand,
}
struct Cave {
    sand_pouring_points: Vec<(i32, i32)>,
    space: HashMap<(i32, i32), SpaceType>,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    total_active_sand: usize,
    active_sand: Vec<(i32, i32)>,
}

impl Cave {
    fn new(total_active_sand: usize) -> Self {
        Self {
            sand_pouring_points: vec![],
            space: HashMap::new(),
            left: i32::MAX,
            right: i32::MIN,
            top: i32::MAX,
            bottom: i32::MIN,
            total_active_sand: total_active_sand,
            active_sand: vec![],
        }
    }

    fn simulate_with_floor(&mut self) -> bool {
        let mut reach_starting_point = false;
        let produced_sand = self.produce_sand();
        while self.active_sand.len() > 0 && reach_starting_point == false {
            let mut new_sand = vec![];
            for (_, sand) in self.active_sand.clone().iter().enumerate() {
                let new_pos = self.sand_falling(&sand.clone());
                if new_pos.0 == sand.0 && new_pos.1 == sand.1 {
                    // sand doesn't move
                    self.assign_sand(sand);
                    if self.sand_pouring_points.contains(&new_pos) {
                        reach_starting_point = true;
                    }
                } else {
                    if new_pos.1 == self.bottom + 1 {
                        self.assign_sand(&new_pos);

                        if self.sand_pouring_points.contains(&new_pos) {
                            reach_starting_point = true;
                        }
                    } else {
                        new_sand.push(new_pos.clone());
                    }
                }
            }

            self.active_sand.clear();
            self.active_sand.append(&mut new_sand);
        }

        return reach_starting_point;
    }

    fn assign_sand(&mut self, point: &(i32, i32)) {
        self.space.insert(point.clone(), SpaceType::Sand);
        self.set_boundary(&(point.0, self.bottom));
    }

    fn simulate(&mut self) -> bool {
        let mut fall_to_abyss = false;
        let produced_sand = self.produce_sand();
        while self.active_sand.len() > 0 && fall_to_abyss == false {
            let mut new_sand = vec![];
            for (_, sand) in self.active_sand.clone().iter().enumerate() {
                let new_pos = self.sand_falling(&sand.clone());
                if new_pos.0 == sand.0 && new_pos.1 == sand.1 {
                    // sand doesn't move
                    self.space.insert(sand.clone(), SpaceType::Sand);
                } else {
                    new_sand.push(new_pos.clone());
                    if new_pos.1 > self.bottom {
                        // Fall to abyss
                        self.active_sand.clear();
                        fall_to_abyss = true;
                        break;
                    }
                }
            }

            self.active_sand.clear();
            self.active_sand.append(&mut new_sand);
        }

        return fall_to_abyss;
    }

    fn counting_sand(&self) -> usize {
        let mut count = 0;
        for (_, node) in self.space.iter() {
            if matches!(node, SpaceType::Sand) {
                count += 1;
            }
        }
        count
    }
    fn sand_falling(&self, point: &(i32, i32)) -> (i32, i32) {
        let expected_down = (point.0, point.1 + 1);
        let expected_down_left = (expected_down.0 - 1, expected_down.1);
        let expected_down_right = (expected_down.0 + 1, expected_down.1);

        if let Some(node) = self.space.get(&expected_down) {
            if matches!(node, SpaceType::Air) {
                return expected_down;
            }
        } else {
            return expected_down;
        }

        if let Some(node) = self.space.get(&expected_down_left) {
            if matches!(node, SpaceType::Air) {
                return expected_down_left;
            }
        } else {
            return expected_down_left;
        }

        if let Some(node) = self.space.get(&expected_down_right) {
            if matches!(node, SpaceType::Air) {
                return expected_down_right;
            }
        } else {
            return expected_down_right;
        }

        point.clone()
    }

    fn produce_sand(&mut self) -> bool {
        let mut produced_sand = false;
        let current_active_sand = self.active_sand.len();
        if current_active_sand < self.total_active_sand {
            let mut total_produce = self.total_active_sand - current_active_sand;
            let mut index = 0;
            while total_produce > 0 && index < self.sand_pouring_points.len() {
                let point =
                    self.sand_pouring_points[index % self.sand_pouring_points.len()].clone();
                if self.space.contains_key(&point) == false {
                    self.active_sand.push(point);
                    total_produce -= 1;
                    produced_sand = true;
                }
                index += 1;
            }
        }

        produced_sand
    }

    fn set_sand_pouring(&mut self, point: &(i32, i32)) {
        if self.sand_pouring_points.contains(point) == false {
            self.sand_pouring_points.push(point.clone());
            self.set_boundary(point);
        }
    }
    fn apply_scan(&mut self, scan_input: &Vec<Vec<(i32, i32)>>) {
        for path in scan_input.iter() {
            self.add_path(path);
        }
    }
    fn add_path(&mut self, path: &Vec<(i32, i32)>) {
        let mut starting = path[0].clone();
        for (index, target) in path.iter().enumerate() {
            if index == 0 {
                continue;
            }

            let points = self.get_points(&starting, &target);
            for point in points.iter() {
                self.space.insert(point.clone(), SpaceType::Rock);
                self.set_boundary(point);
            }

            starting = target.clone();
        }
    }

    fn get_points(&self, start: &(i32, i32), end: &(i32, i32)) -> Vec<(i32, i32)> {
        if start.0 == end.0 {
            let source = start.1.min(end.1);
            let dest = start.1.max(end.1);
            return (source..=dest).map(|v| (start.0, v)).collect_vec();
        } else if start.1 == end.1 {
            let source = start.0.min(end.0);
            let dest = start.0.max(end.0);
            return (source..=dest).map(|v| (v, start.1)).collect_vec();
        } else {
            panic!("Path isn't vertical or horizontal straight line");
        }
    }

    fn set_boundary(&mut self, point: &(i32, i32)) {
        if point.0 < self.left {
            self.left = point.0;
        }
        if point.0 > self.right {
            self.right = point.0;
        }

        if point.1 < self.top {
            self.top = point.1;
        }
        if point.1 > self.bottom {
            self.bottom = point.1;
        }
    }

    fn draw(&self) {
        for y in self.top..=self.bottom + 2 {
            for x in self.left..=self.right {
                let key = (x, y);
                match self.space.get(&key) {
                    Some(space_type) => match space_type {
                        SpaceType::Air => {
                            if self.sand_pouring_points.contains(&key)
                                || self.active_sand.contains(&key)
                            {
                                print!("+");
                            } else {
                                print!(".");
                            }
                        }
                        SpaceType::Rock => print!("#"),
                        SpaceType::Sand => print!("+"),
                    },
                    _ => {
                        if self.sand_pouring_points.contains(&key)
                            || self.active_sand.contains(&key)
                        {
                            print!("+");
                        } else {
                            print!(".");
                        }
                    }
                }
            }
            println!("");
        }
    }
}

fn parsing(lines: &Vec<&str>) -> Vec<Vec<(i32, i32)>> {
    let mut result = vec![];
    for line in lines.iter() {
        let no_space = line.replace(" ", "");
        let splited: Vec<&str> = no_space.split("->").collect();
        let path = splited
            .iter()
            .map(|v| {
                let (x, y) = (*v).split_once(",").unwrap();
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            })
            .collect_vec();

        result.push(path);
    }
    result
}
