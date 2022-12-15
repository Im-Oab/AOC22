use std::{collections::HashSet, hash::Hash, time::Instant};

use hashbrown::HashMap;

use crate::file_handler::FileHandler;

pub struct Day15 {}

impl Day15 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_15_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day15::part_01(&lines, 2000000);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day15::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_15".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// I start with the dumb version that puts all the positions scanned by sensors in the HashSet.
    /// It can return the correct result for the example input, but it took ages to get a result from the actual input.
    /// So, I have to revise the code to improve the performance. (Use range instead of putting everything in HashSet)
    fn part_01(lines: &Vec<&str>, at_y: i32) -> usize {
        let data = parsing(lines);
        let mut area = Area::new();
        for (sensor, beacon) in data.iter() {
            let node = Sensor::new(sensor, beacon);
            area.add_sensor(&node);
        }

        area.mapping_scanned_area();

        return area.count_unavailable_place_at(at_y);
    }

    /// Part 2, I tried to look into the rust document about combining multiple ranges and finding a spot inside,
    /// and I could not. Finally, I write a function that will start from the maximum range possible in the row
    /// and trim it down using the range of sensors until it has one spot inside or no spot.
    fn part_02(lines: &Vec<&str>) -> u128 {
        let data = parsing(lines);
        let mut area = Area::new();
        for (sensor, beacon) in data.iter() {
            let node = Sensor::new(sensor, beacon);
            area.add_sensor(&node);
        }

        area.mapping_scanned_area();

        let mut tuning_frequency: u128 = 0;
        for y in 0..=4000000 {
            if let Some((tx, ty)) = area.find_beacon_available_spot_at(y) {
                tuning_frequency = tx as u128 * 4000000 + ty as u128;
                break;
            }

            if y > area.bottom {
                panic!("It should have the result before reach the bottom");
            }
        }

        return tuning_frequency;
    }
}

const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day15::part_01(&lines, 10);
    assert_eq!(result, 26);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day15::part_02(&lines);
    assert_eq!(result, 56000011);
}

#[derive(Clone)]
struct Sensor {
    position: (i32, i32),
    nearby_beacon: (i32, i32),
    distance: i32,
    scanned_area: HashMap<i32, (i32, i32)>,
}

impl Sensor {
    fn new(position: &(i32, i32), beacon: &(i32, i32)) -> Self {
        let distance = (position.0 - beacon.0).abs() + (position.1 - beacon.1).abs();
        Self {
            position: position.clone(),
            nearby_beacon: beacon.clone(),
            distance: distance,
            scanned_area: HashMap::new(),
        }
    }

    fn mapping(&mut self) {
        self.scanned_area.clear();

        for y in (-self.distance)..=self.distance {
            let total_columns = (self.distance - y.abs()) * 2 + 1;
            let start_x = -total_columns / 2;
            let scanned_y = self.position.1 + y;

            let left = self.position.0 + start_x;
            let right = self.position.0 + start_x + total_columns - 1;

            self.scanned_area.insert(scanned_y, (left, right));
        }
    }

    fn is_scanned(&self, x: i32, y: i32) -> bool {
        if x >= self.left() && x <= self.right() {
            if y >= self.top() && y <= self.bottom() {
                if let Some((left, right)) = self.scanned_area.get(&y) {
                    return (*left..=*right).contains(&x);
                }
            }
        }

        false
    }

    fn top(&self) -> i32 {
        self.position.1 - self.distance
    }

    fn bottom(&self) -> i32 {
        self.position.1 + self.distance
    }

    fn left(&self) -> i32 {
        self.position.0 - self.distance
    }

    fn right(&self) -> i32 {
        self.position.0 + self.distance
    }

    fn left_at(&self, y: i32) -> Option<i32> {
        let y = y - self.position.1;
        let total_columns = (self.distance - y.abs()) * 2 + 1;
        if total_columns > 0 {
            let start_x = -total_columns / 2;

            let left = self.position.0 + start_x;

            Some(left)
        } else {
            None
        }
    }

    fn right_at(&self, y: i32) -> Option<i32> {
        let y = y - self.position.1;
        let total_columns = (self.distance - y.abs()) * 2 + 1;
        if total_columns > 0 {
            let start_x = -total_columns / 2;

            let right = self.position.0 + start_x + total_columns - 1;

            Some(right)
        } else {
            None
        }
    }
}

