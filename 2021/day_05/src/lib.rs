use helper::helper;
use std::collections::HashMap;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_id(&self) -> String {
        format!("{}|{}", self.x, self.y)
    }
}

#[derive(Clone)]
struct Line {
    points: Vec<Point>,
    is_not_diag: bool,
}

impl Line {
    fn create(input: &str) -> Line {
        let mut points = input.split("->").map(|x| {
            let n = x
                .trim()
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            Point { x: n[0], y: n[1] }
        });

        let start = points.next().unwrap();
        let end = points.next().unwrap();
        let is_not_diag = start.x == end.x || start.y == end.y;

        let mut points: Vec<Point> = Vec::new();
        if start.x == end.x {
            let min = start.y.min(end.y);
            let max = start.y.max(end.y);

            for i in min..max + 1 {
                let p = Point { x: start.x, y: i };
                points.push(p);
            }
        } else if start.y == end.y {
            let min = start.x.min(end.x);
            let max = start.x.max(end.x);

            for i in min..max + 1 {
                let p = Point { x: i, y: start.y };
                points.push(p);
            }
        } else {
            if start.x < end.x {
                let y_inc = if start.y < end.y { 1 } else { -1 };
                let mut y = start.y;

                for i in start.x..end.x + 1 {
                    let p = Point { x: i, y };
                    y += y_inc;
                    points.push(p);
                }
            } else {
                let y_inc = if end.y < start.y { 1 } else { -1 };
                let mut y = end.y;

                for i in end.x..start.x + 1 {
                    let p = Point { x: i, y };
                    y += y_inc;
                    points.push(p);
                }
            }
        }

        Line {
            points,
            is_not_diag,
        }
    }
}

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<Line>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_05".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let lines = input.split("\n").collect::<Vec<&str>>();
        let lines = lines
            .into_iter()
            .map(|b| Line::create(b))
            .collect::<Vec<_>>();

        ChallengeInput { data: lines }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let lines = input.data.clone();
        let mut hash_map: HashMap<String, i32> = HashMap::new();

        for line in lines {
            if !line.is_not_diag {
                continue;
            }
            for point in line.points.iter() {
                let entry = hash_map.entry(point.get_id()).or_insert(0);
                *entry += 1;
            }
        }

        let x = hash_map.values().filter(|&&x| x > 1).count();
        return x.to_string();
    }

    fn part_02(&self, input: &ChallengeInput) -> String {
        let lines = input.data.clone();
        let mut hash_map: HashMap<String, i32> = HashMap::new();

        for line in lines {
            for point in line.points.iter() {
                let entry = hash_map.entry(point.get_id()).or_insert(0);
                *entry += 1;
            }
        }

        let x = hash_map.values().filter(|&&x| x > 1).count();
        return x.to_string();
    }
}
