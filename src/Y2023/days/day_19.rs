use crate::file_handler::FileHandler;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day19 {}

impl Day19 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_19_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day19::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day19::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_19".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> i64 {
        let (rules, parts) = parse_input_ex(lines);
        parts
            .iter()
            .map(|part| {
                let mut start_rules = "in".to_owned();
                let mut value = 0;
                loop {
                    if let Some(rule) = rules.get(&start_rules) {
                        start_rules = rule.check_ex(*part);
                    } else {
                        if start_rules == "A" {
                            value = calculate_rating_ex(*part);
                        }
                        break;
                    }
                }

                value as i64
            })
            .sum()
    }

    fn part_02(lines: &Vec<&str>) -> i64 {
        let (rules, parts) = parse_input_ex(lines);

        // let total_index: i64 = 4000 * 4000 * 4000 * 4000;
        let total_index = 1;
        (0..=total_index)
            .into_par_iter()
            .map(|index| {
                let s = index % 4000;
                let a = (index / 4000) % 4000;
                let m = ((index / 4000) / 4000) % 4000;
                let ex = (((index / 4000) / 4000) / 4000);

                let mut start_rules = "in".to_owned();
                let mut value = 0;
                let part = (ex as i32 + 1, m as i32 + 1, a as i32 + 1, s as i32 + 1);
                // loop
                // {
                //     if let Some(rule) = rules.get(&start_rules)
                //     {
                //         start_rules = rule.check_ex(part);
                //     }
                //     else
                //     {
                //         if start_rules == "A"
                //         {
                //             value = 1;
                //         }
                //         break;
                //     }
                // }

                value as i64
            })
            .sum()
    }
}

fn parse_input(input: &Vec<&str>) -> (HashMap<String, Rule>, Vec<Vec<(PartType, i32)>>) {
    let index = input
        .iter()
        .find_position(|line| line.len() == 0)
        .unwrap()
        .0;
    let rules_inputs = input[..index].iter().map(|v| *v).collect();
    let rules = parse_rules(&rules_inputs);
    let parts_input = input[(index + 1)..].iter().map(|v| *v).collect();
    let parts = parse_parts(&parts_input);

    (rules, parts)
}

fn parse_input_ex(input: &Vec<&str>) -> (HashMap<String, Rule>, Vec<(i32, i32, i32, i32)>) {
    let index = input
        .iter()
        .find_position(|line| line.len() == 0)
        .unwrap()
        .0;
    let rules_inputs = input[..index].iter().map(|v| *v).collect();
    let rules = parse_rules(&rules_inputs);
    let parts_input = input[(index + 1)..].iter().map(|v| *v).collect();
    let parts = parse_parts_ex(&parts_input);

    (rules, parts)
}

fn parse_rules(input: &Vec<&str>) -> HashMap<String, Rule> {
    let rules: Vec<Rule> = input.iter().map(|text| Rule::from_str(*&text)).collect();
    rules
        .iter()
        .map(|r| (r.entrance.to_owned(), r.clone()))
        .collect()
}

fn parse_parts(input: &Vec<&str>) -> Vec<Vec<(PartType, i32)>> {
    //{x=787,m=2655,a=1222,s=2876}
    input
        .iter()
        .map(|text| {
            let text = text.replace("{", "").replace("}", "");
            text.split(",")
                .map(|v| {
                    let (part, _, value) = PartType::from_str(v);
                    (part, value)
                })
                .collect()
        })
        .collect()
}

fn parse_parts_ex(input: &Vec<&str>) -> Vec<(i32, i32, i32, i32)> {
    //{x=787,m=2655,a=1222,s=2876}
    input
        .iter()
        .map(|text| {
            let mut ex = 0;
            let mut m = 0;
            let mut a = 0;
            let mut s = 0;
            let text = text.replace("{", "").replace("}", "");
            text.split(",").for_each(|v| {
                let (part, _, value) = PartType::from_str(v);
                match part {
                    PartType::Ex => ex = value,
                    PartType::Musical => m = value,
                    PartType::Aero => a = value,
                    PartType::Shiny => s = value,
                    _ => {}
                }
            });
            (ex, m, a, s)
        })
        .collect()
}

fn calculate_rating(part: &Vec<(PartType, i32)>) -> i32 {
    part.iter().map(|(_, value)| *value).sum()
}

fn calculate_rating_ex(part: (i32, i32, i32, i32)) -> i32 {
    part.0 + part.1 + part.2 + part.3
}

#[derive(Debug, Clone, PartialEq)]
enum PartType {
    Ex,
    Musical,
    Aero,
    Shiny,
    Exit,
}

impl PartType {
    fn to_usize(&self) -> usize {
        match self {
            Self::Ex => 0,
            Self::Musical => 1,
            Self::Aero => 2,
            Self::Shiny => 3,
            _ => unreachable!("It should not be here"),
        }
    }
    fn match_conditions(&self, spec: &PartType) -> bool {
        false
    }

