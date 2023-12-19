use crate::file_handler::FileHandler;
use geo::coord;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

pub struct Day17 {}

impl Day17 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_17_1.txt");

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
        return 0;
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        return 0;
    }
}

fn parse_input(
    input: &Vec<&str>,
) -> (
    usize,
    usize,
    HashMap<(usize, usize), i32>,
    Vec<Vec<((usize, usize), i32)>>,
) {
    let height = input.len();
    let mut width = 0;
    let data: Vec<Vec<((usize, usize), i32)>> = (input)
        .par_iter()
        .enumerate()
        .map(|(row, columns_data)| {
            let row: Vec<((usize, usize), i32)> = columns_data
                .chars()
                .enumerate()
                .map(|(column, c)| {
                    let coord = (column, row);
                    if c.is_digit(10) {
                        (coord, c.to_digit(10).unwrap() as i32)
                    } else {
                        unreachable!("It should not be here");
                    }
                })
                .collect();

            row
        })
        .collect();

    let mut grid = HashMap::new();
    data.iter().for_each(|list| {
        width = list.len();
        list.iter().for_each(|(coord, value)| {
            grid.insert(coord.clone(), *value);
        });
    });

    (width, height, grid, data)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TravelDirection {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    None,
}

impl TravelDirection {
    fn get_value(&self) -> i32 {
        match self {
            TravelDirection::Up(value) => *value,
            TravelDirection::Down(value) => *value,
            TravelDirection::Left(value) => *value,
            TravelDirection::Right(value) => *value,
            TravelDirection::None => 0,
        }
    }

    fn to_string(&self) -> String {
        match self {
            TravelDirection::Up(value) => "U".to_owned(),
            TravelDirection::Down(value) => "D".to_owned(),
            TravelDirection::Left(value) => "L".to_owned(),
            TravelDirection::Right(value) => "R".to_owned(),
            TravelDirection::None => "".to_owned(),
        }
    }
}

fn dijkstra(
    width: usize,
    height: usize,
    grids: &HashMap<(usize, usize), i32>,
) -> (
    HashMap<(usize, usize), i32>,
    HashMap<(usize, usize), (usize, usize)>,
) {
    let starting_point = (0, 0);
    let mut shortest_path = vec![];
    let mut heat_loss: HashMap<(usize, usize), i32> = HashMap::new();
    let mut directions = HashMap::new();
    let mut paths: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    heat_loss.insert(starting_point, 0);
    directions.insert(starting_point, TravelDirection::None);
    loop {
        let mut candidate: Vec<((usize, usize), i32)> = heat_loss
            .iter()
            .filter_map(|(coord, value)| {
                if shortest_path.contains(coord) == false {
                    Some((*coord, *value))
                } else {
                    None
                }
            })
            .collect();

        candidate.sort_by(|a, b| a.1.cmp(&b.1));

        if let Some((current_coord, current_heat_loss)) = candidate.first() {
            shortest_path.push(*current_coord);
            if let Some(travel_direction) = directions.get(current_coord) {
                let neighbours =
                    find_neighbour(width, height, grids, *current_coord, *travel_direction);

                neighbours
                    .iter()
                    .for_each(|(next_coord, next_direction, next_heat_loss)| {
                        let (combined_heat_lost, direction) =
                            if let Some(existing_heat_loss) = heat_loss.get(next_coord) {
                                if current_heat_loss + next_heat_loss <= *existing_heat_loss {
                                    // update with a new heat loss value
                                    (current_heat_loss + next_heat_loss, *next_direction)
                                } else {
                                    return;
                                }
                            } else {
                                (current_heat_loss + next_heat_loss, *next_direction)
                            };

                        paths.insert(*next_coord, *current_coord);
                        heat_loss.insert(*next_coord, combined_heat_lost);
                        directions.insert(*next_coord, direction);
                    });
            } else {
                break;
            }

            print_paths_direction(width, height, &heat_loss, &paths, &directions);
        } else {
            break;
        }
    }

    (heat_loss, paths)
}

fn travel(
    width: usize,
    height: usize,
    grids: &HashMap<(usize, usize), i32>,
) -> (
    HashMap<(usize, usize), i32>,
    HashMap<(usize, usize), (usize, usize)>,
    HashMap<(usize, usize), TravelDirection>,
) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut heat_loss: HashMap<(usize, usize), i32> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut directions: HashMap<(usize, usize), TravelDirection> = HashMap::new();
    // (Destination -> Source)
    let mut paths: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let starting_point = (0, 0);

    queue.push_back((starting_point, TravelDirection::None));
    heat_loss.insert(starting_point, 0);
    visited.insert(starting_point);
    directions.insert(starting_point, TravelDirection::None);
    loop {
        if queue.len() == 0 {
            break;
        }

        let mut unsorted_data: Vec<(i32, usize)> = queue
            .iter()
            .enumerate()
            .map(|(index, (coord, direction))| {
                let heat_loss = heat_loss.get(coord).copied().unwrap_or(0);
                (heat_loss, index)
            })
            .collect();
        unsorted_data.sort_by(|a, b| a.0.cmp(&b.0));

        let index = unsorted_data.first().unwrap();

        if let Some((current_coord, _)) = queue.remove(index.1) {
            let current_direction = directions
                .get(&current_coord)
                .copied()
                .unwrap_or(TravelDirection::None);
            let neighbours = find_neighbour(width, height, grids, current_coord, current_direction);
            let current_heat_loss = heat_loss.get(&current_coord).copied().unwrap();

            neighbours
                .iter()
                .for_each(|(next_coord, next_direction, next_heat_loss)| {
                    let combined_heat_loss = current_heat_loss + next_heat_loss;
                    if *next_coord == (11, 7) {
                        println!("{:?} = {}", next_coord, combined_heat_loss);
                    }

                    // new node here
                    if visited.contains(next_coord) == false {
                        visited.insert(*next_coord);
                        heat_loss.insert(*next_coord, combined_heat_loss);
                        paths.insert(*next_coord, current_coord);
                        queue.push_back((*next_coord, *next_direction));
                        directions.insert(*next_coord, *next_direction);
                    } else {
                        // check for better path with less heat_loss
                        if let Some(visited_heat_loss) = heat_loss.get(next_coord) {
                            if combined_heat_loss <= *visited_heat_loss {
                                // It need to check that it cant travel more than 3 blocks with the same direction here
                                if let Some(travel_direction) = directions.get(next_coord) {
                                    if std::mem::discriminant(travel_direction)
                                        == std::mem::discriminant(next_direction)
                                    {
                                        if travel_direction.get_value() + next_direction.get_value()
                                            >= 3
                                        {
                                            return;
                                        }
                                    }
                                }

                                heat_loss.insert(*next_coord, combined_heat_loss);
                                paths.insert(*next_coord, current_coord);
                                queue.push_back((*next_coord, *next_direction));
                                directions.insert(*next_coord, *next_direction);
                            }
                        }
                    }
                });

            print_paths_direction(width, height, &heat_loss, &paths, &directions);
        } else {
            break;
        }
    }

    (heat_loss, paths, directions)
}

