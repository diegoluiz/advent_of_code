use helper::helper;

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<i32>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_07".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let mut crabs = input
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        crabs.sort();

        ChallengeInput { data: crabs }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let crabs = input.data.clone();
        let mid = crabs.len() / 2;

        let median = crabs[mid];
        let mut fuel = 0;

        for crab in crabs {
            let diff = crab - median;
            fuel += diff.abs();
        }

        fuel.to_string()
    }

    fn part_02(&self, input: &ChallengeInput) -> String {
        let crabs = input.data.clone();
        let mut min_fuel = i32::MAX;
        let start = crabs.first().unwrap();
        let end = crabs.last().unwrap();

        for pos in *start..*end {
            let mut fuel = 0;
            for crab in crabs.iter() {
                let diff = (crab - pos).abs() + 1;
                fuel += (diff * (diff - 1)) / 2;
            }

            min_fuel = min_fuel.min(fuel);
        }

        min_fuel.to_string()
    }
}
