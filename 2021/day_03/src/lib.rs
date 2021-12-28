use helper::helper;

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<Vec<char>>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_03".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let mut lines = input.split("\n").collect::<Vec<&str>>();
        lines.sort();
        let lines = lines
            .iter()
            .map(|&x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        ChallengeInput { data: lines }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let lines = input.data.clone();
        let line_count = lines.len();
        let line_size = lines[0].len();

        let mut counters = vec![0; line_size];
        for line in lines {
            let line_chars = line;
            let mut c = 0;
            for x in line_chars {
                if x == '1' {
                    counters[c] += 1;
                }
                c += 1;
            }
        }

        let mut gama_rate = String::new();
        let mut epsilon_rate = String::new();

        for counter in counters {
            if counter > line_count / 2 {
                gama_rate += "1";
                epsilon_rate += "0";
            } else {
                gama_rate += "0";
                epsilon_rate += "1";
            }
        }

        let gama_rate = isize::from_str_radix(&gama_rate, 2).unwrap();
        let epsilon_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();

        return (gama_rate * epsilon_rate).to_string();
    }

    fn part_02(&self, input: &ChallengeInput) -> String {
        let lines = input.data.clone();
        let mut o2_ratings = lines.clone();
        let mut co2_ratings = lines.clone();
        let line_size = lines[0].len();

        for x in 0..line_size {
            let line_count = o2_ratings.len();
            let mut c = 0;

            while o2_ratings[c][x] == '0' {
                c += 1;
            }

            if c > (line_count - c) {
                o2_ratings.drain(c..);
            } else {
                o2_ratings.drain(..c);
            }

            if o2_ratings.len() == 1 {
                break;
            }
        }

        for x in 0..line_size {
            let line_count = co2_ratings.len();
            let mut c = 0;

            while co2_ratings[c][x] == '0' {
                c += 1;
            }

            if c <= (line_count - c) {
                co2_ratings.drain(c..);
            } else {
                co2_ratings.drain(..c);
            }

            if co2_ratings.len() == 1 {
                break;
            }
        }

        let o2_rate = &o2_ratings[0];
        let o2_rate = o2_rate.into_iter().collect::<String>();
        let o2_rate = isize::from_str_radix(&o2_rate, 2).unwrap();

        let co2_rate = &co2_ratings[0];
        let co2_rate = co2_rate.into_iter().collect::<String>();
        let co2_rate = isize::from_str_radix(&co2_rate, 2).unwrap();

        (o2_rate * co2_rate).to_string()
    }
}
