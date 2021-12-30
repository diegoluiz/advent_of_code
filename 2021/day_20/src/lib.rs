use helper::helper;
use std::collections::HashSet;

fn get_neighbours(row: i32, col: i32) -> Vec<(i32, i32)> {
    let mut o = Vec::<(i32, i32)>::new();

    o.push((row - 1, col - 1));
    o.push((row - 1, col));
    o.push((row - 1, col + 1));

    o.push((row, col - 1));
    o.push((row, col));
    o.push((row, col + 1));

    o.push((row + 1, col - 1));
    o.push((row + 1, col));
    o.push((row + 1, col + 1));

    o
}

fn process(max_tick: i32, input: (Vec<char>, Vec<Vec<char>>)) -> String {
    let (iae, image) = input;
    let iae = iae.clone();
    let mut lights: HashSet<(i32, i32)> = image
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, c)| {
                    if *c == '#' {
                        Some((y as i32, x as i32))
                    } else {
                        None
                    }
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect::<Vec<(i32, i32)>>()
        })
        .fold(Vec::<(i32, i32)>::new(), |mut p, c| {
            p.extend(&c);
            p
        })
        .into_iter()
        .collect();

    let mut tick = 0;
    while tick < max_tick {
        let inf_bit = if *iae.first().unwrap() == '#' && tick % 2 != 0 {
            '1'
        } else {
            '0'
        };

        let mut temp = HashSet::<(i32, i32)>::new();
        let mut min_y = i32::MAX;
        let mut min_x = i32::MAX;
        let mut max_y = i32::MIN;
        let mut max_x = i32::MIN;
        for light in lights.iter() {
            min_y = min_y.min(light.0);
            min_x = min_x.min(light.1);
            max_y = max_y.max(light.0);
            max_x = max_x.max(light.1);
        }

        for y in min_y - 1..max_y + 2 {
            for x in min_x - 1..max_x + 2 {
                let neighbours = get_neighbours(y, x);
                let code = neighbours
                    .iter()
                    .map(|(y, x)| {
                        if x < &min_x || y < &min_y || x > &max_x || y > &max_y {
                            return inf_bit;
                        }
                        return if lights.contains(&(*y, *x)) { '1' } else { '0' };
                    })
                    .fold("".to_string(), |mut p, c| {
                        p.push(c);
                        p
                    });
                let n = helper::binary_to_isize(code.as_str()) as usize;
                if iae[n] == '#' {
                    temp.insert((y, x));
                }
            }
        }
        lights = temp;
        tick += 1;
    }

    lights.len().to_string()
}

pub struct Challenge {}
pub struct ChallengeInput {
    data: (Vec<char>, Vec<Vec<char>>),
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_20".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let input = input.split("\n\n").collect::<Vec<_>>();

        let iea = input[0]
            .split("")
            .filter(|x| *x != "")
            .map(|f| f.chars().next().unwrap())
            .collect::<Vec<_>>();

        let image = input[1]
            .split("\n")
            .filter(|x| *x != "")
            .map(|x| {
                x.split("")
                    .filter(|x| *x != "")
                    .map(|x| x.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        ChallengeInput { data: (iea, image) }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let input = input.data.clone();
        process(2, input)
    }

    fn part_02(&mut self, input: &ChallengeInput) -> String {
        let input = input.data.clone();
        process(50, input)
    }
}