fn print_paths_direction(
    width: usize,
    height: usize,
    grids: &HashMap<(usize, usize), i32>,
    paths: &HashMap<(usize, usize), (usize, usize)>,
    directions: &HashMap<(usize, usize), TravelDirection>,
) -> i32 {
    let mut next_path = (width - 1, height - 1);
    let mut total_heat_loss_values = grids.get(&next_path).copied().unwrap_or(0);
    let mut shortest_path = vec![];
    shortest_path.push(next_path);

    loop {
        if let Some(path) = paths.get(&next_path) {
            shortest_path.push(*path);
            next_path = *path;
            total_heat_loss_values += grids.get(&next_path).copied().unwrap_or(0);
        } else {
            break;
        }
    }

    for row in 0..height {
        for column in 0..width {
            let coord = (column, row);
            if shortest_path.contains(&coord) == true {
                if let Some(direction) = directions.get(&coord) {
                    print!(" [{}{}]", direction.to_string(), direction.get_value());
                } else if let Some(value) = grids.get(&coord) {
                    print!("[{:>3}]", value);
                } else {
                    print!("   # ");
                }
            } else if let Some(value) = grids.get(&coord) {
                if let Some(direction) = directions.get(&coord) {
                    print!("  {}{} ", direction.to_string(), direction.get_value());
                } else {
                    print!(" {: >3} ", value);
                }
            } else {
                print!("   - ");
            }
        }
        println!("");
    }

    println!("\n");

    total_heat_loss_values
}

