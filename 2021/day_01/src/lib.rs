use helper::helper;

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<String>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_01".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let lines = input.split("\n").map(|x| x.to_string()).collect::<Vec<_>>();
        ChallengeInput { data: lines }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let lines = input.data.clone();
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

    fn part_02(&mut self, input: &ChallengeInput) -> String {
        let lines = input.data.clone();
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
}
