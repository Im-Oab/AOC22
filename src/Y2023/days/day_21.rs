use std::time::Instant;
use std::collections::{HashMap, HashSet};
use rayon::{prelude::*, result};
use crate::file_handler::FileHandler;

pub struct Day21 {}

impl Day21 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_21_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day21::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day21::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_21".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> usize {
        let (grids, start_coord, width, height) = parse_input(lines);
        step(&grids, start_coord, width, height, 64)
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        return 0;
    }
}

#[derive(Debug,Clone, PartialEq)]
enum SpaceType
{
    PlotGarden,
    Rock

}

fn step(grids: &HashMap<(i32,i32), SpaceType>, start_coord: (i32,i32), width: i32, height: i32, total_steps: i32) -> usize
{
    let mut visited = HashSet::new();
    let mut odd_visited = HashSet::new();
    let mut even_visited = HashSet::new();
    let mut new_nodes = HashSet::new();
    new_nodes.insert(start_coord);
    for step in 1..=total_steps
    {
        let mut next_step_nodes = HashSet::new();
        new_nodes.iter().for_each(|coord|{
        if visited.contains(coord) == false
        {
            visited.insert(*coord);
            let neighbours = get_neighbours(grids, width, height, *coord);
            neighbours.iter().for_each(|next_coord|{
                if step % 2 == 0
                {
                    if even_visited.contains(next_coord) == false
                    {
                        even_visited.insert(*next_coord);
                        next_step_nodes.insert(*next_coord);
                    }
                }
                else
                {
                    if odd_visited.contains(next_coord) == false
                    {
                        odd_visited.insert(*next_coord);
                        next_step_nodes.insert(*next_coord);
                    }
                }
            });
        }
        });

        new_nodes.clear();
        new_nodes = next_step_nodes;
    }

    if total_steps % 2 == 0
    {
        even_visited.len()
    }
    else
    {
        odd_visited.len()
    }
}

fn step_infinity(grids: &HashMap<(i32,i32), SpaceType>, start_coord: (i32,i32), width: i32, height: i32, total_steps: i32) -> usize
{
    let mut visited = HashSet::new();
    let mut odd_visited = HashSet::new();
    let mut even_visited = HashSet::new();
    let mut new_nodes = HashSet::new();
    new_nodes.insert(start_coord);
    for step in 1..=total_steps
    {
        let mut next_step_nodes = HashSet::new();
        new_nodes.iter().for_each(|coord|{
        if visited.contains(coord) == false
        {
            visited.insert(*coord);
            let neighbours = get_neighbours_infinity(grids, width, height, *coord);
            neighbours.iter().for_each(|next_coord|{
                if step % 2 == 0
                {
                    if even_visited.contains(next_coord) == false
                    {
                        even_visited.insert(*next_coord);
                        next_step_nodes.insert(*next_coord);
                    }
                }
                else
                {
                    if odd_visited.contains(next_coord) == false
                    {
                        odd_visited.insert(*next_coord);
                        next_step_nodes.insert(*next_coord);
                    }
                }
            });
        }
        });

        new_nodes.clear();
        new_nodes = next_step_nodes;
    }

    if total_steps % 2 == 0
    {
        even_visited.len()
    }
    else
    {
        odd_visited.len()
    }
}

fn get_neighbours(grids: &HashMap<(i32,i32), SpaceType>, width: i32, height: i32, coord: (i32,i32)) -> Vec<(i32,i32)>
{
    let mut result = vec![];
    // north
    if coord.1 > 0
    {
        let next_coord = (coord.0, coord.1 - 1);
        if grids.contains_key(&next_coord) == false
        {
            result.push(next_coord);
        }
    }

    // south
    if coord.1 < height - 1
    {
        let next_coord = (coord.0, coord.1 + 1);
        if grids.contains_key(&next_coord) == false
        {
            result.push(next_coord);
        }
    }

    // west
    if coord.0 > 0
    {
        let next_coord = (coord.0 - 1, coord.1);
        if grids.contains_key(&next_coord) == false
        {
            result.push(next_coord);
        }
    }

    // east
    if coord.0 < width - 1
    {
        let next_coord = (coord.0 + 1, coord.1);
        if grids.contains_key(&next_coord) == false
        {
            result.push(next_coord);
        }
    }

    result
}

fn convert_to_infinity(coord: (i32,i32), width: i32, height: i32) -> (i32,i32)
{
    let mut column = coord.0;
    let mut row = coord.1;
    if column < 0
    {
        column = width - (column.abs() % width);
    }
    else
    {
        column = column % width;
    }

    if row < 0
    {
        row = height - (row.abs() % height);
    }
    else
    {
        row = row % height;
    }

    (column, row)
}