fn find_neighbour(
    width: usize,
    height: usize,
    grids: &HashMap<(usize, usize), i32>,
    current_coord: (usize, usize),
    previous_moved_direction: TravelDirection,
) -> Vec<((usize, usize), TravelDirection, i32)> {
    let mut result: Vec<((usize, usize), TravelDirection, i32)> = vec![];
    // up
    if current_coord.1 > 0 {
        let next_coord = (current_coord.0, current_coord.1 - 1);
        if matches!(previous_moved_direction, TravelDirection::Up(_)) == false {
            result.push((
                next_coord,
                TravelDirection::Up(1),
                *grids.get(&next_coord).unwrap(),
            ));
        } else if let TravelDirection::Up(travel_value) = previous_moved_direction {
            if travel_value < 3 {
                result.push((
                    next_coord,
                    TravelDirection::Up(travel_value + 1),
                    *grids.get(&next_coord).unwrap(),
                ));
            }
        }
    }

    // down
    if current_coord.1 < height - 1 {
        let next_coord = (current_coord.0, current_coord.1 + 1);
        if matches!(previous_moved_direction, TravelDirection::Down(_)) == false {
            result.push((
                next_coord,
                TravelDirection::Down(1),
                *grids.get(&next_coord).unwrap(),
            ));
        } else if let TravelDirection::Down(travel_value) = previous_moved_direction {
            if travel_value < 3 {
                result.push((
                    next_coord,
                    TravelDirection::Down(travel_value + 1),
                    *grids.get(&next_coord).unwrap(),
                ));
            }
        }
    }

    // left
    if current_coord.0 > 0 {
        let next_coord = (current_coord.0 - 1, current_coord.1);
        if matches!(previous_moved_direction, TravelDirection::Left(_)) == false {
            result.push((
                next_coord,
                TravelDirection::Left(1),
                *grids.get(&next_coord).unwrap(),
            ));
        } else if let TravelDirection::Left(travel_value) = previous_moved_direction {
            if travel_value < 3 {
                result.push((
                    next_coord,
                    TravelDirection::Left(travel_value + 1),
                    *grids.get(&next_coord).unwrap(),
                ));
            }
        }
    }

    // right
    if current_coord.0 < width - 1 {
        let next_coord = (current_coord.0 + 1, current_coord.1);
        if matches!(previous_moved_direction, TravelDirection::Right(_)) == false {
            result.push((
                next_coord,
                TravelDirection::Right(1),
                *grids.get(&next_coord).unwrap(),
            ));
        } else if let TravelDirection::Right(travel_value) = previous_moved_direction {
            if travel_value < 3 {
                result.push((
                    next_coord,
                    TravelDirection::Right(travel_value + 1),
                    *grids.get(&next_coord).unwrap(),
                ));
            }
        }
    }
    result
}

const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

