use crate::file_handler::FileHandler;
use rayon::{prelude::*, result};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day06 {}

impl Day06 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2024/inputs/day_06_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day06::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day06::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_06".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i32 {
        let (width, height, (mut x, mut y), obstacles) = parsing_input(&lines);
        let mut guard_direction = MoveDirection::Up;
        let mut positions = HashSet::new();
        positions.insert((x, y));

        loop {
            match guard_direction.take_a_step(width, height, &obstacles, (&mut x, &mut y)) {
                MoveResult::Success(direction) => {
                    positions.insert((x, y));
                }
                MoveResult::Leave => {
                    break;
                }
                MoveResult::Obstacle => {}
            }
        }

        positions.len() as i32
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        let (width, height, (mut x, mut y), obstacles) = parsing_input(&lines);
        let mut guard_direction = MoveDirection::Up;
        let mut positions = HashMap::new();
        let mut list = HashSet::new();
        list.insert(guard_direction.clone());
        positions.insert((x, y), list);

        let mut put_obstacle_counter = 0;
        loop {
            match guard_direction.take_a_step(width, height, &obstacles, (&mut x, &mut y)) {
                MoveResult::Success(mut direction) => {
                    let key = (x, y);

                    if let Some(list) = positions.get_mut(&key) {
                        list.insert(direction.clone());
                    } else {
                        let mut list = HashSet::new();
                        list.insert(direction.clone());
                        positions.insert(key, list);
                    }

                    let mut can_put_obstacle = false;
                    if let Some(foot_prints) = positions.get(&key) {
                        let mut turned_direction = direction.clone();
                        turned_direction.turn();

                        foot_prints.iter().any(|prev_direction| {
                            if *prev_direction == turned_direction {
                                can_put_obstacle = true;
                                // print_table(width, height, &obstacles, &positions, Some((x,y)));
                                true
                            } else {
                                false
                            }
                        });
                    }

                    if can_put_obstacle == false
                        && look_to_the_right(
                            width,
                            height,
                            &obstacles,
                            &positions,
                            x,
                            y,
                            direction.clone(),
                        )
                    {
                        can_put_obstacle = true;
                        // print_table(width, height, &obstacles, &positions, Some((x,y)));
                    }

                    if can_put_obstacle == true {
                        put_obstacle_counter += 1;
                    }
                }
                MoveResult::Leave => {
                    break;
                }
                MoveResult::Obstacle => {
                    let key = (x, y);

                    if let Some(list) = positions.get_mut(&key) {
                        list.insert(guard_direction.clone());
                    } else {
                        let mut list = HashSet::new();
                        list.insert(guard_direction.clone());
                        positions.insert(key, list);
                    }
                }
            }
        }

        put_obstacle_counter
    }
}

/// `true`. If simulate until found old path.
/// `false`. Walk out side the area without finding any old paths.
fn simulate_walk_to_find_old_path(
    width: usize,
    height: usize,
    obstacles: &HashSet<(usize, usize)>,
    path: &HashMap<(usize, usize), HashSet<MoveDirection>>,
    start_x: usize,
    start_y: usize,
    current_direction: MoveDirection,
    put_x: usize,
    put_y: usize,
) -> bool {
    let mut x = start_x;
    let mut y = start_y;
    let mut direction = current_direction.clone();
    let mut new_path: HashMap<(usize, usize), HashSet<MoveDirection>> = path.clone();
    // let mut list = HashSet::new();
    // list.insert(current_direction.clone());
    // new_path.insert((start_x, start_y), list);

    loop {
        match direction.take_a_step(width, height, obstacles, (&mut x, &mut y)) {
            MoveResult::Success(new_direction) => {
                let key = (x, y);

                if let Some(list) = path.get(&key) {
                    if list.contains(&new_direction) == true {
                        return true;
                    }
                }

                if let Some(list) = new_path.get(&key) {
                    if list.contains(&new_direction) == true {
                        print_table(
                            width,
                            height,
                            obstacles,
                            &new_path,
                            Some((put_x, put_y)),
                            Some((start_x, start_y)),
                        );
                        return true;
                    }
                }

                if new_path.contains_key(&key) == false {
                    new_path.insert(key, HashSet::new());
                }

                if let Some(list) = new_path.get_mut(&key) {
                    list.insert(new_direction);
                }
            }
            MoveResult::Leave => {
                return false;
            }
            MoveResult::Obstacle => {
                let key = (x, y);
                if new_path.contains_key(&key) == false {
                    new_path.insert(key, HashSet::new());
                }

                if let Some(list) = new_path.get_mut(&key) {
                    list.insert(direction.clone());
                }
            }
        }
    }
}

