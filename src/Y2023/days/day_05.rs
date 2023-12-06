use rayon::prelude::*;
use std::time::Instant;

use crate::file_handler::FileHandler;

pub struct Day05 {}

impl Day05 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_05_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day05::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day05::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_05".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> u128 {
        let indexes = find_mapping_indexes(lines);
        let mut data = parsing_seeds(lines[indexes[0]]);

        for index in 1..7 {
            data = mapping_data(lines, indexes[index] + 1, indexes[index + 1] - 1, &data);
        }

        data = mapping_data(lines, indexes[7] + 1, lines.len(), &data);

        data.sort();
        return *data.first().unwrap();
    }

    fn part_02(lines: &Vec<&str>) -> u128 {
        let indexes = find_mapping_indexes(lines);
        let seeds = parsing_seeds(lines[indexes[0]]);
        println!("Prepare seeds");
        let mut data = parsing_seed_ranges(&seeds);

        println!("Start Mapping");
        for index in 1..7 {
            data = mapping_data(lines, indexes[index] + 1, indexes[index + 1] - 1, &data);
        }

        data = mapping_data(lines, indexes[7] + 1, lines.len(), &data);
        println!("Finish Mapping");

        return *data.par_iter().min().unwrap();
    }
}

fn mapping_data(
    input: &Vec<&str>,
    start_index: usize,
    end_index: usize,
    data: &Vec<u128>,
) -> Vec<u128> {
    let mut mappers = vec![];
    for index in start_index..end_index {
        let input = input[index];
        let mapper = parsing_mapping(input);
        mappers.push(mapper);
    }

    let par_iter = data.par_iter().map(|data_number| {
        for mapper in mappers.iter() {
            if let Some(soil_number) = mapper.get_destination(*data_number) {
                return soil_number;
            }
        }

        return *data_number;
    });

    par_iter.collect()
}

fn find_mapping_indexes(input: &Vec<&str>) -> Vec<usize> {
    let mut result = vec![];
    let keywords = vec![
        "seeds:",
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    for (index, line) in input.iter().enumerate() {
        for key in keywords.iter() {
            if line.contains(key) {
                result.push(index);
                break;
            }
        }
    }

    result
}

fn parsing_seeds(input: &str) -> Vec<u128> {
    let values = input.replace("seeds: ", "");
    let values: Vec<&str> = values.split(" ").collect();
    let mut result = vec![];
    for value in values.iter() {
        if let Ok(number) = value.parse::<u128>() {
            result.push(number);
        }
    }

    result
}

fn parsing_seed_ranges(seeds: &Vec<u128>) -> Vec<u128> {
    let mut result = vec![];
    for index in (0..(seeds.len() - 1)).step_by(2) {
        let start = seeds[index];
        let end = start + seeds[index + 1];

        let v: Vec<_> = (start..end).collect();
        let par_iter = v.par_iter().map(|number| *number);
        let mut data: Vec<u128> = par_iter.collect();

        result.append(&mut data);
    }

    result
}

fn parsing_mapping(input: &str) -> DataMapping {
    let values: Vec<&str> = input.split(" ").collect();
    DataMapping::new(
        values[1].parse::<u128>().unwrap(),
        values[0].parse::<u128>().unwrap(),
        values[2].parse::<u128>().unwrap(),
    )
}

#[derive(Debug, PartialEq, Eq)]
struct DataMapping {
    source: u128,
    destination: u128,
    range: u128,
}

impl DataMapping {
    fn new(source: u128, destination: u128, range: u128) -> Self {
        Self {
            source: source,
            destination: destination,
            range: range,
        }
    }

    fn get_destination(&self, source: u128) -> Option<u128> {
        let start_source = self.source;
        let end_source = self.source + self.range;

        let start_destination = self.destination;
        if start_source <= source && source < end_source {
            let diff = source - start_source;
            let result = start_destination + diff;
            Some(result)
        } else {
            None
        }
    }
}

const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_seeds() {
        let input = "seeds: 79 14 55 13";
        let result = parsing_seeds(input);
        assert_eq!(result, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_parsing_mapping() {
        let input = "50 98 2";
        let data = parsing_mapping(input);
        assert_eq!(data, DataMapping::new(98, 50, 2));
    }

    #[test]
    fn test_get_destination_from_mapping() {
        let data = parsing_mapping("50 98 2");
        assert_eq!(data.get_destination(97), None);
        assert_eq!(data.get_destination(98), Some(50));
        assert_eq!(data.get_destination(99), Some(51));
        assert_eq!(data.get_destination(100), None);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day05::part_01(&lines);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day05::part_02(&lines);
        assert_eq!(result, 46);
    }
}
