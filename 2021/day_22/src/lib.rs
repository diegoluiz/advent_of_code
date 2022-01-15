use std::{collections::HashSet, ops::RangeInclusive};

use helper::helper;
use regex::Regex;

pub struct Challenge {}

impl Challenge {
    pub fn new() -> Challenge {
        Challenge {}
    }
}

pub struct ChallengeInput {
    data: Vec<Command>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cube {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
    size: u128,
}

#[derive(Debug, Clone)]
struct Command {
    on: bool,
    cube: Cube,
}

impl Command {
    fn new(line: &str) -> Command {
        let re = Regex::new(r"^(.+) x=(.+),y=(.+),z=(.+)$").unwrap();
        let c = re.captures(line).unwrap();
        let state = &c[1];

        let unwrap = |x: &str| {
            let v = x
                .split("..")
                .map(|a| a.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            v[0]..=v[1]
        };

        Command {
            on: state == "on",
            cube: Cube::new(unwrap(&c[2]), unwrap(&c[3]), unwrap(&c[4])),
        }
    }
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_22".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let lines = input.split("\n").collect::<Vec<_>>();
        let commands = lines.iter().map(|x| Command::new(x)).collect::<Vec<_>>();

        ChallengeInput { data: commands }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let mut cubes = HashSet::<Cube>::new();
        for command in input.data.iter().take(20) {
            self.process_cube(&mut cubes, &command.cube, command.on);
        }
        cubes.iter().map(|c| c.size).sum::<u128>().to_string()
    }

    fn part_02(&mut self, input: &ChallengeInput) -> String {
        let mut cubes = HashSet::<Cube>::new();
        for command in input.data.iter() {
            self.process_cube(&mut cubes, &command.cube, command.on);
        }
        cubes.iter().map(|c| c.size).sum::<u128>().to_string()
    }
}

impl Cube {
    fn intersects(&self, a: &Cube) -> Option<(Cube, Cube)> {
        let vert_intersection = |a: &RangeInclusive<i32>, b: &RangeInclusive<i32>| {
            let s = *a.start().max(b.start());
            let e = *a.end().min(b.end());
            s..=e
        };

        let a = a.clone();
        let s = self.clone();
        let x = vert_intersection(&a.x, &s.x);
        let y = vert_intersection(&a.y, &s.y);
        let z = vert_intersection(&a.z, &s.z);
        let intersection_cube = Cube::new(x, y, z);

        if !intersection_cube.is_valid() {
            return None;
        }

        return Some((intersection_cube, a));
    }

    fn explodes(&self, source: &Cube) -> Vec<Cube> {
        let mut top = self.clone();
        top.z = (*source.z.end() + 1)..=*top.z.end();
        top.recalculate_size();

        let mut bottom = self.clone();
        bottom.z = *bottom.z.start()..=(*source.z.start() - 1);
        bottom.recalculate_size();

        let mut front = self.clone();
        front.x = (*source.x.end() + 1)..=*front.x.end();
        front.z = source.z.clone();
        front.recalculate_size();

        let mut back = self.clone();
        back.x = *back.x.start()..=(*source.x.start() - 1);
        back.z = source.z.clone();
        back.recalculate_size();

        let mut left = self.clone();
        left.y = (*source.y.end() + 1)..=*left.y.end();
        left.x = source.x.clone();
        left.z = source.z.clone();
        left.recalculate_size();

        let mut right = self.clone();
        right.y = *right.y.start()..=(*source.y.start() - 1);
        right.x = source.x.clone();
        right.z = source.z.clone();
        right.recalculate_size();

        vec![top, bottom, front, back, left, right]
            .into_iter()
            .filter(|x| x.is_valid())
            .collect()
    }

    fn is_valid(&self) -> bool {
        self.x.start() <= self.x.end()
            && self.y.start() <= self.y.end()
            && self.z.start() <= self.z.end()
    }

    fn recalculate_size(&mut self) {
        self.size = (self.x.end() - self.x.start() + 1) as u128
            * (self.y.end() - self.y.start() + 1) as u128
            * (self.z.end() - self.z.start() + 1) as u128;
    }

    fn new(x: RangeInclusive<i32>, y: RangeInclusive<i32>, z: RangeInclusive<i32>) -> Cube {
        let mut cube = Cube { x, y, z, size: 0 };
        if cube.is_valid() { 
            cube.recalculate_size();
        }
        cube
    }
}

impl Challenge {
    fn process_cube(&self, cubes: &mut HashSet<Cube>, cube: &Cube, on: bool) {
        let intersections = cubes
            .iter()
            .map(|x| cube.intersects(&x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        for (intersection_cube, existing_cube) in intersections {
            cubes.remove(&existing_cube);
            let new_cubes = existing_cube.explodes(&intersection_cube);
            new_cubes.iter().for_each(|x| {
                cubes.insert(x.clone());
            });
        }
        if on {
            cubes.insert(cube.clone());
        }
    }
}
