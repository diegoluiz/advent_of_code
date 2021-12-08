use helper::helper;
use std::{collections::HashMap, time::Instant};

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_id(&self) -> String {
        format!("{}|{}", self.x, self.y)
    }
}

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

fn main() {
    let now = Instant::now();

    let input_file_path = helper::get_input_file(false);
    let input = helper::load_input(input_file_path);

    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut lines = lines
        .into_iter()
        .map(|b| Line::create(b))
        .collect::<Vec<_>>();

    let now_p1 = Instant::now();
    let part_01_output = part_01(&mut lines);
    println!(
        "Part 1 result: {}   Took: {} microseconds",
        part_01_output,
        now_p1.elapsed().as_micros()
    );

    let now_p2 = Instant::now();
    let part_02_output = part_02(&mut lines);
    println!(
        "Part 2 result: {}   Took: {} microseconds",
        part_02_output,
        now_p2.elapsed().as_micros()
    );

    println!("Done in {} microseconds", now.elapsed().as_micros());
}

fn part_01(lines: &mut Vec<Line>) -> String {
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

fn part_02(lines: &mut Vec<Line>) -> String {
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