    fn from_str(input: &str) -> (PartType, i32, i32) {
        let operand_index = input
            .to_string()
            .find(|c| c == '<' || c == '>' || c == '=')
            .unwrap();

        let p_type = input[..operand_index].to_owned();
        let operand = match input[operand_index..(operand_index + 1)]
            .to_owned()
            .as_str()
        {
            "<" => -1,
            ">" => 1,
            "=" => 0,
            _ => unreachable!("It should not be here"),
        };
        let value = input[(operand_index + 1)..]
            .to_string()
            .parse::<i32>()
            .unwrap();

        match p_type.as_str() {
            "x" => (PartType::Ex, operand, value),
            "s" => (PartType::Shiny, operand, value),
            "m" => (PartType::Musical, operand, value),
            "a" => (PartType::Aero, operand, value),
            _ => unreachable!("It should not be here"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Rule {
    // entrance label. It is the rule name
    entrance: String,

    conditions: Vec<((PartType, i32, i32), String)>,
}
impl Rule {
    fn from_str(input: &str) -> Self {
        let split: Vec<&str> = input.split("{").collect();
        let label = split[0].to_owned();
        let conditions_text = split[1].to_owned().replace("}", "");
        let split: Vec<&str> = conditions_text.split(",").collect();
        let conditions = split
            .iter()
            .map(|v| {
                let text: Vec<&str> = v.split(":").collect();
                if text.len() == 2 {
                    let part_type = PartType::from_str(text[0]);
                    let go_to = text[1].to_owned();
                    (part_type, go_to)
                } else {
                    let go_to = text[0].to_owned();
                    ((PartType::Exit, 0, 0), go_to)
                }
            })
            .collect();

        Self {
            entrance: label,
            conditions: conditions,
        }
    }

    fn check(&self, part: &Vec<(PartType, i32)>) -> String {
        for ((condition, operand, cond_value), go_to) in self.conditions.iter() {
            for (part_attribute, spec_vale) in part.iter() {
                if part_attribute == condition {
                    if *operand < 0 {
                        if spec_vale < cond_value {
                            return go_to.to_owned();
                        }
                    } else if *operand > 0 {
                        if spec_vale > cond_value {
                            return go_to.to_owned();
                        }
                    }
                }
            }
        }

        self.conditions.last().unwrap().1.to_owned()
    }

    fn check_ex(&self, part: (i32, i32, i32, i32)) -> String {
        let part_values = [part.0, part.1, part.2, part.3];
        for ((condition, operand, cond_value), go_to) in self.conditions.iter() {
            if matches!(condition, PartType::Exit) == false {
                let condition_index = condition.to_usize();
                let spec_value = part_values[condition_index];

                if *operand < 0 {
                    if spec_value < *cond_value {
                        return go_to.to_owned();
                    }
                } else if *operand > 0 {
                    if spec_value > *cond_value {
                        return go_to.to_owned();
                    }
                }
            }
        }

        self.conditions.last().unwrap().1.to_owned()
    }
}

const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input: Vec<&str> = TEST_INPUT.lines().collect();
        let (rules, parts) = parse_input(&input);
        assert_eq!(rules.len(), 11);
        assert_eq!(rules["px"].entrance, "px".to_owned());

        assert_eq!(parts.len(), 5);

        let first_rule = rules.get("in").unwrap();
        assert_eq!(first_rule.check(&parts[0]), "qqz".to_owned());
        assert_eq!(rules["qqz"].check(&parts[0]), "qs".to_owned());
        assert_eq!(rules["qs"].check(&parts[0]), "lnx".to_owned());
        assert_eq!(rules["lnx"].check(&parts[0]), "A".to_owned());
        assert_eq!(calculate_rating(&parts[0]), 7540);

        //{x=1679,m=44,a=2067,s=496}: in -> px -> rfg -> gd -> R
        assert_eq!(rules["in"].check(&parts[1]), "px".to_owned());
        assert_eq!(rules["px"].check(&parts[1]), "rfg".to_owned());
        assert_eq!(rules["rfg"].check(&parts[1]), "gd".to_owned());
        assert_eq!(rules["gd"].check(&parts[1]), "R".to_owned());

        //{x=2036,m=264,a=79,s=2244}: in -> qqz -> hdj -> pv -> A
        assert_eq!(rules["in"].check(&parts[2]), "qqz".to_owned());
        assert_eq!(rules["qqz"].check(&parts[2]), "hdj".to_owned());
        assert_eq!(rules["hdj"].check(&parts[2]), "pv".to_owned());
        assert_eq!(rules["pv"].check(&parts[2]), "A".to_owned());
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day19::part_01(&lines);
        assert_eq!(result, 19114);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day19::part_02(&lines);
        assert_eq!(result, 167409079868000);
    }
}