/// look to the right for the direction from previous path that have the same `direction` as `looking direction`.
fn look_to_the_right(
    width: usize,
    height: usize,
    obstacles: &HashSet<(usize, usize)>,
    path: &HashMap<(usize, usize), HashSet<MoveDirection>>,
    x: usize,
    y: usize,
    current_direction: MoveDirection,
) -> bool {
    let (put_x, put_y) = current_direction.step_forward(x, y);

    let mut looking_direction = current_direction.clone();
    looking_direction.turn();

    if simulate_walk_to_find_old_path(
        width,
        height,
        obstacles,
        path,
        x,
        y,
        looking_direction.clone(),
        put_x,
        put_y,
    ) {
        return true;
    }

    false
}

fn print_table(
    width: usize,
    height: usize,
    obstacles: &HashSet<(usize, usize)>,
    path: &HashMap<(usize, usize), HashSet<MoveDirection>>,
    put_obstacle: Option<(usize, usize)>,
    start: Option<(usize, usize)>,
) {
    (0..height).for_each(|row| {
        (0..width).for_each(|column| {
            let key = (column, row);

            let start_it = if let Some((start_x, start_y)) = start.as_ref() {
                *start_x == column && *start_y == row
            } else {
                false
            };

            let put_it = if let Some((put_x, put_y)) = put_obstacle.as_ref() {
                *put_x == column && *put_y == row
            } else {
                false
            };

            if start_it == true {
                print!("^");
            } else if put_it == true {
                print!("O");
            } else if obstacles.contains(&key) {
                print!("#");
            } else if let Some(directions) = path.get(&key) {
                if (directions.contains(&MoveDirection::Down)
                    || directions.contains(&MoveDirection::Up))
                    && (directions.contains(&MoveDirection::Left)
                        || directions.contains(&MoveDirection::Right))
                {
                    print!("+")
                } else {
                    directions.iter().all(|v| {
                        v.print();
                        false
                    });
                }
            } else {
                print!(".");
            }
        });

        println!("");
    });

    println!("\n\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_table() {
        let mut list = HashSet::new();
        list.insert(MoveDirection::Up);
        list.insert(MoveDirection::Right);
        let mut path = HashMap::new();
        path.insert((1, 1), list);
        print_table(4, 4, &HashSet::new(), &path, None, None);
    }

    #[test]
    fn test_up() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (width, height, (mut x, mut y), obstacles) = parsing_input(&lines);
        assert_eq!(x, 4);
        assert_eq!(y, 6);

        let mut guard_direction = MoveDirection::Up;

        (0..5).for_each(|_| {
            let result = guard_direction.up(width, height, &obstacles, (&mut x, &mut y));
            assert_eq!(result, MoveResult::Success(MoveDirection::Up));
        });

        let result = guard_direction.up(width, height, &obstacles, (&mut x, &mut y));
        assert_eq!(result, MoveResult::Obstacle);
    }

    #[test]
    fn test_down() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (width, height, (mut x, mut y), obstacles) = parsing_input(&lines);
        assert_eq!(x, 4);
        assert_eq!(y, 6);

        let mut guard_direction = MoveDirection::Down;

        (0..3).for_each(|_| {
            let result = guard_direction.down(width, height, &obstacles, (&mut x, &mut y));
            assert_eq!(result, MoveResult::Success(MoveDirection::Down));
        });

        let result = guard_direction.down(width, height, &obstacles, (&mut x, &mut y));
        assert_eq!(result, MoveResult::Leave);
    }

    #[test]
    fn test_left() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (width, height, (mut x, mut y), obstacles) = parsing_input(&lines);
        assert_eq!(x, 4);
        assert_eq!(y, 6);

        let mut guard_direction = MoveDirection::Left;

        (0..2).for_each(|_| {
            let result = guard_direction.left(width, height, &obstacles, (&mut x, &mut y));
            assert_eq!(result, MoveResult::Success(MoveDirection::Left));
        });

        let result = guard_direction.left(width, height, &obstacles, (&mut x, &mut y));
        assert_eq!(result, MoveResult::Obstacle);
    }

    #[test]
    fn test_right() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let (width, height, (mut x, mut y), obstacles) = parsing_input(&lines);
        assert_eq!(x, 4);
        assert_eq!(y, 6);

        let mut guard_direction = MoveDirection::Right;

        (0..5).for_each(|_| {
            let result = guard_direction.right(width, height, &obstacles, (&mut x, &mut y));
            assert_eq!(result, MoveResult::Success(MoveDirection::Right));
        });

        let result = guard_direction.right(width, height, &obstacles, (&mut x, &mut y));
        assert_eq!(result, MoveResult::Leave);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day06::part_01(&lines);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day06::part_02(&lines);
        assert_eq!(result, 6);
    }
}

fn step(
    width: usize,
    height: usize,
    obstacles: &HashSet<(usize, usize)>,
    guard_position: (&mut usize, &mut usize),
    move_direction: &mut MoveDirection,
) {
}

