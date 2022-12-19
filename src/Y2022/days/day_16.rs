use std::{collections::VecDeque, time::Instant};

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::file_handler::FileHandler;

const TOTAL_MINUTES: i32 = 30;
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
        let (start, graph) = parsing(lines);
        process(&graph, &"AA".to_owned())
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
    assert_eq!(result, 1651);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day16::part_02(&lines);
    assert_eq!(result, 0);
}

#[test]
fn test_calculate_releasing_pressure_from_valves() {
    let mut opened_valves = HashMap::new();
    opened_valves.insert("A".to_owned(), 5);
    assert_eq!(5, calculate_releasing_pressure_from_valves(&opened_valves));

    opened_valves.insert("B".to_owned(), 3);
    assert_eq!(8, calculate_releasing_pressure_from_valves(&opened_valves));
}

#[test]
fn test_calculate_release_pressure_from_path() {
    let mut path = vec![];
    path.push(("A".to_owned(), 2, 5, (TOTAL_MINUTES - 2) * 5));
    assert_eq!(
        (TOTAL_MINUTES - 2) * 5,
        calculate_release_pressure_from_path(&path)
    );

    path.push(("B".to_owned(), 5, 10, (TOTAL_MINUTES - 5) * 10));
    assert_eq!(
        (TOTAL_MINUTES - 2) * 5 + (TOTAL_MINUTES - 5) * 10,
        calculate_release_pressure_from_path(&path)
    );
}

#[test]
fn test_calculate_release_pressure_from_path_at() {
    let mut path = vec![];
    path.push(("A".to_owned(), 2, 5, (TOTAL_MINUTES - 2) * 5));
    assert_eq!(
        (3 - 2) * 5,
        calculate_release_pressure_from_path_at(&path, 3)
    );

    path.push(("B".to_owned(), 5, 10, (TOTAL_MINUTES - 5) * 10));
    assert_eq!(
        (10 - 2) * 5 + (10 - 5) * 10,
        calculate_release_pressure_from_path_at(&path, 10)
    );
}

struct OptimalResult {
    total_process: i32,
    path: Vec<(String, i32, i32, i32)>,
    travel_path: Vec<String>,
}

fn process(graph: &HashMap<String, (i32, Vec<String>)>, start: &String) -> i32 {
    let mut optimal_result = OptimalResult {
        total_process: 0,
        path: vec![],
        travel_path: vec![],
    };

    let opened_valves = HashMap::new();
    let mut unopened_valves = HashMap::new();
    for (valve, (flow_rate, _)) in graph.iter() {
        if *flow_rate > 0 {
            unopened_valves.insert(valve.to_owned(), *flow_rate);
        }
    }
    // println!(
    //     "process: {} graph size: {}\nunopened {}: {:?}\n",
    //     start,
    //     graph.len(),
    //     unopened_valves.len(),
    //     unopened_valves
    // );
    dfs(
        graph,
        &mut optimal_result,
        &opened_valves,
        &unopened_valves,
        &vec![],
        0,
        start,
        &vec![],
    );

    // println!(
    //     "Result: {}\npath: {:?}\ntraveled: {:?}\n--",
    //     calculate_release_pressure_from_path(&optimal_result.path),
    //     optimal_result.path,
    //     optimal_result.travel_path,
    // );

    calculate_release_pressure_from_path(&optimal_result.path)
}

fn calculate_releasing_pressure_from_valves(opened_valves: &HashMap<String, i32>) -> i32 {
    let mut total_pressure = 0;
    for (_, value) in opened_valves.iter() {
        total_pressure += *value;
    }

    total_pressure
}

// calculate released pressure from path at TOTAL MINUTES
fn calculate_release_pressure_from_path(path: &Vec<(String, i32, i32, i32)>) -> i32 {
    let mut total_released_pressure = 0;
    for (_, _, _, total_released) in path.iter() {
        total_released_pressure += total_released;
    }

    total_released_pressure
}

fn calculate_release_pressure_from_path_at(
    path: &Vec<(String, i32, i32, i32)>,
    current_minutes: i32,
) -> i32 {
    let mut total_released_pressure = 0;
    for (_, minutes, flow_rate, _) in path.iter() {
        let active_minutes = current_minutes - minutes;
        total_released_pressure += flow_rate * active_minutes;
    }

    total_released_pressure
}