fn get_neighbours_infinity(grids: &HashMap<(i32,i32), SpaceType>, width: i32, height: i32, coord: (i32,i32)) -> Vec<(i32,i32)>
{
    let mut result = vec![];
    // north
    {
        let next_coord = (coord.0, coord.1 - 1);
        let grid_coord: (i32, i32) = convert_to_infinity(next_coord, width, height);
        if grids.contains_key(&grid_coord) == false
        {
            result.push(next_coord);
        }
    }

    // south
    {
        let next_coord = (coord.0, coord.1 + 1);
        let grid_coord: (i32, i32) = convert_to_infinity(next_coord, width, height);
        if grids.contains_key(&grid_coord) == false
        {
            result.push(next_coord);
        }
    }

    // west
    {
        let next_coord = (coord.0 - 1, coord.1);
        let grid_coord: (i32, i32) = convert_to_infinity(next_coord, width, height);
        if grids.contains_key(&grid_coord) == false
        {
            result.push(next_coord);
        }
    }

    // east
    {
        let next_coord = (coord.0 + 1, coord.1);
        let grid_coord: (i32, i32) = convert_to_infinity(next_coord, width, height);
        if grids.contains_key(&grid_coord) == false
        {
            result.push(next_coord);
        }
    }

    result
}
fn parse_input(input: &Vec<&str>) -> (HashMap<(i32,i32), SpaceType>, (i32,i32), i32, i32)
{
    let height = input.len();
    let mut width = 0;
    let mut start_coord = (0,0);
    let data: Vec<Vec<((i32,i32), SpaceType)>> = input.iter().enumerate().map(|(row, column_data)|{
        width = column_data.len();
        column_data.chars().enumerate().filter_map(|(column, value)|{
            let coord = (column as i32, row as i32);
            
            match value
            {
                '.' => {
                    None
                }
                '#' => {
                    Some((coord, SpaceType::Rock))
                }
                'S' => {
                    start_coord = coord;
                    None
                }
                _ => unreachable!("It should not be here")
            }
        }).collect()
    }).collect();

    let mut hash : HashMap<(i32,i32), SpaceType> = HashMap::new();
    data.iter().for_each(|list|{
        list.iter().for_each(|(coord, value)|{
            hash.insert(*coord, value.clone());
        });
    });

    (hash, start_coord, width as i32, height as i32)
}

const TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_infinity()
    {
        let input: Vec<&str> = TEST_INPUT.lines().collect();
        let (grids, start_coord, width, height) = parse_input(&input);
        let total_steps = step_infinity(&grids, start_coord, width, height, 1);
        assert_eq!(total_steps, 2);

        let total_steps = step_infinity(&grids, start_coord, width, height, 2);
        assert_eq!(total_steps, 4);

        let total_steps = step_infinity(&grids, start_coord, width, height, 6);
        assert_eq!(total_steps, 16);

        let total_steps = step_infinity(&grids, start_coord, width, height, 10);
        assert_eq!(total_steps, 50);
        

        let total_steps = step_infinity(&grids, start_coord, width, height, 50);
        assert_eq!(total_steps, 1594);

        // let total_steps = step_infinity(&grids, start_coord, width, height, 100);
        // assert_eq!(total_steps, 6536);

        // let total_steps = step_infinity(&grids, start_coord, width, height, 1000);
        // assert_eq!(total_steps, 668697);

        // let total_steps = step_infinity(&grids, start_coord, width, height, 500);
        // assert_eq!(total_steps, 167004);

        let total_steps = step_infinity(&grids, start_coord, width, height, 5000);
        assert_eq!(total_steps, 16733044);
    }

    #[test]
    fn test_step()
    {
        let input: Vec<&str> = TEST_INPUT.lines().collect();
        let (grids, start_coord, width, height) = parse_input(&input);
        let total_steps = step(&grids, start_coord, width, height, 1);
        assert_eq!(total_steps, 2);

        let total_steps = step(&grids, start_coord, width, height, 2);
        assert_eq!(total_steps, 4);

        let total_steps = step(&grids, start_coord, width, height, 6);
        assert_eq!(total_steps, 16);
    }

    #[test]
    fn test_parse_input()
    {
        let input: Vec<&str> = TEST_INPUT.lines().collect();
        let (hash, start_coord, width, height) = parse_input(&input);
        assert_eq!(start_coord, (5, 5));
        assert_eq!(hash.len(), 40);
        assert_eq!(width, 11);
        assert_eq!(height, 11);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day21::part_01(&lines);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day21::part_02(&lines);
        assert_eq!(result, 2286);
    }
}

