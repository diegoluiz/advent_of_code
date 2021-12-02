use std::time::{Instant};

use std::fs::File;
use std::io::Read;
use std::env;

fn load_input(path: String) -> String {
    println!("Loading {}", path);
    let mut data = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    return data;
}

fn challenge_directory() -> String {
    let current_exe = env::current_exe().unwrap();
    println!("Executable directory: {}", current_exe.as_path().to_str().unwrap());
    let current_dir = current_exe.parent().unwrap().parent().unwrap();
    let dir_name = current_dir.as_os_str().to_str().unwrap();
    return dir_name.to_string();
}

fn main() {
    let now = Instant::now();

    let challenge_dir_name = challenge_directory();
    let input_path = format!("{}/data/input", challenge_dir_name);
    let input = load_input(input_path.to_string());

    let lines = input
        .split("\n")
        .collect::<Vec<&str>>();
    let lines_arr = &lines[..];

    let now_p1 = Instant::now();
    let part_01_output = part_01(lines_arr);
    println!("Part 1 result: {}   Took: {} microseconds", part_01_output, now_p1.elapsed().as_micros());

    let now_p2 = Instant::now();
    let part_02_output = part_02(lines_arr);
    println!("Part 2 result: {}   Took: {} microseconds", part_02_output, now_p2.elapsed().as_micros());

    println!("Done in {} microseconds", now.elapsed().as_micros());
}

fn part_01(lines: &[&str]) -> String {
    let mut hor = 0;
    let mut dep = 0;

    for line in lines{
        let x: Vec<&str> = line.split(" ").collect();
        let action = x[0];
        let pos = x[1].parse::<i32>().unwrap();

        if action == "forward" {
            hor += pos;
        } else if action == "down" {
            dep += pos;
        } else if action == "up"{
            dep -= pos;
        }
    }

    return (hor * dep).to_string();
}

fn part_02(lines: &[&str]) -> String {
    let mut hor = 0;
    let mut dep = 0;
    let mut aim = 0;

    for line in lines{
        let x: Vec<&str> = line.split(" ").collect();
        let action = x[0];
        let pos = x[1].parse::<i32>().unwrap();

        if action == "forward" {
            hor += pos;
            dep += aim * pos;
        } else if action == "down" {
            aim += pos;
        } else if action == "up"{
            aim -= pos;
        }
    }

    return (hor * dep).to_string();
}