fn dfs(
    graph: &HashMap<String, (i32, Vec<String>)>,
    optimal_result: &mut OptimalResult,
    opened_valves: &HashMap<String, i32>,
    unopened_valves: &HashMap<String, i32>,
    current_path: &Vec<(String, i32, i32, i32)>,
    minutes: i32,
    current_node: &String,
    travel_path: &Vec<String>,
) {
    /// Reach time limit.
    if minutes > TOTAL_MINUTES || unopened_valves.len() == 0 {
        // Is current path it the best
        update_optimal_result(optimal_result, current_path, travel_path, minutes);

        optimal_result.total_process += 1;

        return;
    }

    // println!("DFS path length: {}", current_path.len());

    // before continue recursive. It need to check that this it is possible that
    // it can have result better than optimal result
    if validate_possible_result(
        optimal_result,
        &opened_valves,
        unopened_valves,
        current_path,
        minutes,
    ) == false
    {
        return;
    }

    // recursive next node from unopened valves
    for (valve, flow_rate) in unopened_valves.iter() {
        let (operation_minute_cost, mut traveled_path) =
            calculate_operation_cost(graph, current_node, valve);
        let new_minutes = minutes + operation_minute_cost + 1;
        // if new_minutes > TOTAL_MINUTES {
        //     continue;
        // }

        let minutes_left = TOTAL_MINUTES - new_minutes;

        let mut new_path = if minutes_left > 0 {
            let mut new_path = current_path.clone();
            new_path.push((
                valve.to_owned(),
                new_minutes,
                *flow_rate,
                flow_rate * minutes_left,
            ));
            new_path
        } else {
            current_path.clone()
        };

        let mut new_travel_path = travel_path.clone();
        new_travel_path.append(&mut traveled_path);

        let mut new_unopened_valves = unopened_valves.clone();
        new_unopened_valves.remove(valve);

        let mut new_opened_valves = opened_valves.clone();
        new_opened_valves.insert(valve.to_owned(), *flow_rate);
        if optimal_result.path.len() == 0 {
            println!("{:?}", new_path);
        }

        dfs(
            graph,
            optimal_result,
            &new_opened_valves,
            &new_unopened_valves,
            &new_path,
            new_minutes,
            &valve.clone(),
            &new_travel_path,
        );
    }
}

fn update_optimal_result(
    optimal_result: &mut OptimalResult,
    current_path: &Vec<(String, i32, i32, i32)>,
    travel_path: &Vec<String>,
    minutes: i32,
) {
    if calculate_release_pressure_from_path(&optimal_result.path)
        < calculate_release_pressure_from_path(current_path)
        || optimal_result.path.len() == 0
    {
        optimal_result.path.clear();
        optimal_result.path.append(&mut current_path.clone());

        optimal_result.travel_path.clear();
        optimal_result.travel_path.append(&mut travel_path.clone());

        // println!(
        //     "dfs END: {} minutes {} times\nrelease pressure: {}\npath: {:?}\n--\n",
        //     optimal_result.total_process,
        //     minutes,
        //     calculate_release_pressure_from_path(current_path),
        //     current_path
        // );
    }
}

fn calculate_operation_cost(
    graph: &HashMap<String, (i32, Vec<String>)>,
    start: &String,
    target: &String,
) -> (i32, Vec<String>) {
    let parents = bfs(graph, start);
    let (_, path) = construct_path(&parents, start, target);
    // println!("calculate_operation_cost:\nstart: {} target: {}\npath: {:?}\n---\n", start, target, path);
    (path.len() as i32, path)
}

