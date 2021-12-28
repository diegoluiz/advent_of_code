use helper::helper;
use std::collections::HashSet;

const DOT: char = '#';
const EMPTY: char = ' ';

fn print_grid(grid: HashSet<(usize, usize)>) {
    println!("------------------");

    let mut max_x = 0;
    let mut max_y = 0;
    grid.iter().for_each(|(x, y)| {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    });

    for grid_y in 0..max_y + 1 {
        let mut line = String::new();
        for grid_x in 0..max_x + 1 {
            let c = if grid.contains(&(grid_x, grid_y)) {
                DOT
            } else {
                EMPTY
            };
            line.push(c);
        }
        println!("{}", line);
    }

    println!("------------------");
}

pub struct Challenge {}
pub struct ChallengeInput {
    data: (Vec<Vec<usize>>, Vec<(char, usize)>),
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_13".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let lines = input.split("\n\n").collect::<Vec<_>>();

        let dots = lines[0]
            .split("\n")
            .map(|x| {
                x.split(",")
                    .filter(|y| *y != "")
                    .map(|y| y.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let folds = lines[1]
            .split("\n")
            .map(|x| {
                (
                    x.chars().nth(11).unwrap(),
                    x[13..].parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        ChallengeInput {
            data: (dots, folds),
        }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let dots = input.data.0.clone();
        let folds = input.data.1.clone();

        let mut set: HashSet<(usize, usize)> = dots.iter().map(|x| (x[0], x[1])).collect();

        let (axis, mid) = folds.iter().next().unwrap();
        for (x, y) in set.clone().iter() {
            if *axis == 'y' && y > &*mid {
                let t = mid - (y - mid);
                set.insert((*x, t));
                set.remove(&(*x, *y));
            }

            if *axis == 'x' && x > &*mid {
                let t = mid - (x - mid);
                set.insert((t, *y));
                set.remove(&(*x, *y));
            }
        }

        set.len().to_string()
    }

    fn part_02(&self, input: &ChallengeInput) -> String {
        let dots = input.data.0.clone();
        let folds = input.data.1.clone();

        let mut set: HashSet<(usize, usize)> = dots.iter().map(|x| (x[0], x[1])).collect();

        for (axis, mid) in folds.iter() {
            for (x, y) in set.clone().iter() {
                if *axis == 'y' && y > &*mid {
                    let t = mid - (y - mid);
                    set.insert((*x, t));
                    set.remove(&(*x, *y));
                }

                if *axis == 'x' && x > &*mid {
                    let t = mid - (x - mid);
                    set.insert((t, *y));
                    set.remove(&(*x, *y));
                }
            }
        }

        print_grid(set);
        "see above".to_string()
    }
}
