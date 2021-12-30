use std::collections::HashMap;

use helper::helper;

pub struct Challenge {
    cache: HashMap<((usize, usize), (usize, usize)), (u128, u128)>,
    quantum_dice: QuantumDice
}
pub struct ChallengeInput {
    data: (usize, usize),
}

impl Challenge {
    pub fn new() -> Challenge {
        Challenge {
            cache: HashMap::new(),
            quantum_dice: QuantumDice::new()
        }
    }

    fn part_02_process(&mut self, p1: (usize, usize), p2: (usize, usize), d: i32) -> (u128, u128) {
        if self.cache.contains_key(&(p1, p2)) {
            return self.cache.get(&(p1, p2)).unwrap().clone();
        }

        let (p1_pos, p1_score) = p1;
        let (p2_pos, p2_score) = p2;

        let mut p1_wins: u128 = 0;
        let mut p2_wins: u128 = 0;
        let dice = self.quantum_dice.clone();
        for (d1,d2,d3) in dice.default_outcomes.iter() {
            let p1_pos_new = calc_new_position(p1_pos, (d1+d2+d3) as usize);
            let p1_score_new = p1_score + p1_pos_new;
            if p1_score_new >= 21 {
                p1_wins += 1;
            } else {
                let (b, a) =
                    self.part_02_process((p2_pos, p2_score), (p1_pos_new, p1_score_new), d + 1);
                p1_wins += a;
                p2_wins += b;
            }
        }
        let ret = (p1_wins, p2_wins);
        self.cache.insert((p1, p2), ret);
        ret
    }
}

struct Dice {
    value: usize,
    rolls: usize,
}

impl Dice {
    fn new() -> Dice {
        Dice { value: 1, rolls: 0 }
    }
    fn next(&mut self) -> usize {
        let n = self.value;
        self.value += 1;
        self.rolls += 1;
        if self.value > 100 {
            self.value = 1;
        }
        n
    }
    fn next_n_rows(&mut self, n: usize) -> usize {
        let mut total = 0;
        for _ in 0..n {
            total += self.next()
        }
        total
    }
}

#[derive(Clone)]
struct QuantumDice {
    default_outcomes: Vec<(u128, u128, u128)>,
}

impl QuantumDice {
    fn new() -> QuantumDice {
        let mut default_outcomes = Vec::<(u128, u128, u128)>::new();
        for r1 in 1..4 {
            for r2 in 1..4 {
                for r3 in 1..4 {
                    default_outcomes.push((r1 as u128, r2 as u128, r3 as u128));
                }
            }
        }

        QuantumDice {
            default_outcomes,
        }
    }
}

fn calc_new_position(pos: usize, n: usize) -> usize {
    let mut x = pos + n;
    while x > 10 {
        x = x - 10;
    }

    x
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_21".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let input = input.split("\n").collect::<Vec<_>>();

        let player_1_pos = input[0].get(28..).unwrap().parse::<usize>().unwrap();
        let player_2_pos = input[1].get(28..).unwrap().parse::<usize>().unwrap();
        ChallengeInput {
            data: (player_1_pos, player_2_pos),
        }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let (mut p1_pos, mut p2_pos) = input.data.clone();

        let mut p1_score = 0;
        let mut p2_score = 0;

        let limit = 1000;
        let mut dice = Dice::new();
        let n = loop {
            let n = dice.next_n_rows(3);
            p1_pos = calc_new_position(p1_pos, n);
            p1_score += p1_pos;

            if p1_score >= limit {
                break p2_score * dice.rolls;
            }

            let n = dice.next_n_rows(3);
            p2_pos = calc_new_position(p2_pos, n);
            p2_score += p2_pos;

            if p2_score > limit {
                break p1_score * dice.rolls;
            }
        };

        n.to_string()
    }

    fn part_02(&mut self, input: &ChallengeInput) -> String {
        let (p1_pos, p2_pos) = input.data.clone();
        let (p1_score, p2_score) = self.part_02_process((p1_pos, 0), (p2_pos, 0), 0);
        p1_score.max(p2_score).to_string()
    }
}
