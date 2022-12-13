use std::{time::Instant, collections::VecDeque};

use hashbrown::HashMap;
use itertools::Itertools;

use crate::file_handler::FileHandler;


pub struct Day13 {}

impl Day13 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_13_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day13::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day13::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_13".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> usize {
        let packets = parsing(lines);
        let packets = packets.iter().map(|p| converting_raw_data(p.clone())).collect_vec();
        
        let mut total_right_order = 0;
        for index in (0..packets.len()).step_by(2)
        {
            // println!("Pair {}:",index/2 + 1 );
            let result = compare_data(&packets[index],&packets[index + 1]);
            // println!("Result: {:?}\n---", result);
            if matches!(result, CompareDataResult::TRUE)
            {
                total_right_order += index/2 + 1;
            }
            else
            {
                // println!("Pair: {}\n{:?}\n\n{:?}\n---\n\n", index/2 + 1, packets[index], packets[index + 1]);
            }
        }
        
        
        return total_right_order;
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        return 0;
    }
}

const TEST_INPUT: &str = 
"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day13::part_01(&lines);
    assert_eq!(result, 13);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day13::part_02(&lines);
    assert_eq!(result, 140);
}

#[derive(Debug, Clone)]
enum Data
{
    Number(i32),
    List(VecDeque<Data>),
    
}

#[derive(Debug, Clone)]
enum CompareDataResult
{
    SKIP=0,
    TRUE=1,
    FALSE=-1
}

fn compare_data(first: &Data, second: &Data) -> CompareDataResult
{
    // println!("\ncompare_data(): \n{:?}\n{:?}", first, second);
    if matches!(first, Data::List(_)) && matches!(second, Data::List(_))
    {
        if let Data::List(mut first_list) = first.clone()
        {
            if let Data::List(mut second_list) = second.clone()
            {
                loop
                {
                    if first_list.len() == 0 && second_list.len() > 0
                    {
                        // left ran out of items
                        // println!("True: left ran out of items\n");
                        return CompareDataResult::TRUE;
                    }       
                    else if first_list.len() > 0 && second_list.len() == 0
                    {
                        // right ran out of items
                        // println!("False: right ran out of items");
                        return CompareDataResult::FALSE;
                    }
                    else if first_list.len() == 0 && second_list.len() == 0
                    {
                        // both ran out of item
                        // println!("SKIP: both ran out of item\n");
                        return CompareDataResult::SKIP;
                    }
                    else
                    {
                        let first_value = first_list.pop_front().unwrap();
                        let second_value = second_list.pop_front().unwrap();
                        // both are number
                        if let Some(result) = compare_number(&first_value, &second_value)
                        {
                            if matches!(result, CompareDataResult::SKIP)
                            {
                                continue;
                            }
                            // println!("{:?} compare_number()", result);
                            return result;
                        }
                        else if matches!(first_value, Data::List(_)) && matches!(second_value, Data::List(_))
                        {
                            // both are list. So, recursive
                            let result = compare_data(&first_value, &second_value);
                            if matches!(result, CompareDataResult::SKIP)
                            {
                                continue;
                            }
                            
                            // println!("{:?}: Recursive compare data: {:?} --- {:?}",result, first_value, second_value);
                            return result;
                        }
                        else
                        {
                            // first value need to put it as a list
                            if matches!(first_value, Data::Number(_))
                            {
                                let first_value = convert_number_to_list(&first_value);
                                first_list.push_front(first_value);
                                second_list.push_front(second_value);
                            }
                            else
                            {
                                let second_value = convert_number_to_list(&second_value);
                                first_list.push_front(first_value);
                                second_list.push_front(second_value);
                            }
                        }
                    }
                }
            }
        }
    }

    panic!("It should not be here");
}

fn convert_number_to_list(data: &Data) -> Data
{
    if let Data::Number(value) = data
    {
        let mut new_list: VecDeque<Data> = VecDeque::new();
        new_list.push_back(Data::Number(*value));
        // new_list.push_back(Data::End);
        return Data::List(new_list);
    }

    panic!("convert_number_to_list():: data has to be number");
}

fn compare_number(first: &Data, second: &Data) -> Option<CompareDataResult>
{

    if let Data::Number(first_value) = first
    {
        if let Data::Number(second_value) = second
        {
            // println!("Compare {} VS {}", first_value, second_value);
            if *first_value < *second_value
            {
                return Some(CompareDataResult::TRUE);
            }
            else if *first_value > *second_value
            {
                return Some(CompareDataResult::FALSE);
            }
            else
            {
                return Some(CompareDataResult::SKIP);
            }
        }
    }

    return None;
    // panic!("compare_number():: Both data has to be number");
}

fn converting_raw_data(packet: VecDeque<String>) -> Data
{
    // println!("converting_raw_data:  {:?}", packet);

    let mut packet = packet.clone();
    remove_outer_bracket(&mut packet);
    
    let mut list = VecDeque::new();
    while packet.len() > 0
    {
        let first = packet.pop_front().unwrap();
        let data = match first.parse::<i32>()
        {
            Ok(v) => {
                Data::Number(v)
            }
            Err(_) => {
                if first == "["
                {
                    let closed_index = find_closed_list_index(&packet, 0);

                    let mut new_list = packet.drain(..=closed_index).collect::<VecDeque<String>>();
                    new_list.push_front("[".to_owned());
                    
                    converting_raw_data(new_list)
                }
                else
                {
                    continue;
                }
                
            }
        };
        
        list.push_back(data);
    }
    Data::List(list)
}

fn remove_outer_bracket(packet: &mut VecDeque<String>)
{
    packet.pop_back();
    packet.pop_front();
}

fn find_closed_list_index(packet: &VecDeque<String>, start_index: usize) -> usize
{
    
    let mut open = 0;
    for (index, value) in packet.iter().enumerate()
    {
        if index >= start_index
        {
            if value == "["
            {
                open += 1;
            }
            else if value == "]"
            {
                if open > 0
                {
                    open -= 1;
                }
                else
                {
                    return index;
                }
            }
        }
    }

    panic!("find_group_range: something wrong");
}

fn parsing(lines:&Vec<&str>) -> Vec<VecDeque<String>>
{
    let mut result = vec![];
    for line in lines.iter()
    {   
        if line.len() > 0
        {
            let splited : Vec<char> = line.chars().map(|c|  c.to_owned()).collect_vec();
            let mut packet = VecDeque::new();
            let mut temp = String::new();
            for c in splited.iter()
            {
                if *c == '['
                {
                    packet.push_back(c.to_string());
                }
                else if *c == ',' || *c == ']'
                {
                    if temp.len() > 0
                    {
                        packet.push_back(temp.to_owned());
                        temp.clear();
                    }
                    

                    if *c == ']'
                    {
                        packet.push_back(c.to_string());
                    }
                    
                }
                else
                {
                    temp.push_str(c.to_string().as_str());
                }
            }

            result.push(packet)
        }
        
    }

    result
}