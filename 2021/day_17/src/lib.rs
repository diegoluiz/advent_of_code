use helper::helper;
use regex::Regex;

pub struct Challenge {}
pub struct ChallengeInput {
    data: ((i32, i32), (i32, i32)),
}

fn simulate(x_vel: i32, y_vel: i32, t_x: &(i32, i32), t_y: &(i32, i32)) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut x_vel = x_vel;
    let mut y_vel = y_vel;
    loop {
        x += x_vel;
        y += y_vel;

        if t_x.0 <= x && x <= t_x.1 && t_y.0 <= y && y <= t_y.1 {
            return true;
        }

        if x_vel == 0 && (x < t_x.0 || x > t_x.1) {
            return false;
        }

        if y < t_y.0 || x > t_x.1 {
            return false;
        }

        if x_vel > 0 {
            x_vel -= 1;
        }
        y_vel -= 1;
    }
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_17".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let re = Regex::new(r"^target area: x=(.+)\.\.(.+), y=(.+)\.\.(.+)$").unwrap();
        let c = re.captures(input.as_str()).unwrap();

        let x = (
            *&c[1].parse::<i32>().unwrap(),
            *&c[2].parse::<i32>().unwrap(),
        );
        let y = (
            *&c[3].parse::<i32>().unwrap(),
            *&c[4].parse::<i32>().unwrap(),
        );

        ChallengeInput { data: (x, y) }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let y = input.data.1.clone();
        let min_y = y.0.min(y.1);

        let mut y_pos = 0;
        let mut y_speed = min_y.abs() - 1;
        while y_speed != 0 {
            y_pos += y_speed;
            y_speed -= 1;
        }

        y_pos.to_string()
    }

    fn part_02(&self, input: &ChallengeInput) -> String {
        let y = input.data.1.clone();
        let x = input.data.0.clone();

        let max_x_vel = x.1;
        let min_x_vel = (x.0 as f64).sqrt() as i32;

        let min_y_vel = -100;
        let max_y_vel = 100;

        let mut matches = Vec::<(i32, i32)>::new();
        for x_vel in min_x_vel..max_x_vel + 1 {
            for y_vel in min_y_vel..max_y_vel {
                match simulate(x_vel, y_vel, &x, &y) {
                    true => matches.push((x_vel, y_vel)),
                    _ => (),
                }
            }
        }

        matches.iter().count().to_string()
    }
}
