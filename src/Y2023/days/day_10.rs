use std::{collections::VecDeque, time::Instant};

use geo::{polygon, Contains, LineString, Point, Polygon};
use hashbrown::{HashMap, HashSet};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::*;

use crate::file_handler::FileHandler;

pub struct Day10 {}

impl Day10 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_10_1.txt");

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

    fn part_01(lines: &Vec<&str>) -> i32 {
        let grid = Grids::new(lines);
        return grid.process();
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let mut grid = Grids::new(lines);
        grid.travel_pipes();

        grid.stray_pipes
            .par_iter()
            .map(|coord| if grid.is_enclose(*coord) { 1 } else { 0 })
            .sum()
    }
}

struct Grids {
    width: usize,
    height: usize,
    starting_point: (i32, i32),
    grids: HashMap<(i32, i32), GridUnit>,

    polygon: Option<Polygon<i32>>,
    connected_pipes: Vec<(i32, i32)>,
    stray_pipes: Vec<(i32, i32)>,
}

impl Grids {
    fn process(&self) -> i32 {
        let mut visited = HashSet::new();
        let mut distance = HashMap::new();
        let mut queue = VecDeque::new();
        visited.insert(self.starting_point);
        queue.push_back(self.starting_point);
        distance.insert(self.starting_point, 0);

        loop {
            if let Some(coordinate) = queue.pop_front() {
                let current_distance = distance.get(&coordinate).unwrap().clone();

                // handle neighbours
                let current_node = self.grids.get(&coordinate).unwrap();

                if current_node.connected_north() == true {
                    let next_coordinate = (coordinate.0, coordinate.1 - 1);
                    if self.grids.contains_key(&next_coordinate) == true
                        && visited.contains(&next_coordinate) == false
                    {
                        queue.push_back(next_coordinate);
                        visited.insert(next_coordinate);
                        distance.insert(next_coordinate, current_distance + 1);
                    }
                }

                if current_node.connected_south() == true {
                    let next_coordinate = (coordinate.0, coordinate.1 + 1);
                    if self.grids.contains_key(&next_coordinate) == true
                        && visited.contains(&next_coordinate) == false
                    {
                        queue.push_back(next_coordinate);
                        visited.insert(next_coordinate);
                        distance.insert(next_coordinate, current_distance + 1);
                    }
                }

                if current_node.connected_east() == true {
                    let next_coordinate = (coordinate.0 + 1, coordinate.1);
                    if self.grids.contains_key(&next_coordinate) == true
                        && visited.contains(&next_coordinate) == false
                    {
                        queue.push_back(next_coordinate);
                        visited.insert(next_coordinate);
                        distance.insert(next_coordinate, current_distance + 1);
                    }
                }

                if current_node.connected_west() == true {
                    let next_coordinate = (coordinate.0 - 1, coordinate.1);
                    if self.grids.contains_key(&next_coordinate) == true
                        && visited.contains(&next_coordinate) == false
                    {
                        queue.push_back(next_coordinate);
                        visited.insert(next_coordinate);
                        distance.insert(next_coordinate, current_distance + 1);
                    }
                }
            } else {
                break;
            }
        }

        distance.iter().map(|(_, value)| *value).max().unwrap()
    }

    fn travel_pipes(&mut self) {
        let mut visited = vec![];
        let mut queue = VecDeque::new();
        visited.push(self.starting_point);
        queue.push_back(self.starting_point);

        loop {
            if let Some(coordinate) = queue.pop_front() {
                // handle neighbours
                let current_node = self.grids.get(&coordinate).unwrap();

                if current_node.connected_north() == true {
                    let next_coordinate = (coordinate.0, coordinate.1 - 1);
                    if self.grids.contains_key(&next_coordinate) == true
                        && visited.contains(&next_coordinate) == false
                    {
                        queue.push_back(next_coordinate);
                        visited.push(next_coordinate);
                        continue;
                    }
                }

                if current_node.connected_south() == true {
                    let next_coordinate = (coordinate.0, coordinate.1 + 1);
                    if self.grids.contains_key(&next_coordinate) == true
                        && visited.contains(&next_coordinate) == false
                    {
                        queue.push_back(next_coordinate);
                        visited.push(next_coordinate);
                        continue;
                    }
                }

                if current_node.connected_east() == true {
                    let next_coordinate = (coordinate.0 + 1, coordinate.1);
                    if self.grids.contains_key(&next_coordinate) == true
                        && visited.contains(&next_coordinate) == false
                    {
                        queue.push_back(next_coordinate);
                        visited.push(next_coordinate);
                        continue;
                    }
                }

                if current_node.connected_west() == true {
                    let next_coordinate = (coordinate.0 - 1, coordinate.1);
                    if self.grids.contains_key(&next_coordinate) == true
                        && visited.contains(&next_coordinate) == false
                    {
                        queue.push_back(next_coordinate);
                        visited.push(next_coordinate);
                        continue;
                    }
                }
            } else {
                break;
            }
        }
        visited.push(self.starting_point);

        let polygon = Polygon::new(LineString::from(visited.clone()), vec![]);
        self.polygon = Some(polygon);

        let stray_pipes: Vec<(i32, i32)> = self
            .grids
            .iter_mut()
            .filter_map(|(coordinate, _)| {
                if visited.contains(coordinate) == false {
                    Some(coordinate.clone())
                } else {
                    None
                }
            })
            .collect();

        self.connected_pipes = visited;
        self.stray_pipes = stray_pipes;
    }