struct Area {
    sensors: Vec<Sensor>,
    scanned_area: HashSet<(i32, i32)>,
    sensor_positions: HashSet<(i32, i32)>,
    beacon_positions: HashSet<(i32, i32)>,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Area {
    fn new() -> Self {
        Self {
            sensors: vec![],
            scanned_area: HashSet::new(),
            sensor_positions: HashSet::new(),
            beacon_positions: HashSet::new(),
            left: i32::MAX,
            right: i32::MIN,
            top: i32::MAX,
            bottom: i32::MIN,
        }
    }

    fn add_sensor(&mut self, sensor: &Sensor) {
        self.sensor_positions.insert(sensor.position.clone());
        self.beacon_positions.insert(sensor.nearby_beacon.clone());
        self.sensors.push(sensor.clone());
    }

    fn mapping_scanned_area(&mut self) {
        self.scanned_area.clear();
        for sensor in self.sensors.iter_mut() {
            sensor.mapping();

            self.left = self.left.min(sensor.left());
            self.right = self.right.max(sensor.right());
            self.top = self.top.min(sensor.top());
            self.bottom = self.bottom.max(sensor.bottom());
        }
    }

    fn draw(&self) {
        for y in self.top..=self.bottom {
            self.draw_line(y);
        }

        println!("---\n");
    }

    fn draw_line(&self, y: i32) {
        print!("{: >5},{: >5} ", self.left, y);
        {
            for x in self.left..=self.right {
                let key = (x, y);
                if self.sensor_positions.contains(&key) {
                    print!("S");
                } else if self.beacon_positions.contains(&key) {
                    print!("B");
                } else if self.is_scanned(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("  {: >5},{: >5}", self.right, y);
        }
    }

    fn count_unavailable_place_at(&self, y: i32) -> usize {
        let mut total_count = 0;
        for x in self.left..=self.right {
            let key = (x, y);
            if self.sensor_positions.contains(&key) {
                continue;
            } else if self.beacon_positions.contains(&key) {
                continue;
            } else {
                for sensor in self.sensors.iter() {
                    if sensor.is_scanned(x, y) {
                        total_count += 1;
                        break;
                    }
                }
            }
        }

        total_count
    }

    fn is_scanned(&self, x: i32, y: i32) -> bool {
        let key = (x, y);
        if self.sensor_positions.contains(&key) {
        } else if self.beacon_positions.contains(&key) {
        } else {
            for sensor in self.sensors.iter() {
                if sensor.is_scanned(x, y) {
                    return true;
                }
            }
        }
        false
    }

    fn find_beacon_available_spot_at(&self, y: i32) -> Option<(i32, i32)> {
        let mut most_left = self.left_at(y);
        let mut most_right = self.right_at(y);
        if most_left == most_right {
            return None;
        }

        let mut loop_count = 0;
        'outer: loop {
            for sensor in self.sensors.iter() {
                if let Some(sensor_left) = sensor.left_at(y) {
                    if let Some(sensor_right) = sensor.right_at(y) {
                        if most_left >= sensor_left && most_left <= sensor_right {
                            most_left = sensor_right;
                        } else if most_left + 1 == sensor_left {
                            most_left = sensor_left;
                        }

                        if most_right >= sensor_left && most_right <= sensor_right {
                            most_right = sensor_left;
                        } else if most_right - 1 == sensor_right {
                            most_right = sensor_right;
                        }

                        if most_left >= most_right {
                            break 'outer;
                        }
                    }
                }
            }

            if loop_count > 5 && (most_right - most_left).abs() == 2 {
                return Some((most_left + 1, y));
            }

            loop_count += 1;
        }

        None
    }

    fn left_at(&self, y: i32) -> i32 {
        let mut most_left = i32::MAX;
        for sensor in self.sensors.iter() {
            most_left = most_left.min(sensor.left_at(y).unwrap_or(i32::MAX))
        }

        most_left
    }

    fn right_at(&self, y: i32) -> i32 {
        let mut most_right = i32::MIN;
        for sensor in self.sensors.iter() {
            most_right = most_right.max(sensor.right_at(y).unwrap_or(i32::MIN));
        }

        most_right
    }
}

fn parsing(lines: &Vec<&str>) -> Vec<((i32, i32), (i32, i32))> {
    let mut result = vec![];
    for line in lines.iter() {
        let splited: Vec<&str> = line.split(": closest beacon is at x=").collect();
        let sensor_part = splited[0].replace("Sensor at x=", "").replace(" y=", "");
        let (sensor_x, sensor_y) = sensor_part.split_once(",").unwrap();
        let sensor_x = sensor_x.parse::<i32>().unwrap();
        let sensor_y = sensor_y.parse::<i32>().unwrap();

        let beacon_part = splited[1].replace(" y=", "");
        let (beacon_x, beacon_y) = beacon_part.split_once(",").unwrap();
        let beacon_x = beacon_x.parse::<i32>().unwrap();
        let beacon_y = beacon_y.parse::<i32>().unwrap();

        result.push(((sensor_x, sensor_y), (beacon_x, beacon_y)));
    }

    result
}
