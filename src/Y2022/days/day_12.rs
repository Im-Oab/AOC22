use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use crate::file_handler::FileHandler;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct Day12 {}

impl Day12 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_12_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day12::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day12::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_12".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// After reading the puzzle, I knew it needed to do the shortest path, Instead of using the existing algorithms.
    /// I wanted to implement my own. It took me 2 hours, and it ended up a failure. :')
    /// In the end, I implemented the BFS to solve the puzzle.
    fn part_01(lines: &Vec<&str>) -> usize {
        let (grid, width, height, start, end) = parsing(lines);

        let parents = bfs(&grid, width as usize * height as usize, start.clone());

        let start_index = start.1 * width + start.0;
        let end_index = end.1 * width + end.0;
        let path = construct_path(
            parents,
            width as usize,
            start_index as usize,
            end_index as usize,
        );

        return path.1.len();
    }

    /// Part 2 is quite easy, instead of the initial starting point. It just needs to find the best starting point.
    /// So, I modified the code from part 1 and did a brute-force by searching for the best one by calculating the shortest part of all nodes.
    fn part_02(lines: &Vec<&str>) -> usize {
        let (grid, width, height, _, end) = parsing(lines);
        let starting_points = find_starting_points(&grid);

        let mut min_steps = usize::MAX;
        for start in starting_points.iter() {
            let parents = bfs(&grid, width as usize * height as usize, start.clone());

            let start_index = start.1 * width + start.0;
            let end_index = end.1 * width + end.0;
            let path = construct_path(
                parents,
                width as usize,
                start_index as usize,
                end_index as usize,
            );
            if path.0 == true && min_steps > path.1.len() {
                min_steps = path.1.len();
            }
        }

        return min_steps;
    }
}

const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day12::part_01(&lines);
    assert_eq!(result, 31);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day12::part_02(&lines);
    assert_eq!(result, 29);
}

fn find_starting_points(grid: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut result = vec![];
    let a = 'a' as i32;
    let mut pos = (0, 0);
    for row in grid.iter() {
        pos.0 = 0;
        for value in row.iter() {
            if *value == a {
                result.push(pos.clone());
            }
            pos.0 += 1;
        }
        pos.1 += 1;
    }

    result
}

fn construct_path(
    parents: Vec<Option<(i32, i32)>>,
    width: usize,
    start: usize,
    end: usize,
) -> (bool, Vec<(i32, i32)>) {
    let mut path = vec![];
    let mut current_node = parents[end];
    let mut found_start_node = false;

    loop {
        match current_node {
            Some((x, y)) => {
                path.push((x, y));

                let index = y as usize * width + x as usize;
                current_node = parents[index];

                if start == index {
                    found_start_node = true;
                }
            }
            None => break,
        }
    }

    (found_start_node, path)
}

fn bfs(grid: &Vec<Vec<i32>>, total_nodes: usize, start: (i32, i32)) -> Vec<Option<(i32, i32)>> {
    let width = total_nodes / grid.len();
    let height = grid.len();
    let start_index = start.1 as usize * width + start.0 as usize;

    let mut queue = VecDeque::new();
    let mut visited = vec![false; total_nodes];
    visited[start_index] = true;

    let mut parents: Vec<Option<(i32, i32)>> = vec![None; total_nodes];
    let mut total_visited = 1;
    queue.push_back(start);
    while queue.len() > 0 {
        let current_node = queue.pop_front().unwrap();
        let neighbours = get_neighbours(grid, current_node.clone());
        for (x, y) in neighbours.iter() {
            let index = (*y as usize * width) + *x as usize;
            if visited[index] == false {
                visited[index] = true;
                parents[index] = Some(current_node.clone());
                queue.push_back((*x, *y));

                total_visited += 1;
            }
        }
    }

    parents
}

fn get_neighbours(grid: &Vec<Vec<i32>>, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = vec![];
    if let Some(current_value) = grid_value(grid, pos) {
        for direction in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        {
            let next_pos = get_next_position(pos, direction.clone());
            if let Some(value) = grid_value(grid, next_pos) {
                if current_value == value || current_value + 1 == value || current_value > value {
                    result.push(next_pos);
                }
            }
        }
    }

    result
}

fn get_next_position(pos: (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn grid_value(grid: &Vec<Vec<i32>>, pos: (i32, i32)) -> Option<i32> {
    // grid[y][x]
    if let Some(row) = grid.get(pos.1 as usize) {
        if let Some(value) = row.get(pos.0 as usize) {
            return Some(*value);
        }
    }

    None
}

fn parsing(lines: &Vec<&str>) -> (Vec<Vec<i32>>, i32, i32, (i32, i32), (i32, i32)) {
    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;

    let mut grid = vec![];
    let mut pos = (0, 0);
    let mut start_position = (0, 0);
    let mut target_position = (0, 0);
    for line in lines.iter() {
        pos.0 = 0;
        let mut columns = vec![];
        for c in line.chars() {
            match c {
                'S' => {
                    columns.push('a' as i32);
                    start_position = pos.clone();
                }
                'E' => {
                    columns.push('z' as i32);
                    target_position = pos.clone();
                }
                _ => {
                    columns.push(c as i32);
                }
            }

            pos.0 += 1;
        }

        grid.push(columns);
        pos.1 += 1;
    }

    (grid, width, height, start_position, target_position)
}
