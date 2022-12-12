use std::time::Instant;

use itertools::Itertools;

use crate::file_handler::FileHandler;

#[derive(Debug, Clone)]
enum Operations {
    Multiply(u128),
    MultiplyItSelf,
    Add(u128),
}

// Monkey 0:
// Starting items: 79, 98
// Operation: new = old * 19
// Test: divisible by 23
//   If true: throw to monkey 2
//   If false: throw to monkey 3

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    items: Vec<u128>,
    test_divisible: u128,
    operation: Operations,
    true_throw: usize,
    false_throw: usize,
    total_inspected: u128,
}

impl Monkey {
    fn from(lines: &Vec<&str>) -> Self {
        let name = (lines[0].split(":").collect::<Vec<&str>>())[0]
            .to_owned()
            .to_lowercase();
        let items: Vec<u128> = {
            let item_input = lines[1].replace("Starting items:", "").replace(" ", "");
            item_input
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| v.parse::<u128>().unwrap())
                .collect_vec()
        };
        let operation = {
            let input = (lines[2]
                .split("Operation: new = old ")
                .collect::<Vec<&str>>())[1]
                .clone();
            let (operand, value) = input.split_once(" ").unwrap();
            match operand {
                "*" => match value.parse::<u128>() {
                    Ok(v) => Operations::Multiply(v),
                    _ => Operations::MultiplyItSelf,
                },
                "+" => Operations::Add(value.parse::<u128>().unwrap()),
                _ => {
                    panic!("Unknown logic: {}", lines[2]);
                }
            }
        };

        let test_divisible = (lines[3].split("Test: divisible by ").collect::<Vec<&str>>())[1]
            .parse::<u128>()
            .unwrap();
        let first_throw = lines[4]
            .split("If true: throw to monkey ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let second_throw = lines[5]
            .split("If false: throw to monkey ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Self {
            name: name,
            items: items,
            test_divisible: test_divisible,
            operation: operation,
            true_throw: first_throw.to_owned(),
            false_throw: second_throw.to_owned(),
            total_inspected: 0,
        }
    }

    fn inspect(&mut self) -> Vec<(usize, u128)> {
        let mut result = vec![];
        if self.items.len() > 0 {
            for item in self.items.iter() {
                let worry_level = self.worry_level(*item);
                let bored_level = self.get_bore(worry_level);

                let throw_data = self.test(bored_level);
                result.push(throw_data);
            }

            self.total_inspected += self.items.len() as u128;
            self.items.clear();
        }
        result
    }

    fn less_worry_inspect(&mut self, common_modular: u128) -> Vec<(usize, u128)> {
        let mut result = vec![];
        if self.items.len() > 0 {
            for item in self.items.iter() {
                let worry_level = self.worry_level(*item);

                let throw_data = self.test(worry_level % common_modular);
                result.push(throw_data);
            }

            self.total_inspected += self.items.len() as u128;
            self.items.clear();
        }
        result
    }

    fn worry_level(&self, item: u128) -> u128 {
        match self.operation {
            Operations::MultiplyItSelf => match item.checked_mul(item) {
                Some(v) => v,
                None => panic!("Multiply overflow: {} * {}", item, item),
            },
            Operations::Add(value) => item + value,
            Operations::Multiply(value) => match item.checked_mul(value) {
                Some(v) => v,
                None => panic!("Multiply overflow: {} * {}", item, value),
            },
        }
    }

    fn get_bore(&self, worry_level: u128) -> u128 {
        worry_level / 3
    }

    fn test(&self, worry_level: u128) -> (usize, u128) {
        if worry_level % self.test_divisible == 0 {
            (self.true_throw, worry_level)
        } else {
            (self.false_throw, worry_level)
        }
    }
}

pub struct Day11 {}

impl Day11 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/inputs/day_11_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day11::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day11::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_11".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// For part 1, It took me almost an hour to finish implementing the parsing input code.
    /// I create a struct of Monkey to keep the data and processing. Everything was easy after that.
    fn part_01(lines: &Vec<&str>) -> u128 {
        let mut monkeys = parse_input(lines);

        for _ in 1..=20 {
            round(&mut monkeys);
        }

        let mut total_inspected_list = monkeys.iter().map(|m| m.total_inspected).collect_vec();
        total_inspected_list.sort();
        let most_inspected_list: Vec<&u128> = total_inspected_list.iter().rev().take(2).collect();
        let result = most_inspected_list.iter().map(|v| **v).product();

        return result;
    }

    /// The difficulty of Part 2 rises very quickly because of the overflow of the value.
    /// And those who know will quickly solve this part, but it is not for me. I tried to fix it using `u128`,
    /// but It still overflowed. So, I even look for any crate that supports a bigger value than u128.
    /// Moreover, the program becomes very slow to solve the puzzle.

    /// I asked for the hint for this part, and Someone told me about modular.
    /// I try with a common modular value from all monkeys, and it works. :)
    fn part_02(lines: &Vec<&str>) -> u128 {
        let mut monkeys = parse_input(lines);

        let common_modular: u128 = monkeys.iter().map(|m| m.test_divisible).product();

        // let interested_round = [
        //     1, 20, 30, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000,
        // ];

        for _ in 1..=10000 {
            round_with_less_worry(&mut monkeys, common_modular);
            // if interested_round.contains(&round_number) {

            //     for monkey in monkeys.iter() {
            //         println!(
            //             "{} inspect items {} times",
            //             monkey.name, monkey.total_inspected
            //         );
            //     }

            //     println!("\n{:?}\n", monkeys);
            //     println!("----\n");
            // }
        }

        let mut total_inspected_list = monkeys.iter().map(|m| m.total_inspected).collect_vec();
        total_inspected_list.sort();
        let most_inspected_list: Vec<&u128> = total_inspected_list.iter().rev().take(2).collect();
        let result = most_inspected_list.iter().map(|v| **v).product();

        return result;
    }
}

fn round(monkeys: &mut Vec<Monkey>) {
    for index in 0..monkeys.len() {
        let current_monkey = &mut monkeys[index];
        let result = current_monkey.inspect();
        for (received_monkey_index, item) in result.iter() {
            if let Some(received_monkey) = monkeys.get_mut(*received_monkey_index) {
                received_monkey.items.push(*item);
            }
        }
    }
}

fn round_with_less_worry(monkeys: &mut Vec<Monkey>, common_modular: u128) {
    for index in 0..monkeys.len() {
        let current_monkey = &mut monkeys[index];
        let result = current_monkey.less_worry_inspect(common_modular);
        for (received_monkey_index, item) in result.iter() {
            if let Some(received_monkey) = monkeys.get_mut(*received_monkey_index) {
                received_monkey.items.push(*item);
            }
        }
    }
}
const TEST_INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day11::part_01(&lines);

    assert_eq!(result, 10605);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day11::part_02(&lines);

    assert_eq!(result, 2713310158);
}

fn parse_input(lines: &Vec<&str>) -> Vec<Monkey> {
    let mut group = vec![];
    let mut monkeys = vec![];
    for line in lines.iter() {
        if line.len() == 0 {
            monkeys.push(Monkey::from(&group));
            group.clear();
        } else {
            group.push(line.to_owned());
        }
    }

    monkeys.push(Monkey::from(&group));

    monkeys
}
