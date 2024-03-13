use crate::file_handler::FileHandler;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

pub struct Day20 {}

impl Day20 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_20_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day20::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day20::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_20".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i128 {
        let data = parse_input(&lines);
        
        let mut data = prepare_data(&data);
        let mut total_low_pulse = 0;
        let mut total_high_pulse = 0;
        (0..1000).for_each(|_|{
            let (low,high) = send_pulse(&mut data);
            total_low_pulse += low;
            total_high_pulse += high;
        });

        total_low_pulse * total_high_pulse
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        return 0;
    }
}

fn send_pulse(modules: &mut HashMap<String, Module>) -> (i128,i128)
{
    let mut total_low_pulse = 0;
    let mut total_high_pulse = 0;
    let mut queue = VecDeque::new();
    queue.push_back(("button".to_owned(), "broadcaster".to_owned(), 0));
    loop
    {
        if let Some((from, to, pulse)) = queue.pop_front()
        {
            // println!("{} -{}-> {}", from, pulse, to);
            if pulse == 0
            {
                total_low_pulse += 1;
            }
            else
            {
                total_high_pulse += 1;
            }
            if let Some(module) = modules.get_mut(&to)
            {
                let new_pulses = module.process(&from, pulse);
                new_pulses.iter().for_each(|data|{
                    queue.push_back(data.clone());
                })
            }
        }
        else {
            break;
        }
    }

    (total_low_pulse, total_high_pulse)
}

#[derive(Debug, Clone, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    BroadCast,
}

#[derive(Debug, Clone, PartialEq)]
struct Module {
    module_type: ModuleType,
    name: String,
    destinations: Vec<String>,
    remember: HashMap<String, i32>,
    on: bool,
}

impl Module {
    fn new(data: &(ModuleType, String, Vec<String>)) -> Self {
        Self {
            remember: HashMap::new(),
            module_type: data.0.clone(),
            name: data.1.to_owned(),
            destinations: data.2.clone(),

            on: false,
        }
    }

    fn prepare(&mut self, input_modules: &Vec<String>) {
        input_modules.iter().for_each(|name| {
            self.remember.insert(name.to_owned(), 0);
        });
    }

    fn process(&mut self, input_module: &String, pulse: i32) -> Vec<(String, String, i32)> {
        match self.module_type {
            ModuleType::FlipFlop => {
                if pulse == 0 {
                    // On
                    if self.on == true {
                        // turn off
                        self.on = false;
                        // send low pulse
                        self.destinations
                            .iter()
                            .map(|label| (self.name.to_owned(), label.to_owned(), 0))
                            .collect()
                    }
                    // Off
                    else {
                        // turn on
                        self.on = true;
                        // send high pulse
                        self.destinations
                            .iter()
                            .map(|label| (self.name.to_owned(),label.to_owned(), 1))
                            .collect()
                    }
                } else {
                    vec![]
                }
            }
            ModuleType::Conjunction => {
                if let Some(on_state) = self.remember.get_mut(input_module) {
                    *on_state = pulse;
                } else {
                    unreachable!("It should have all input remember")
                }

                let all_high = self.remember.iter().all(|(_, pulse)| *pulse > 0);

                let returning_pulse = if all_high == true { 0 } else { 1 };

                self.destinations
                    .iter()
                    .map(|label| (self.name.to_owned(), label.to_owned(), returning_pulse))
                    .collect()
            }
            ModuleType::BroadCast => self
                .destinations
                .iter()
                .map(|label| (self.name.to_owned(), label.to_owned(), pulse))
                .collect(),
        }
    }
}

fn prepare_data(input: &Vec<(ModuleType, String, Vec<String>)>) -> HashMap<String, Module> {
    let mut modules: Vec<Module> = input.par_iter().map(|data| Module::new(data)).collect();

    // module_name: input_of_this_module
    let mut input_modules: HashMap<String, Vec<String>> = HashMap::new();
    modules.iter().for_each(|module| {
        module.destinations.iter().for_each(|output| {
            if let Some(input_list) = input_modules.get_mut(output) {
                input_list.push(module.name.to_owned());
            } else {
                input_modules.insert(output.to_owned(), vec![module.name.to_owned()]);
            }
        })
    });

    modules.iter_mut().for_each(|module| {
        if let Some(input_list) = input_modules.get(&module.name) {
            module.prepare(input_list);
        }
    });

    modules
        .iter()
        .map(|module| (module.name.to_owned(), module.clone()))
        .collect()
}

fn parse_input(input: &Vec<&str>) -> Vec<(ModuleType, String, Vec<String>)> {
    input
        .iter()
        .filter_map(|text| {
            let split: Vec<&str> = text.split(" -> ").collect_vec();
            let destinations: Vec<String> = split[1]
                .replace(" ", "")
                .split(",")
                .map(|v| v.to_string())
                .collect();

            let module = split[0].to_string();
            if module.contains("%") == true {
                Some((ModuleType::FlipFlop, module[1..].to_string(), destinations))
            } else if module.contains("&") == true {
                Some((
                    ModuleType::Conjunction,
                    module[1..].to_string(),
                    destinations,
                ))
            } else if module.contains("broadcaster") == true {
                Some((ModuleType::BroadCast, module.to_string(), destinations))
            } else {
                None
            }
        })
        .collect()
}

const TEST_INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

const TEST_INPUT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_pulse_1000_1(){
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let data = parse_input(&lines);
        assert_eq!(data.len(), 5);

        let mut data = prepare_data(&data);
        let mut total_low_pulse = 0;
        let mut total_high_pulse = 0;
        (0..1000).for_each(|_|{
            let (low,high) = send_pulse(&mut data);
            total_low_pulse += low;
            total_high_pulse += high;
        });
        
        assert_eq!(total_low_pulse * total_high_pulse, 32000000);
    }

    #[test]
    fn test_send_pulse_1000_2(){
        let lines: Vec<&str> = TEST_INPUT_2.lines().collect();
        let data = parse_input(&lines);
        assert_eq!(data.len(), 5);

        let mut data = prepare_data(&data);
        let mut total_low_pulse = 0;
        let mut total_high_pulse = 0;
        (0..1000).for_each(|_|{
            let (low,high) = send_pulse(&mut data);
            total_low_pulse += low;
            total_high_pulse += high;
        });
        
        assert_eq!(total_low_pulse * total_high_pulse, 11687500);
    }
    
    #[test]
    fn test_parse_input() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let data = parse_input(&lines);
        assert_eq!(data.len(), 5);

        let data = prepare_data(&data);
        assert_eq!(data.len(), 5);
        assert_eq!(data["a"].destinations, vec!["b".to_owned()]);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day20::part_01(&lines);
        assert_eq!(result, 32000000);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day20::part_02(&lines);
        assert_eq!(result, 2286);
    }
}
