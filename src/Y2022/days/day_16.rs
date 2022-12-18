use std::{collections::VecDeque, time::Instant};

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::file_handler::FileHandler;

pub struct Day16 {}

impl Day16 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_16_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day16::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day16::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_16".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i32 {
        let (start, graphs) = parsing(lines);
        process(&graphs, &start);
        return 0;
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        return 0;
    }
}

const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day16::part_01(&lines);
    assert_eq!(result, 0);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day16::part_02(&lines);
    assert_eq!(result, 0);
}

fn process(graphs: &HashMap<String, (i32, Vec<String>)>, start: &String) {
    let mut pressure_tunnels: HashMap<String, i32> = HashMap::new();
    let mut unopened_valves = HashSet::new();
    for (tunnel, (rate, _)) in graphs.iter() {
        if *rate > 0 {
            pressure_tunnels.insert(tunnel.to_owned(), *rate);
            unopened_valves.insert(tunnel.to_owned());
        }
    }

    let mut opened_valves: HashMap<String, i32> = HashMap::new();
    let mut current_tunnel = start.to_owned();
    let mut minutes = 1;
    let mut target_tunnel: Option<String> = None;
    let mut moving_paths = VecDeque::new();
    while minutes <= 30 {
        match &target_tunnel {
            // it will move to target tunnel and open the valve. This process cost minutes
            Some(tunnel) => {
                // update pressure values from all tunnels here

                // if arrived at target tunnel. It have to open valve
                if &current_tunnel == tunnel {
                    print_step(
                        minutes,
                        &opened_valves,
                        None,
                        Some(current_tunnel.to_owned()),
                    );

                    // open valve
                    opened_valves.insert(
                        current_tunnel.to_owned(),
                        *pressure_tunnels.get(&current_tunnel).unwrap(),
                    );
                    unopened_valves.remove(&current_tunnel);
                    target_tunnel = None;

                    
                }
                // it need to keep moving
                else {
                    if let Some(next_tunnel) = moving_paths.pop_front() {
                        current_tunnel = next_tunnel;
                    } else {
                        panic!("Path should possible to reach the target tunnel");
                    }

                    print_step(
                        minutes,
                        &opened_valves,
                        Some(current_tunnel.to_owned()),
                        None,
                    );
                }

                minutes += 1;
            }
            // choose target tunnel. This process will not cost minutes unless all pressured valves opened
            None => {
                if unopened_valves.len() > 0 {
                    let (target, path) = look_for_target_tunnel(
                        graphs,
                        &unopened_valves,
                        &pressure_tunnels,
                        &current_tunnel,
                        31 - minutes
                    );
                    target_tunnel = target;
                    moving_paths = path;
                } else {
                    // update pressure valves value
                    print_step(minutes, &opened_valves, None, None);
                    minutes += 1;
                }
            }
        }
    }
}

fn print_step(
    minutes: i32,
    opened_valves: &HashMap<String, i32>,
    move_to: Option<String>,
    open: Option<String>,
) {
    println!("== Minute {} ==", minutes);
    if opened_valves.len() == 0 {
        println!("No valves are open.");
    } else {
        println!(
            "Valve {:?} is open, releasing {} pressure.",
            opened_valves.keys(),
            calculate_releasing_pressure(opened_valves)
        );
    }

    if let Some(tunnel) = move_to {
        println!("You move to valve {}.", tunnel);
    }

    if let Some(tunnel) = open {
        println!("You open valve {}.", tunnel);
    }
}

fn calculate_releasing_pressure(opened_valves: &HashMap<String, i32>) -> i32 {
    let mut total_pressure = 0;
    for (_, value) in opened_valves.iter() {
        total_pressure += *value;
    }

    total_pressure
}

fn look_for_target_tunnel(
    graphs: &HashMap<String, (i32, Vec<String>)>,
    unopened_valves: &HashSet<String>,
    pressure_tunnels: &HashMap<String, i32>,
    current_tunnel: &String,
    minutes_left: i32,
) -> (Option<String>, VecDeque<String>) {
    println!("look_for_target_tunnel: {}", current_tunnel);
    let parents = bfs(graphs, &current_tunnel);
    let mut maximum_value = i32::MIN;
    let mut target_tunnel = None;
    let mut moving_paths = VecDeque::new();
    // choose target here
    for tunnel in unopened_valves.iter() {
        
        let (completed, path) = construct_path(&parents, &current_tunnel, tunnel);
        let values = if completed == true {
            if let Some(rate) = pressure_tunnels.get(tunnel) {
                let cost = path.len() + 1;
                *rate * (minutes_left - cost as i32)
            } else {
                println!("pressure_tunnels not exist {}", tunnel);
                i32::MIN
            }
        } else {
            println!("Not completed path: {}\n{:?}\n", tunnel, path);
            i32::MIN
        };

        println!("Checking tunnel: {} value: {}", tunnel, values);

        if values > maximum_value {
            maximum_value = values;
            target_tunnel = Some(tunnel.to_owned());
            moving_paths = VecDeque::from_iter(path.into_iter());
            println!("Found best value: {} target: {:?}", maximum_value, target_tunnel);
        }
    }
    println!{"target: {:?}\npath:\n{:?}\n--",target_tunnel, moving_paths};
    (target_tunnel, moving_paths)
}

fn construct_path(
    parents: &HashMap<String, String>,
    start: &String,
    target: &String,
) -> (bool, Vec<String>) {
    
    let mut path = vec![];
    let mut current_node = parents.get(target);
    let mut path_completed = false;
    path.push(target.to_owned());
    loop {
        match current_node {
            Some(node) => {
                
                
                if node == start {
                    path_completed = true;
                }
                else
                {
                    path.push(node.to_owned());
                }

                current_node = parents.get(node);
            }
            None => {
                break;
            }
        }
    }

    // println!("Construct {} to {}\n{:?}\n--", start, target, path.clone().into_iter().rev());
    (path_completed, Vec::from_iter(path.into_iter().rev()))
}

fn bfs(graphs: &HashMap<String, (i32, Vec<String>)>, start: &String) -> HashMap<String, String> {
    println!("BFS: {:?}", start);
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(start.to_owned());
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.to_owned());
    let mut parents: HashMap<String, String> = HashMap::new();

    while queue.len() > 0 {
        let current_node = queue.pop_front().unwrap();
        if let Some((_, neighbours)) = graphs.get(&current_node) {
            for node in neighbours.iter() {
                if visited.contains(node) == false {
                    visited.insert(node.to_owned());
                    parents.insert(node.to_owned(), current_node.to_owned());
                    queue.push_back(node.to_owned());
                    // println!("{:?}", parents);
                }
            }
        }
    }

    parents
}

fn parsing(lines: &Vec<&str>) -> (String, HashMap<String, (i32, Vec<String>)>) {
    let mut start_tunnel = String::new();
    let mut result = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        let (first, second) = line.split_once(" has flow rate=").unwrap();
        let valve = first.replace("Valve ", "");
        let (second, third) = if second.contains("tunnels") {
            second.split_once("; tunnels lead to valves ").unwrap()
        } else {
            second.split_once("; tunnel leads to valve ").unwrap()
        };

        let rate = second.parse::<i32>().unwrap();
        let tunnels = third
            .replace(" ", "")
            .split(",")
            .map(|v| v.to_owned())
            .collect_vec();

        result.insert(valve.to_owned(), (rate, tunnels));

        if index == 0 {
            start_tunnel = valve.to_owned()
        }
    }
    (start_tunnel, result)
}
