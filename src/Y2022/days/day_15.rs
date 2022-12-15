use std::{collections::HashSet, hash::Hash, time::Instant};

use crate::file_handler::FileHandler;

pub struct Day15 {}

impl Day15 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2022/inputs/day_15_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day15::part_01(&lines,2000000 );
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

    fn part_01(lines: &Vec<&str>, at_y: i32) -> usize {
        let data = parsing(lines);
        let mut area = Area::new();
        for (sensor, beacon) in data.iter() {
            let node = Sensor::new(sensor, beacon);
            area.add_sensor(&node);
            
        }

        area.mapping_scanned_area();
        area.draw();

        return area.count_unavailable_place_at(at_y);
    }

    fn part_02(lines: &Vec<&str>) -> i32 {
        return 0;
    }
}

const TEST_INPUT: &str = 
"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
    assert_eq!(result, 0);
}

#[derive(Clone)]
struct Sensor {
    position: (i32, i32),
    nearby_beacon: (i32, i32),
    distance: i32,
}

impl Sensor {
    fn new(position: &(i32, i32), beacon: &(i32, i32)) -> Self {
        let distance = (position.0 - beacon.0).abs() + (position.1 - beacon.1).abs();

        println!(
            "Sensor distance: ({}).abs() + ({}).abs() = {}",
            (position.0 - beacon.0).abs(), (position.1 - beacon.1).abs(), distance
        );
        Self {
            position: position.clone(),
            nearby_beacon: beacon.clone(),
            distance: distance,
        }
    }

    fn scanned_area(&self) -> HashSet<(i32, i32)> {
        let mut result = HashSet::new();
        for y in (-self.distance)..=self.distance {
            let total_columns = (self.distance - y.abs()) * 2 + 1;
            let start_x = -total_columns / 2;
            let scanned_y = self.position.1 + y;
            for x in 0..total_columns {
                let scanned_x = self.position.0 + (start_x + x);
                result.insert((scanned_x, scanned_y));
            }
        }

        result
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
        for sensor in self.sensors.iter() {
            let area = sensor.scanned_area();
            self.scanned_area.extend(area);

            self.left = self.left.min(sensor.left());
            self.right = self.right.max(sensor.right());
            self.top = self.top.min(sensor.top());
            self.bottom = self.bottom.max(sensor.bottom());
        }
    }

    fn draw(&self) {
        println!("Left,Top: {}, {}", self.left, self.top);
        println!("Right,Bottom: {}, {}", self.right, self.bottom);
        for y in self.top..=self.bottom {
            for x in self.left..=self.right {
                let key = (x, y);
                if self.sensor_positions.contains(&key) {
                    print!("S");
                } else if self.beacon_positions.contains(&key) {
                    print!("B");
                } else if self.scanned_area.contains(&key) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }

        println!("---\n");
    }

    fn count_unavailable_place_at(&self, y: i32) -> usize
    {
        let mut total_count = 0;
        for x in self.left..=self.right
        {
            let key = (x, y);
            if self.sensor_positions.contains(&key)
            {
                continue;
            }
            else if self.beacon_positions.contains(&key)
            {
                continue;
            }
            else if self.scanned_area.contains(&key)
            {
                total_count+=1;
            }
        }

        total_count
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
