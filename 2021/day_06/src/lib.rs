use helper::helper;

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<usize>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_06".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir,is_sample_input);
        let input = helper::load_input(input_file_path);
    
        let fishes = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    
        ChallengeInput { data: fishes }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let fishes = input.data.clone();
        let mut days = 80;
    
        let mut fish_times = [0; 9];
    
        for fish in fishes {
            fish_times[fish] += 1;
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

    fn part_02(&self, input: &ChallengeInput) -> String {
        let fishes = input.data.clone();
        let mut days = 256;
    
        let mut fish_times = [0; 9];
    
        for fish in fishes {
            fish_times[fish] += 1;
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

}