#[derive(Debug, PartialEq, Eq)]
enum MoveResult {
    Obstacle,
    Success(MoveDirection),
    Leave,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum MoveDirection {
    Up,
    Right,
    Down,
    Left,
}

impl MoveDirection {
    fn print(&self) {
        match self {
            MoveDirection::Down | MoveDirection::Up => print!("|"),
            MoveDirection::Left | MoveDirection::Right => print!("-"),
        }
    }
    fn turn(&mut self) {
        *self = match self {
            MoveDirection::Up => MoveDirection::Right,
            MoveDirection::Right => MoveDirection::Down,
            MoveDirection::Down => MoveDirection::Left,
            MoveDirection::Left => MoveDirection::Up,
        };
    }

    fn step_back(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            MoveDirection::Up => (x, y.saturating_add(1)),
            MoveDirection::Down => (x, y.saturating_sub(1)),
            MoveDirection::Left => (x.saturating_add(1), y),
            MoveDirection::Right => (x.saturating_sub(1), y),
        }
    }

    fn step_forward(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            MoveDirection::Up => (x, y.saturating_sub(1)),
            MoveDirection::Down => (x, y.saturating_add(1)),
            MoveDirection::Left => (x.saturating_sub(1), y),
            MoveDirection::Right => (x.saturating_add(1), y),
        }
    }

    fn take_a_step(
        &mut self,
        width: usize,
        height: usize,
        obstacles: &HashSet<(usize, usize)>,
        guard_position: (&mut usize, &mut usize),
    ) -> MoveResult {
        let result = match self {
            MoveDirection::Up => self.up(width, height, obstacles, guard_position),
            MoveDirection::Right => self.right(width, height, obstacles, guard_position),
            MoveDirection::Down => self.down(width, height, obstacles, guard_position),
            MoveDirection::Left => self.left(width, height, obstacles, guard_position),
        };

        match result {
            MoveResult::Success(_) => {}
            MoveResult::Leave => {}
            MoveResult::Obstacle => {
                self.turn();
            }
        }

        result
    }

    fn up(
        &self,
        width: usize,
        height: usize,
        obstacles: &HashSet<(usize, usize)>,
        guard_position: (&mut usize, &mut usize),
    ) -> MoveResult {
        let (x, y) = guard_position;
        if y.checked_sub(1).is_none() == true {
            return MoveResult::Leave;
        }

        let next_y = y.saturating_sub(1);
        if obstacles.contains(&(*x, next_y)) == true {
            return MoveResult::Obstacle;
        }

        *y = next_y;
        MoveResult::Success(MoveDirection::Up)
    }

    fn down(
        &self,
        width: usize,
        height: usize,
        obstacles: &HashSet<(usize, usize)>,
        guard_position: (&mut usize, &mut usize),
    ) -> MoveResult {
        let (x, y) = guard_position;
        if *y + 1 >= height {
            return MoveResult::Leave;
        }

        let next_y = y.saturating_add(1);
        if obstacles.contains(&(*x, next_y)) == true {
            return MoveResult::Obstacle;
        }

        *y = next_y;
        MoveResult::Success(MoveDirection::Down)
    }

    fn left(
        &self,
        width: usize,
        height: usize,
        obstacles: &HashSet<(usize, usize)>,
        guard_position: (&mut usize, &mut usize),
    ) -> MoveResult {
        let (x, y) = guard_position;
        if x.checked_sub(1).is_none() == true {
            return MoveResult::Leave;
        }

        let next_x = x.saturating_sub(1);
        if obstacles.contains(&(next_x, *y)) == true {
            return MoveResult::Obstacle;
        }

        *x = next_x;
        MoveResult::Success(MoveDirection::Left)
    }

    fn right(
        &self,
        width: usize,
        height: usize,
        obstacles: &HashSet<(usize, usize)>,
        guard_position: (&mut usize, &mut usize),
    ) -> MoveResult {
        let (x, y) = guard_position;
        if *x + 1 >= width {
            return MoveResult::Leave;
        }

        let next_x = x.saturating_add(1);
        if obstacles.contains(&(next_x, *y)) == true {
            return MoveResult::Obstacle;
        }

        *x = next_x;
        MoveResult::Success(MoveDirection::Right)
    }
}

fn parsing_input(lines: &Vec<&str>) -> (usize, usize, (usize, usize), HashSet<(usize, usize)>) {
    let height = lines.len();
    let width = lines.first().unwrap().len();

    let mut obstacles = HashSet::new();
    let mut guard_position = (0, 0);
    lines.iter().enumerate().for_each(|(row, text)| {
        text.chars().enumerate().for_each(|(column, character)| {
            if character == '#' {
                obstacles.insert((column, row));
            } else if character == '^' {
                guard_position = (column, row);
            }
        });
    });

    (width, height, guard_position, obstacles)
}

const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