const TEST_INPUT_2: &str = "12999
11111
99991";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let lines: Vec<&str> = TEST_INPUT_2.lines().collect();
        let (width, height, grids, list) = parse_input(&lines);
        let (heat_loss, paths) = dijkstra(width, height, &grids);
        let end_point = (width - 1, height - 1);

        print_paths(width, height, &heat_loss, &paths);
        // print_djk(width, height, &heat_loss);

        assert_eq!(heat_loss.get(&end_point).unwrap().clone(), 102);
    }

    #[test]
    fn test_travel() {
        let lines: Vec<&str> = TEST_INPUT_2.lines().collect();
        let (width, height, grids, list) = parse_input(&lines);
        let (heat_loss, paths, directions) = travel(width, height, &grids);

        let mut next_path = *paths.get(&(12, 12)).unwrap();
        println!("{:?}", next_path);

        print_paths_direction(width, height, &grids, &paths, &directions);

        let result = print_paths(width, height, &grids, &paths);
        assert_eq!(result, 102);
    }

    #[test]
    fn test_find_neighbours() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (width, height, grids, list) = parse_input(&lines);
        let current_coord = (0, 0);
        let previous_moved_direction = TravelDirection::None;
        let result = find_neighbour(
            width,
            height,
            &grids,
            current_coord,
            previous_moved_direction,
        );
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, (0, 1));
        assert_eq!(result[0].1, TravelDirection::Down(1));
        assert_eq!(result[0].2, 3);

        assert_eq!(result[1].0, (1, 0));
        assert_eq!(result[1].1, TravelDirection::Right(1));
        assert_eq!(result[1].2, 4);

        let previous_moved_direction = TravelDirection::Down(2);
        let result = find_neighbour(
            width,
            height,
            &grids,
            current_coord,
            previous_moved_direction,
        );
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, (0, 1));
        assert_eq!(result[0].1, TravelDirection::Down(3));
        assert_eq!(result[0].2, 3);

        assert_eq!(result[1].0, (1, 0));
        assert_eq!(result[1].1, TravelDirection::Right(1));
        assert_eq!(result[1].2, 4);

        let previous_moved_direction = TravelDirection::Right(2);
        let result = find_neighbour(
            width,
            height,
            &grids,
            current_coord,
            previous_moved_direction,
        );
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, (0, 1));
        assert_eq!(result[0].1, TravelDirection::Down(1));
        assert_eq!(result[0].2, 3);

        assert_eq!(result[1].0, (1, 0));
        assert_eq!(result[1].1, TravelDirection::Right(3));
        assert_eq!(result[1].2, 4);

        let previous_moved_direction = TravelDirection::Right(3);
        let result = find_neighbour(
            width,
            height,
            &grids,
            current_coord,
            previous_moved_direction,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, (0, 1));
        assert_eq!(result[0].1, TravelDirection::Down(1));
        assert_eq!(result[0].2, 3);

        let previous_moved_direction = TravelDirection::Down(3);
        let result = find_neighbour(
            width,
            height,
            &grids,
            current_coord,
            previous_moved_direction,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, (1, 0));
        assert_eq!(result[0].1, TravelDirection::Right(1));
        assert_eq!(result[0].2, 4);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day17::part_01(&lines);
        assert_eq!(result, 102);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day17::part_02(&lines);
        assert_eq!(result, 2286);
    }
}

fn print_djk(width: usize, height: usize, grids: &HashMap<(usize, usize), i32>) {
    let mut next_path = (width - 1, height - 1);

    for row in 0..height {
        for column in 0..width {
            let coord = (column, row);
            if let Some(value) = grids.get(&coord) {
                print!("{: >3}", value);
            } else {
                print!(" - ");
            }
        }
        println!("");
    }

    println!("\n");
}

fn print_paths(
    width: usize,
    height: usize,
    grids: &HashMap<(usize, usize), i32>,
    paths: &HashMap<(usize, usize), (usize, usize)>,
) -> i32 {
    let mut next_path = (width - 1, height - 1);
    let mut total_heat_loss_values = grids.get(&next_path).copied().unwrap_or(0);
    let mut shortest_path = vec![];
    shortest_path.push(next_path);

    loop {
        if let Some(path) = paths.get(&next_path) {
            shortest_path.push(*path);
            next_path = *path;
            total_heat_loss_values += grids.get(&next_path).copied().unwrap_or(0);
        } else {
            break;
        }
    }

    for row in 0..height {
        for column in 0..width {
            let coord = (column, row);
            if shortest_path.contains(&coord) == true {
                if let Some(value) = grids.get(&coord) {
                    print!("[{: >3}]", value);
                } else {
                    print!("   # ");
                }
            } else if let Some(value) = grids.get(&coord) {
                print!(" {: >3} ", value);
            } else {
                print!("   - ");
            }
        }
        println!("");
    }

    println!("\n");

    total_heat_loss_values
}
