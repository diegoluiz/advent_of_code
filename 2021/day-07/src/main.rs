use helper::helper;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input_file_path = helper::get_input_file(false);
    let input = helper::load_input(input_file_path);

    let mut crabs = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    crabs.sort();

    let now_p1 = Instant::now();
    let part_01_output = part_01(&crabs);
    println!(
        "Part 1 result: {}   Took: {} microseconds",
        part_01_output,
        now_p1.elapsed().as_micros()
    );

    let now_p2 = Instant::now();
    let part_02_output = part_02(&crabs);
    println!(
        "Part 2 result: {}   Took: {} microseconds",
        part_02_output,
        now_p2.elapsed().as_micros()
    );

    println!("Done in {} microseconds", now.elapsed().as_micros());
}

fn part_01(crabs: &Vec<i32>) -> String {
    let mid = crabs.len() / 2;

    let median = crabs[mid];
    let mut fuel = 0;

    for crab in crabs {
        let diff = *crab - median;
        fuel += diff.abs();
    }

    fuel.to_string()
}

fn part_02(crabs: &Vec<i32>) -> String {
    let mut min_fuel = i32::MAX;
    let start = crabs.first().unwrap();
    let end = crabs.last().unwrap();

    for pos in *start..*end {
        let mut fuel = 0;
        for crab in crabs {
            let diff = (*crab - pos).abs() + 1;
            fuel += (diff * (diff - 1)) / 2;
        }

        min_fuel = min_fuel.min(fuel);
    }

    min_fuel.to_string()
}
