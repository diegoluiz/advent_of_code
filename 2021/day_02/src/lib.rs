use helper::helper;

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<String>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_02".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let lines = input.split("\n").map(|x| x.to_string()).collect::<Vec<_>>();
        ChallengeInput { data: lines }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let lines = input.data.clone();
        let mut hor = 0;
        let mut dep = 0;

        for line in lines {
            let x: Vec<&str> = line.split(" ").collect();
            let action = x[0];
            let pos = x[1].parse::<i32>().unwrap();

            if action == "forward" {
                hor += pos;
            } else if action == "down" {
                dep += pos;
            } else if action == "up" {
                dep -= pos;
            }
        }

        return (hor * dep).to_string();
    }

    fn part_02(&self, input: &ChallengeInput) -> String {
        let lines = input.data.clone();
        let mut hor = 0;
        let mut dep = 0;
        let mut aim = 0;

        for line in lines {
            let x: Vec<&str> = line.split(" ").collect();
            let action = x[0];
            let pos = x[1].parse::<i32>().unwrap();

            if action == "forward" {
                hor += pos;
                dep += aim * pos;
            } else if action == "down" {
                aim += pos;
            } else if action == "up" {
                aim -= pos;
            }
        }

        return (hor * dep).to_string();
    }
}

// fn main() {
//     let now = Instant::now();

//     let input_file_path = helper::get_input_file(false);
//     let input = helper::load_input(input_file_path);

//     let lines = input.split("\n").collect::<Vec<&str>>();
//     let lines_arr = &lines[..];

//     let now_p1 = Instant::now();
//     let part_01_output = part_01(lines_arr);
//     println!(
//         "Part 1 result: {}   Took: {} microseconds",
//         part_01_output,
//         now_p1.elapsed().as_micros()
//     );

//     let now_p2 = Instant::now();
//     let part_02_output = part_02(lines_arr);
//     println!(
//         "Part 2 result: {}   Took: {} microseconds",
//         part_02_output,
//         now_p2.elapsed().as_micros()
//     );

//     println!("Done in {} microseconds", now.elapsed().as_micros());
// }
