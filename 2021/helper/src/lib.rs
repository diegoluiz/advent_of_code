pub mod helper {
    use std::fs::File;
    use std::io::Read;
    use std::env;
    
    pub fn load_input(path: String) -> String {
        println!("Loading {}", path);
        let mut data = String::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut data).expect("Unable to read string");
        return data;
    }

    pub fn get_input_file(is_sample: bool) -> String {
        let curr_dir = env::current_dir().unwrap();
        let curr_dir = curr_dir.to_str().unwrap();
        println!("Current directory: {}", curr_dir);

        if is_sample {
            return format!("{}/data/input_sample", curr_dir.to_string());
        }

        format!("{}/data/input", curr_dir.to_string())
    }

    pub fn binary_to_isize(s: &str) -> isize {
        isize::from_str_radix(&s, 2).unwrap()
    }
}
