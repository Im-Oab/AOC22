use std::{collections::{HashSet, VecDeque}, time::Instant};

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

    fn part_01(lines: &Vec<&str>) -> i32 {
        let (grid, start, end) = parsing(lines);
        println!("Start: {:?}\nTarget: {:?}\n\n", start, end);
        
        return 0;
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        return 0;
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
    assert_eq!(result, 0);
}

// TODO: continue implement dijkstra.
fn dijkstra(graph : &Vec<Vec<i32>>, total_nodes: i32, start: (i32,i32))
{
    let count = total_nodes as usize;
    let mut visitedVertex = vec![false; count];
    let mut distances = vec![i32::MAX; count];
    
    distances[]
    for i in 0..count
    {
        let u = find_min_distance(&distances, &visitedVertex);
        visitedVertex[u as usize] = true;

        for v in 0..count
        {
            if visitedVertex[v] == false && graph[u][v] != 0 && (distances[u] + graph[u][v] < distances[v]) {
                distances[v]
            }
        }
    }
}

fn find_min_distance(distances : &Vec<i32>, visited: &Vec<bool>) -> i32{
    let mut min_distance = i32::MAX;
    let mut min_distance_vertex = -1;
    for i in 0..distances.len()
    {
        if visited[i] == false && distances[i] < min_distance
        {
            min_distance = distances[i].clone();
            min_distance_vertex = i as i32;
        }
    }

    return min_distance_vertex;
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
    if let Some(columns) = grid.get(pos.1 as usize) {
        if let Some(value) = columns.get(pos.0 as usize) {
            return Some(*value);
        }
    }

    None
}

fn parsing(lines: &Vec<&str>) -> (Vec<Vec<i32>>, i32, (i32, i32), (i32, i32)) {
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

    (grid, width * height, start_position, target_position)
}
