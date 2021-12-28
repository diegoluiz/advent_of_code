use helper::helper;
use itertools::{self, Itertools};
use regex::Regex;
use std::cmp::Ordering;

enum Position {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
struct Scanner {
    id: String,
    beacons: Vec<Beacon>,
    dists: Vec<(f64, (Beacon, Beacon, Beacon))>,
}

impl Scanner {
    fn build_distance(&mut self) {
        self.dists.clear();
        for beacon in self.beacons.iter() {
            let dists: Vec<_> = self
                .beacons
                .iter()
                .filter(|x| *x != beacon)
                .map(|peer| (calculate_distance(&beacon, peer), peer))
                .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .collect();

            let a = dists[0];
            let b = dists[1];
            let c = calculate_distance(a.1, b.1);

            let key = (a.0 + b.0) * c;
            self.dists.push((key, (*beacon, *a.1, *b.1)));
        }
        // self.dists = self
        //     .dists
        //     .clone()
        //     .into_iter()
        //     .sorted_by(|a, b| a.partial_cmp(&b).unwrap())
        //     .collect();
    }

    fn rotate(&mut self, pos: Position) {
        for beacon in self.beacons.iter_mut() {
            beacon.rotate(&pos);
        }
    }

    fn flip(&mut self, pos: Position) {
        for beacon in self.beacons.iter_mut() {
            beacon.flip(&pos);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl Beacon {
    fn new(x: i32, y: i32, z: i32) -> Beacon {
        Beacon { x, y, z }
    }

    fn rotate(&mut self, pos: &Position) {
        match pos {
            Position::X => {
                let n = self.y;
                self.y = self.z;
                self.z = n;
            }
            Position::Y => {
                let n = self.x;
                self.x = self.z;
                self.z = n;
            }
            Position::Z => {
                let n = self.x;
                self.x = self.y;
                self.y = n;
            }
        }
    }

    fn flip(&mut self, pos: &Position) {
        match pos {
            Position::X => {
                self.x = -self.x;
            }
            Position::Y => {
                self.y = -self.y;
            }
            Position::Z => {
                self.z = -self.z;
            }
        }
    }
}

impl PartialEq for Beacon {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialOrd for Beacon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Beacon {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = self.x.cmp(&other.x);
        let y = self.y.cmp(&other.y);
        let z = self.z.cmp(&other.z);

        if x != Ordering::Equal {
            return x;
        }
        if y != Ordering::Equal {
            return y;
        }
        if z != Ordering::Equal {
            return z;
        }

        return Ordering::Equal;
    }
}

fn calculate_distance(a: &Beacon, b: &Beacon) -> f64 {
    let x = (a.x - b.x).pow(2);
    let y = (a.y - b.y).pow(2);
    let z = (a.z - b.z).pow(2);

    ((x + y + z) as f64).sqrt()
}

fn get_matches(a: &Scanner, b: &Scanner) -> Vec<f64> {
    let a_dists = a.dists.iter().map(|x| x.0).collect::<Vec<_>>();
    let b_dists = b.dists.iter().map(|x| x.0).collect::<Vec<_>>();
    a_dists
        .into_iter()
        .filter(|x| b_dists.contains(x))
        .collect()
}

fn find_next_scanner(scanners: &Vec<Scanner>, scanner_0: &Scanner) -> Scanner {
    let x = scanners
        .iter()
        .filter(|x| {
            let matches = get_matches(scanner_0, x);
            return matches.len() >= 12;
        })
        .next()
        .unwrap();

    return x.clone();
}

fn fix_orientation(scanner_0: &mut Scanner, next: &mut Scanner) -> (i32, i32, i32) {
    let matches = get_matches(&scanner_0, &next);

    let get_match = |scanner: &Scanner| -> (Beacon, Beacon, Beacon) {
        scanner
            .dists
            .iter()
            .filter(|x| x.0 == matches[0])
            .next()
            .unwrap()
            .1
    };

    let a = get_match(&scanner_0);
    let mut b = get_match(&next);

    let a_p1_dist = a.0.x - a.1.x;
    let mut b_p1_dist = b.0.x - b.1.x;

    let mut rotated = false;
    while a_p1_dist != b_p1_dist {
        if a_p1_dist == -b_p1_dist {
            next.flip(Position::X);
        } else {
            if !rotated {
                next.rotate(Position::Z);
                rotated = true;
            } else {
                next.rotate(Position::Y);
            }
        }

        next.build_distance();
        b = get_match(&next);
        b_p1_dist = b.0.x - b.1.x;
    }

    let a_p1_dist = a.0.y - a.1.y;
    let mut b_p1_dist = b.0.y - b.1.y;

    while a_p1_dist != b_p1_dist {
        if a_p1_dist == -b_p1_dist {
            next.flip(Position::Y);
        } else {
            next.rotate(Position::X);
        }
        next.build_distance();
        b = get_match(&next);
        b_p1_dist = b.0.y - b.1.y;
    }

    let a_p1_dist = a.0.z - a.1.z;
    let mut b_p1_dist = b.0.z - b.1.z;

    while a_p1_dist != b_p1_dist {
        if a_p1_dist == -b_p1_dist {
            next.flip(Position::Z);
            next.build_distance();
            b = get_match(&next);
            b_p1_dist = b.0.z - b.1.z;
        }
    }

    let diff_x = a.0.x - b.0.x;
    let diff_y = a.0.y - b.0.y;
    let diff_z = a.0.z - b.0.z;

    (diff_x, diff_y, diff_z)
}

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<Scanner>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_19".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let scanners = input
            .split("\n\n")
            .map(|scanner| {
                let l = scanner.split("\n").filter(|x| *x != "").collect::<Vec<_>>();

                let re = Regex::new(r"^--- scanner (.+) ---$").unwrap();
                let c = re.captures(l[0]).unwrap();
                let id = &c[1].to_string();

                let beacons = l[1..]
                    .iter()
                    .map(|beacon| {
                        let pos = beacon
                            .split(",")
                            .filter(|x| *x != "")
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect::<Vec<_>>();
                        Beacon::new(pos[0], pos[1], pos[2])
                    })
                    .collect::<Vec<_>>();

                Scanner {
                    id: id.clone(),
                    beacons,
                    dists: Vec::new(),
                }
            })
            .collect::<Vec<_>>();

        ChallengeInput { data: scanners }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let mut scanners = input.data.clone();

        let mut uniq_triangle_dist = Vec::<f64>::new();
        for scanner in scanners.iter_mut() {
            scanner.build_distance();
            scanner
                .dists
                .iter()
                .map(|x| x.0)
                .for_each(|x| uniq_triangle_dist.push(x));

            uniq_triangle_dist = uniq_triangle_dist
                .into_iter()
                .sorted_by(|a, b| a.partial_cmp(&b).unwrap())
                .collect();
        }

        let c = uniq_triangle_dist
            .iter()
            .unique_by(|x| ((*x) * 1000000000.0) as u64)
            .count();

        c.to_string()
    }

    fn part_02(&self, input: &ChallengeInput) -> String {
        let mut scanners = input.data.clone();

        scanners.iter_mut().for_each(|f| f.build_distance());
        let mut scanner_0 = scanners[0].clone();
        scanners.drain(0..1);

        let mut locations = vec![(0, 0, 0)];

        while scanners.len() > 0 {
            scanner_0.build_distance();
            let mut next = find_next_scanner(&scanners, &scanner_0);

            let next_location = fix_orientation(&mut scanner_0, &mut next);
            locations.push(next_location);

            next.beacons
                .iter()
                .map(|new| {
                    let mut new = new.clone();
                    new.x += next_location.0;
                    new.y += next_location.1;
                    new.z += next_location.2;
                    new
                })
                .for_each(|new| scanner_0.beacons.push(new));

            scanner_0.beacons = scanner_0.beacons.into_iter().unique().collect();

            scanners = scanners
                .iter()
                .filter(|x| *x.id != next.id)
                .map(|x| x.clone())
                .collect();
        }

        let mut max_dist = 0;

        for i in 0..locations.len() {
            for j in i..locations.len() {
                let dist = (locations[i].0 - locations[j].0).abs()
                    + (locations[i].1 - locations[j].1).abs()
                    + (locations[i].2 - locations[j].2).abs();
                max_dist = max_dist.max(dist);
            }
        }

        max_dist.to_string()
    }
}