#[test]
fn test_validate_possible_result() {
    let B = ("B".to_owned(), 3, 2, (TOTAL_MINUTES - 3) * 2);
    let A = ("A".to_owned(), 15, 5, (TOTAL_MINUTES - 15) * 5);
    let C = ("C".to_owned(), 20, 10, (TOTAL_MINUTES - 20) * 10);
    let optimal_result = OptimalResult {
        total_process: 0,
        path: vec![B.clone(), A.clone(), C.clone()],
        travel_path: vec![],
    };

    println!(
        "Optimal Result: {}",
        calculate_release_pressure_from_path(&optimal_result.path)
    );

    let mut opened_valves = HashMap::new();
    let mut unopened_valves = HashMap::new();
    unopened_valves.insert("A".to_owned(), 5);
    unopened_valves.insert("B".to_owned(), 2);
    unopened_valves.insert("C".to_owned(), 10);
    assert_eq!(
        true,
        validate_possible_result(
            &optimal_result,
            &opened_valves,
            &unopened_valves,
            &vec![],
            5
        )
    );
    assert_eq!(
        false,
        validate_possible_result(
            &optimal_result,
            &opened_valves,
            &unopened_valves,
            &vec![],
            29
        )
    );

    opened_valves.insert("A".to_owned(), 15);
    let mut unopened_valves = HashMap::new();
    unopened_valves.insert("B".to_owned(), 2);
    unopened_valves.insert("C".to_owned(), 10);

    let A = ("A".to_owned(), 2, 5, (TOTAL_MINUTES - 2) * 5);
    let mut current_path = vec![A];
    assert_eq!(
        true,
        validate_possible_result(
            &optimal_result,
            &opened_valves,
            &unopened_valves,
            &current_path,
            5
        )
    );
    assert_eq!(
        true,
        validate_possible_result(
            &optimal_result,
            &opened_valves,
            &unopened_valves,
            &current_path,
            15
        )
    );
    assert_eq!(
        true,
        validate_possible_result(
            &optimal_result,
            &opened_valves,
            &unopened_valves,
            &current_path,
            20
        )
    );
    assert_eq!(
        true,
        validate_possible_result(
            &optimal_result,
            &opened_valves,
            &unopened_valves,
            &current_path,
            25
        )
    );
    assert_eq!(
        false,
        validate_possible_result(
            &optimal_result,
            &opened_valves,
            &unopened_valves,
            &current_path,
            26
        )
    );
    assert_eq!(
        false,
        validate_possible_result(
            &optimal_result,
            &opened_valves,
            &unopened_valves,
            &current_path,
            27
        )
    );
    assert_eq!(
        false,
        validate_possible_result(
            &optimal_result,
            &opened_valves,
            &unopened_valves,
            &current_path,
            30
        )
    );
}

fn validate_possible_result(
    optimal_result: &OptimalResult,
    opened_valves: &HashMap<String, i32>,
    unopened_valves: &HashMap<String, i32>,
    current_path: &Vec<(String, i32, i32, i32)>,
    minutes: i32,
) -> bool {
    let minute_left = TOTAL_MINUTES - minutes;
    let opened_release_value = calculate_releasing_pressure_from_valves(opened_valves);
    let unopened_release_value = calculate_releasing_pressure_from_valves(&unopened_valves);
    let possible_value = calculate_release_pressure_from_path_at(current_path, minutes)
        + opened_release_value * minute_left
        + unopened_release_value * minute_left;

    if calculate_release_pressure_from_path(&optimal_result.path) < possible_value {
        // println!("validate_possible_result: TRUE {} | {}\nopened: {} / {}\n{:?}\nunopened: {} / {}\n{:?}\noptimal: {}\npath:\n{:?}\n++\n",
        //     minutes, minute_left,
        //     opened_release_value, opened_release_value * minute_left, opened_valves,
        //     unopened_release_value, unopened_release_value * minute_left, unopened_valves,
        //     calculate_release_pressure_from_path(&optimal_result.path), optimal_result.path);
        true
    } else {
        // println!("validate_possible_result: FALSE {} | {}:\nopened: {} / {}\n{:?}\nunopened: {} / {}\n{:?}\ncurrent_path:\n{:?}\noptimal: {}\npath:\n{:?}\n--\n",
        //     minutes, minute_left,
        //     opened_release_value, opened_release_value * minute_left, opened_valves,
        //     unopened_release_value, unopened_release_value * minute_left, unopened_valves,
        //     current_path,
        //     calculate_release_pressure_from_path(&optimal_result.path) , optimal_result.path);
        false
    }
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
                } else {
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
    // println!("BFS: {:?}", start);
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
