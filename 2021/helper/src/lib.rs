pub mod helper {
    use std::time::Instant;

    use std::env;
    use std::fs::File;
    use std::io::Read;

    pub fn load_input(path: String) -> String {
        // println!("Loading {}", path);
        let mut data = String::new();
        let mut f = File::open(&path).expect(format!("Unable to open file: {}", &path).as_str());
        f.read_to_string(&mut data).expect("Unable to read string");
        return data;
    }

    pub fn get_input_file(base_dir: String, is_sample: bool) -> String {
        let curr_dir = env::current_dir().unwrap();
        let curr_dir = curr_dir.to_str().unwrap();
        if is_sample {
            return format!("{}/{}/data/input_sample", curr_dir.to_string(), base_dir);
        }
        format!("{}/{}/data/input", curr_dir.to_string(), base_dir)
    }

    pub fn binary_to_isize(s: &str) -> isize {
        isize::from_str_radix(&s, 2).unwrap()
    }

    pub trait Process<T> {
        fn challenge_name(&self) -> String;
        fn load_input(&self, base_dir: String, is_sample_input: bool) -> T;
        fn part_01(&self, input: &T) -> String;
        fn part_02(&mut self, input: &T) -> String;
        fn process(&mut self, is_sample_input: bool) {
            let now = Instant::now();

            println!("{}:", self.challenge_name());

            let now_p1 = Instant::now();
            let lines_arr = self.load_input(self.challenge_name(), is_sample_input);
            let part_01_output = self.part_01(&lines_arr);
            println!(
                "  p1: {}   Took: {} microseconds",
                part_01_output,
                now_p1.elapsed().as_micros()
            );

            let now_p2 = Instant::now();
            let part_02_output = self.part_02(&lines_arr);
            println!(
                "  p2: {}   Took: {} microseconds",
                part_02_output,
                now_p2.elapsed().as_micros()
            );

            println!(" took {} microseconds", now.elapsed().as_micros());            

            // Short
            // println!("{} -> p1: [{}] p2: [{}]. Finished in {} microseconds", self.challenge_name(), part_01_output,part_02_output, now.elapsed().as_micros());
        }
    }
}
