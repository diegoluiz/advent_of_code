use helper::helper;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input_file_path = helper::get_input_file(false);
    let input = helper::load_input(input_file_path);

    let lines = input.split("\n").collect::<Vec<&str>>();
    let lines_arr = &lines[..];

    let now_p1 = Instant::now();
    let part_01_output = part_01(lines_arr);
    println!(
        "Part 1 result: {}   Took: {} microseconds",
        part_01_output,
        now_p1.elapsed().as_micros()
    );

    let now_p2 = Instant::now();
    let part_02_output = part_02(lines_arr);
    println!(
        "Part 2 result: {}   Took: {} microseconds",
        part_02_output,
        now_p2.elapsed().as_micros()
    );

    println!("Done in {} microseconds", now.elapsed().as_micros());
}

fn part_01(lines: &[&str]) -> String {
    let mut last = 0;
    let mut total_inc = 0;

    for x in lines {
        let n = x.parse::<i32>().unwrap();
        if n > last {
            total_inc += 1;
        }
        last = n;
    }

    return (total_inc - 1).to_string();
}

fn part_02(lines: &[&str]) -> String {
    let mut last = 0;
    let mut total_inc = 0;
    let mut i = 0;

    while i < (lines.len() - 2) {
        let n = lines[i].parse::<i32>().unwrap();
        let n1 = lines[i + 1].parse::<i32>().unwrap();
        let n2 = lines[i + 2].parse::<i32>().unwrap();
        let window = n + n1 + n2;
        if window > last {
            total_inc += 1;
        }
        last = window;
        i += 1;
    }

    return (total_inc - 1).to_string();
}
