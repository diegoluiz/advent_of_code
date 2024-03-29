use helper::helper;
use std::collections::HashMap;

fn process(steps: i32, template: &String, rules: &HashMap<String, String>) -> String {
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();

    let map: HashMap<(char, char), ((char, char), (char, char))> = rules
        .iter()
        .map(|x| {
            let a = x.0.chars().nth(0).unwrap();
            let b = x.0.chars().nth(1).unwrap();
            let c = x.1.chars().nth(0).unwrap();

            let input = (a, b);
            let output1 = (a, c);
            let output2 = (c, b);
            (input, (output1, output2))
        })
        .collect();

    for i in 0..template.len() - 1 {
        let a = template[i..i + 1].chars().next().unwrap();
        let b = template[i + 1..i + 2].chars().next().unwrap();
        pairs.entry((a, b)).and_modify(|v| *v += 1).or_insert(1);
    }

    for _ in 0..steps {
        for (pair, count) in pairs.clone() {
            if count <= 0 {
                continue;
            }
            let new_pairs = map.get(&pair).unwrap();
            pairs.entry(pair).and_modify(|v| *v -= count);
            pairs
                .entry(new_pairs.0)
                .and_modify(|v| *v += count)
                .or_insert(count);
            pairs
                .entry(new_pairs.1)
                .and_modify(|v| *v += count)
                .or_insert(count);
        }
    }

    let mut letters = pairs
        .iter()
        .map(|(pair, count)| (pair.0, *count))
        .collect::<Vec<_>>();

    letters.push((template.chars().nth_back(0).unwrap(), 1));

    let mut letters_count: HashMap<char, usize> = HashMap::new();

    for letter in letters {
        letters_count
            .entry(letter.0)
            .and_modify(|v| *v += letter.1)
            .or_insert(letter.1);
    }

    let max = *letters_count.iter().map(|f| f.1).max().unwrap();
    let min = *letters_count.iter().map(|f| f.1).min().unwrap();

    (max - min).to_string()
}

pub struct Challenge {}
pub struct ChallengeInput {
    data: (String, HashMap<String, String>),
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_14".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let lines = input.split("\n\n").collect::<Vec<_>>();

        let template = String::from(lines[0]);
        let rules: HashMap<String, String> = lines[1]
            .split("\n")
            .map(|x| x.split(" -> ").collect::<Vec<&str>>())
            .map(|x| (x[0].to_string(), x[1].to_string()))
            .collect();

        ChallengeInput {
            data: (template, rules),
        }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let template = input.data.0.clone();
        let rules = input.data.1.clone();
        process(10, &template, &rules)
    }

    fn part_02(&mut self, input: &ChallengeInput) -> String {
        let template = input.data.0.clone();
        let rules = input.data.1.clone();
        process(40, &template, &rules)
    }
}
