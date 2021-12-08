use helper::helper;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input_file_path = helper::get_input_file(false);
    let input = helper::load_input(input_file_path);

    let fishes = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let now_p1 = Instant::now();
    let part_01_output = part_01(&fishes);
    println!(
        "Part 1 result: {}   Took: {} microseconds",
        part_01_output,
        now_p1.elapsed().as_micros()
    );

    let now_p2 = Instant::now();
    let part_02_output = part_02(&fishes);
    println!(
        "Part 2 result: {}   Took: {} microseconds",
        part_02_output,
        now_p2.elapsed().as_micros()
    );

    println!("Done in {} microseconds", now.elapsed().as_micros());
}

fn part_01(fishes: &Vec<usize>) -> String {
    let mut days = 80;

    let mut fish_times = [0; 9];

    for fish in fishes {
        fish_times[*fish] += 1;
    }

    while days > 0 {
        let new_fishes = fish_times[0];

        for i in 1..fish_times.len() {
            fish_times[i-1] = fish_times[i];
        }

        fish_times[6] += new_fishes;
        fish_times[8] = new_fishes;

        days -= 1;
    }

    fish_times.iter().sum::<usize>().to_string()
}

fn part_02(fishes: &Vec<usize>) -> String {
    let mut days = 256;

    let mut fish_times = [0; 9];

    for fish in fishes {
        fish_times[*fish] += 1;
    }

    while days > 0 {
        let new_fishes = fish_times[0];

        for i in 1..fish_times.len() {
            fish_times[i-1] = fish_times[i];
        }

        fish_times[6] += new_fishes;
        fish_times[8] = new_fishes;

        days -= 1;
    }

    fish_times.iter().sum::<usize>().to_string()
}