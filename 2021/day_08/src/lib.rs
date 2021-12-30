use helper::helper;
use std::{collections::HashSet, iter::FromIterator};

#[derive(Clone)]
struct Entry {
    signals: Vec<String>,
    digits: Vec<String>,
}

impl Entry {
    fn create(input: &str) -> Entry {
        let parts = input.split("|").collect::<Vec<_>>();
        let signals = parts[0]
            .trim()
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let digits = parts[1]
            .trim()
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        Entry { signals, digits }
    }
}

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<Entry>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_08".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let entries = input
            .split("\n")
            .map(|x| Entry::create(x))
            .collect::<Vec<_>>();

        ChallengeInput { data: entries }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let entries = input.data.clone();
        let mut count = 0;
        for entry in entries {
            let digits = entry.digits.iter().collect::<Vec<_>>();
            for digit in digits {
                let len = (*digit).len();
                match len {
                    2 | 3 | 4 | 7 => count += 1,
                    _ => {}
                }
            }
        }

        count.to_string()
    }

    fn part_02(&mut self, input: &ChallengeInput) -> String {
        let entries = input.data.clone();
        let mut sum = 0;
        for entry in entries {
            let signals = entry
                .signals
                .iter()
                .map(|x| {
                    let chars = x
                        .split("")
                        .filter(|x| *x != "")
                        .map(|x| x.chars().next().unwrap())
                        .collect::<Vec<_>>();

                    let hs: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
                    hs
                })
                .collect::<Vec<_>>();

            let digits = entry
                .digits
                .iter()
                .map(|x| {
                    let chars = x
                        .split("")
                        .filter(|x| *x != "")
                        .map(|x| x.chars().next().unwrap())
                        .collect::<Vec<_>>();

                    let hs: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
                    hs
                })
                .collect::<Vec<_>>();

            let mut leds = [' '; 7];

            let n1 = signals.iter().filter(|x| x.len() == 2).next().unwrap();
            let n4 = signals.iter().filter(|x| x.len() == 4).next().unwrap();
            let n8 = signals.iter().filter(|x| x.len() == 7).next().unwrap();
            let n7 = signals.iter().filter(|x| x.len() == 3).next().unwrap();

            leds[0] = *(n7 - n1).iter().next().unwrap();

            let n6 = signals
                .iter()
                .filter(|x| x.len() == 6)
                .filter(|x| {
                    let y = *x - n1;
                    x.len() - 1 == y.len()
                })
                .next()
                .unwrap();

            leds[2] = *(n8 - n6).iter().next().unwrap();

            let n5 = signals
                .iter()
                .filter(|x| x.len() == 5)
                .filter(|x| (*x - n6).len() == 0)
                .next()
                .unwrap();

            leds[4] = *(n6 - n5).iter().next().unwrap();

            let n2 = signals
                .iter()
                .filter(|x| x.len() == 5)
                .filter(|x| x.contains(&leds[2]) && x.contains(&leds[4]))
                .next()
                .unwrap();

            let n3 = signals
                .iter()
                .filter(|x| x.len() == 5)
                .filter(|x| *x != n2 && *x != n5)
                .next()
                .unwrap();

            let n0 = signals
                .iter()
                .filter(|x| x.len() == 6)
                .filter(|x| x.contains(&leds[2]) && x.contains(&leds[4]))
                .next()
                .unwrap();

            let n9 = signals
                .iter()
                .filter(|x| x.len() == 6)
                .filter(|x| *x != n0 && *x != n6)
                .next()
                .unwrap();

            let mut displayed = String::new();
            for digit in digits {
                if digit == *n0 {
                    displayed.push('0');
                }
                if digit == *n1 {
                    displayed.push('1');
                }
                if digit == *n2 {
                    displayed.push('2');
                }
                if digit == *n3 {
                    displayed.push('3');
                }
                if digit == *n4 {
                    displayed.push('4');
                }
                if digit == *n5 {
                    displayed.push('5');
                }
                if digit == *n6 {
                    displayed.push('6');
                }
                if digit == *n7 {
                    displayed.push('7');
                }
                if digit == *n8 {
                    displayed.push('8');
                }
                if digit == *n9 {
                    displayed.push('9');
                }
            }

            sum += &displayed.parse::<i32>().unwrap();
        }

        sum.to_string()
    }
}