    fn new(input: &Vec<&str>) -> Self {
        let height = input.len();
        let width = input[0].len();
        let mut units = HashMap::new();
        let result: Vec<Vec<GridUnit>> = input
            .par_iter()
            .enumerate()
            .map(|(row, line)| {
                let row = row;
                let columns: Vec<GridUnit> = (*line)
                    .chars()
                    .enumerate()
                    .map(|(column, c)| GridUnit::from(&String::from(c), column as i32, row as i32))
                    .collect();

                columns
            })
            .collect();

        let mut starting_point = (0, 0);
        result.iter().for_each(|list| {
            list.iter().for_each(|unit| {
                if unit.starting_point == true {
                    starting_point = unit.coordinate.clone();
                }

                let coord = unit.coordinate.clone();
                let value = unit.clone();

                units.insert(coord, value);
            });
        });

        if let Some(mut node) = units.get_mut(&starting_point).cloned() {
            node.connected = predict_possible_pipe(node.coordinate, &units);
            units.insert(starting_point, node);
        }

        Self {
            width: width,
            height: height,
            starting_point: starting_point,
            grids: units,
            connected_pipes: vec![],
            stray_pipes: vec![],
            polygon: None,
        }
    }

    fn get_starting_point(&self) -> Option<&GridUnit> {
        self.grids.get(&self.starting_point)
    }

    fn is_enclose(&self, coordinate: (i32, i32)) -> bool {
        let (x1, y1) = coordinate;
        if let Some(poly) = self.polygon.as_ref() {
            return poly.contains(&Point::new(x1, y1));
        }

        false
    }
}

fn predict_possible_pipe(
    coordinate: (i32, i32),
    grids: &HashMap<(i32, i32), GridUnit>,
) -> [bool; 4] {
    let mut result = [false, false, false, false];
    // North
    if let Some(node) = grids.get(&(coordinate.0, coordinate.1 - 1)) {
        result[0] = node.connected_south();
    }
    // South
    if let Some(node) = grids.get(&(coordinate.0, coordinate.1 + 1)) {
        result[2] = node.connected_north();
    }
    // East
    if let Some(node) = grids.get(&(coordinate.0 + 1, coordinate.1)) {
        result[1] = node.connected_west();
    }
    // West
    if let Some(node) = grids.get(&(coordinate.0 - 1, coordinate.1)) {
        result[3] = node.connected_east();
    }

    result
}

#[derive(Debug, Clone)]
struct GridUnit {
    coordinate: (i32, i32),
    /// North, East, South, West
    connected: [bool; 4],

    starting_point: bool,
}

impl GridUnit {
    pub fn from(value: &str, column: i32, row: i32) -> Self {
        let connected = match value {
            "|" => [true, false, true, false],
            "-" => [false, true, false, true],
            "L" => [true, true, false, false],
            "J" => [true, false, false, true],
            "7" => [false, false, true, true],
            "F" => [false, true, true, false],
            _ => [false, false, false, false],
        };

        let starting_point = value == "S";

        Self {
            coordinate: (column, row),
            connected: connected,
            starting_point: starting_point,
        }
    }

    fn connected_south(&self) -> bool {
        self.connected[2]
    }

    fn connected_north(&self) -> bool {
        self.connected[0]
    }

    fn connected_east(&self) -> bool {
        self.connected[1]
    }

    fn connected_west(&self) -> bool {
        self.connected[3]
    }
}

const TEST_INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enclose() {
        let input: Vec<&str> = TEST_INPUT.lines().collect();
        let mut grid = Grids::new(&input);
        grid.travel_pipes();

        for coord in grid.stray_pipes.iter() {
            assert_eq!(grid.is_enclose(*coord), false);
        }

        assert_eq!(grid.is_enclose((2, 2)), true);
    }

    #[test]
    fn test_parsing_input() {
        let input: Vec<&str> = TEST_INPUT.lines().collect();
        let grid = Grids::new(&input);
        assert_eq!(grid.starting_point, (1, 1));

        if let Some(node) = grid.get_starting_point() {
            assert_eq!(node.connected_east(), true);
            assert_eq!(node.connected_south(), true);
            assert_eq!(node.connected_north(), false);
            assert_eq!(node.connected_west(), false);
        }
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day10::part_01(&lines);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day10::part_02(&lines);
        assert_eq!(result, 1);
    }
}
