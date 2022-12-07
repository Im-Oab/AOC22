use std::{fmt::format, time::Instant};

use crate::file_handler::FileHandler;

#[derive(Debug)]
enum Commands {
    MOVE_IN(String),
    MOVE_OUT,
    LIST,
    DIRECTORY(String),
    FILE(usize, String),
}

pub struct Day07 {}

impl Day07 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/inputs/day_07_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day07::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day07::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_07".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    /// After I read the puzzle, I struggle with what kind of data structure to choose.
    /// I end up with two lists for directories and files. Furthermore, I use an iterator to look for the directory size
    /// from the file that has parents containing the directory name. If it contains,
    /// then this file should be children of the directory.
    fn part_01(lines: &Vec<&str>) -> i32 {
        let commands = self::parsing_input(lines);
        let (files, directories) = self::construct(&commands);
        let maximum_directory_size = 100000;
        let mut total_size = 0;
        for (parent, name) in directories.iter() {
            let file_parent_name = self::construct_file_parent_name(parent, name);
            let directory_size = self::get_directory_size(&file_parent_name, &files);
            if directory_size < maximum_directory_size {
                total_size += directory_size;
            }
        }
        return total_size as i32;
    }

    /// Part 2 is relatively easy after part 1. I need to look at only the most minor directory size bigger than the threshold value.
    fn part_02(lines: &Vec<&str>) -> i32 {
        let maximum_disk_space = 70000000;
        let required_disk_space = 30000000;
        let commands = self::parsing_input(lines);
        let (files, directories) = self::construct(&commands);
        let used_space = {
            let file_parent_name =
                self::construct_file_parent_name(&"".to_owned(), &"/".to_owned());
            self::get_directory_size(&file_parent_name, &files)
        };
        let current_free_space = maximum_disk_space - used_space;
        let minimum_deleting_space = required_disk_space - current_free_space;
        let mut smallest_directory_size = maximum_disk_space;
        for (parent, name) in directories.iter() {
            let file_parent_name = self::construct_file_parent_name(parent, name);
            let directory_size = self::get_directory_size(&file_parent_name, &files);
            if directory_size >= minimum_deleting_space {
                if directory_size < smallest_directory_size {
                    smallest_directory_size = directory_size;
                }
            }
        }

        return smallest_directory_size as i32;
    }
}

fn construct_file_parent_name(parent: &String, directory_name: &String) -> String {
    if parent.len() == 0 {
        directory_name.to_owned()
    } else {
        format!("{},{}", parent, directory_name)
    }
}

fn get_directory_size(parent_directory: &String, files: &Vec<(String, usize, String)>) -> usize {
    let mut total_size = 0;
    for (parent, file_size, _) in files.iter() {
        if parent.contains(parent_directory) {
            total_size += file_size;
        }
    }

    total_size
}

fn construct(commands: &Vec<Commands>) -> (Vec<(String, usize, String)>, Vec<(String, String)>) {
    let mut directory_stack: Vec<String> = vec![];
    let mut file_systems = vec![];
    let mut directory_systems = vec![];
    for cmd in commands.iter() {
        match cmd {
            Commands::MOVE_IN(name) => {
                let parent = self::get_parent_name(&directory_stack);
                directory_systems.push((parent, name.to_owned()));
                directory_stack.push(name.to_owned());
            }
            Commands::MOVE_OUT => {
                directory_stack.pop();
            }
            Commands::FILE(file_size, filename) => {
                let parent = self::get_parent_name(&directory_stack);
                file_systems.push((parent, *file_size, filename.to_owned()));
            }
            _ => {}
        }
    }

    (file_systems, directory_systems)
}

fn get_parent_name(stacks: &Vec<String>) -> String {
    let mut result = String::new();
    for name in stacks.iter() {
        if result.len() > 0 {
            result.push_str(",");
        }
        result.push_str(name.as_str());
    }

    result
}

fn parsing_input(lines: &Vec<&str>) -> Vec<self::Commands> {
    let mut result = vec![];
    for line in lines.iter() {
        if line.contains("$ cd ..") {
            result.push(self::Commands::MOVE_OUT);
        } else if line.contains("$ cd ") {
            let name = line.to_owned().replace("$ cd ", "");
            result.push(self::Commands::MOVE_IN(name));
        } else if line.contains("$ ls") {
            result.push(self::Commands::LIST);
        } else if line.contains("dir ") {
            let name = line.to_owned().replace("dir ", "");
            result.push(self::Commands::DIRECTORY(name));
        } else {
            let values = line.split(" ").collect::<Vec<&str>>();
            let file_size = values[0].parse::<usize>().unwrap();
            let filename = values[1].to_owned();
            result.push(self::Commands::FILE(file_size, filename));
        }
    }

    result
}

const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[test]
fn test_part_1() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day07::part_01(&lines);
    assert_eq!(result, 95437);
}

#[test]
fn test_part_2() {
    let lines: Vec<&str> = TEST_INPUT.lines().collect();
    let result = Day07::part_02(&lines);
    assert_eq!(result, 24933642);
}
