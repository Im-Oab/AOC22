use std::time::Instant;

use hashbrown::HashSet;
use itertools::Itertools;

use crate::file_handler::FileHandler;

#[derive(Clone, Copy, Debug)]
enum Commands {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

#[derive(Clone)]
struct Knot {
    x: i32,
    y: i32,
    history: HashSet<String>,
    logging: bool,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        let mut hash = HashSet::new();
        hash.insert(format!("{},{}", x, y));
        Self {
            x: x,
            y: y,
            history: hash,
            logging: false,
        }
    }

    fn zero(logging: bool) -> Self {
        let mut hash = HashSet::new();
        hash.insert("0,0".to_owned());

        Self {
            x: 0,
            y: 0,
            history: hash,
            logging: logging,
        }
    }

    fn is_apart(&self, other: &Knot) -> bool {
        let diff_x = other.x - self.x;
        let diff_y = other.y - self.y;
        diff_x.abs() > 1 || diff_y.abs() > 1
    }

    fn apply(&mut self, other: &Knot) {
        self.x = other.x;
        self.y = other.y;
        if self.logging == true {
            self.history.insert(format!("{},{}", self.x, self.y));
        }
    }

    fn execute(&mut self, command: &Commands) -> Commands {
        match command {
            Commands::Up(value) => {
                self.y += 1;
                Commands::Up(value - 1)
            }
            Commands::Down(value) => {
                self.y -= 1;
                Commands::Down(value - 1)
            }
            Commands::Left(value) => {
                self.x -= 1;
                Commands::Left(value - 1)
            }
            Commands::Right(value) => {
                self.x += 1;
                Commands::Right(value - 1)
            }
        }
    }

    fn total_visited(&self) -> i32 {
        self.history.len() as i32
    }
}

pub struct Day09 {}

impl Day09 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/inputs/day_09_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day09::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day09::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_09".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// Part 1 took me some time to get the correct result. Furthermore, the issue is that I start implementing the code
    /// before ensuring I understand what I need to do correctly. Also, The logic of the movement of the rope is basic.
    fn part_01(lines: &Vec<&str>) -> i32 {
        let mut head = Knot::zero(false);
        let mut tail = vec![Knot::zero(true)];
        let commands = parsing_input(lines);

        execute_all_commands_with_multiple_knots(&commands, &mut head, &mut tail);
        return tail.last().unwrap().total_visited();
    }

    /// Part 2 shows me that the code I write for part 1 does not support multiple knots on the tails. I have to rewrite the code
    /// for moving the knots. Furthermore, the same issue occurs again. I struggled to get the result
    /// because I did not understand it correctly and kept coding the incorrect logic.
    ///Also, I have to tidy up the code to make it run faster and look ok. The issue of the slows is because
    /// I calculate distance using sqrt() and log all the steps of all knots instead of the last only knot.
    fn part_02(lines: &Vec<&str>) -> i32 {
        let mut head = Knot::zero(false);
        let mut tail = (0..9).map(|index| Knot::zero(index == 8)).collect_vec();
        let commands = parsing_input(lines);

        execute_all_commands_with_multiple_knots(&commands, &mut head, &mut tail);

        return tail.last().unwrap().total_visited();
    }
}

const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day09::part_01(&lines);
    assert_eq!(result, 13);
}

const TEST_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT_2.lines().collect();
    let result = Day09::part_02(&lines);
    assert_eq!(result, 36);
}

/// I have learnt how to use split_once(). :)
fn parsing_input(lines: &Vec<&str>) -> Vec<Commands> {
    lines
        .iter()
        .map(|line| {
            let input = (*line).to_owned();
            let (command, value) = input.split_once(" ").unwrap();
            let value = value.parse::<usize>().unwrap();
            let command = command.to_owned();
            if command == "U" {
                Commands::Up(value)
            } else if command == "D" {
                Commands::Down(value)
            } else if command == "L" {
                Commands::Left(value)
            } else if command == "R" {
                Commands::Right(value)
            } else {
                Commands::Up(0)
            }
        })
        .collect_vec()
}

fn execute_all_commands_with_multiple_knots(
    commands: &Vec<Commands>,
    head: &mut Knot,
    tail: &mut Vec<Knot>,
) {
    for command in commands.iter() {
        let mut current_command: Commands = (*command).clone();

        loop {
            let first_tail = tail.first_mut().unwrap();

            current_command = excute_command(&current_command, head, first_tail);

            for index in 1..tail.len() {
                let target_knot = tail[index - 1].clone();
                let knot = tail.get_mut(index).unwrap();
                if move_knot(knot, &target_knot) == false {
                    break;
                }
            }

            match current_command {
                Commands::Down(value)
                | Commands::Up(value)
                | Commands::Left(value)
                | Commands::Right(value) => {
                    if value == 0 {
                        break;
                    }
                }
            }
        }
    }
}

fn excute_command(command: &Commands, head: &mut Knot, knot: &mut Knot) -> Commands {
    let new_command = head.execute(&command);

    move_knot(knot, head);

    new_command
}

fn move_knot(knot: &mut Knot, target: &Knot) -> bool {
    let is_apart = knot.is_apart(target);
    if is_apart == true {
        let offset_x = {
            if knot.x < target.x {
                1
            } else if knot.x > target.x {
                -1
            } else {
                0
            }
        };

        let offset_y = {
            if knot.y < target.y {
                1
            } else if knot.y > target.y {
                -1
            } else {
                0
            }
        };
        knot.apply(&Knot::new(knot.x + offset_x, knot.y + offset_y));

        true
    } else {
        false
    }
}
