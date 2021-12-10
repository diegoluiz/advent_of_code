use helper::helper;
use std::{
    collections::HashMap,
    time::Instant,
};
fn part_01(lines: &Vec<Vec<char>>) -> String {
    let points = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let map_open = HashMap::from([('[', ']'), ('{', '}'), ('(', ')'), ('<', '>')]);
    let map_close = HashMap::from([(']', '['), ('}', '{'), (')', '('), ('>', '<')]);

    let mut sum_points = 0;

    for line in lines {
        let mut stack = Vec::<char>::new();
        for c in line {
            if map_open.contains_key(c) {
                stack.push(*c);
                continue;
            }

            let expected_open = map_close.get(c).unwrap();
            let top_stack = stack.last().unwrap();

            if expected_open != top_stack {
                sum_points += points.get(c).unwrap();
                break;
            }

            stack.remove(stack.len() - 1);
        }
    }
    sum_points.to_string()
}

fn part_02(lines: &Vec<Vec<char>>) -> String {
    let points = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let map_open = HashMap::from([('[', ']'), ('{', '}'), ('(', ')'), ('<', '>')]);
    let map_close = HashMap::from([(']', '['), ('}', '{'), (')', '('), ('>', '<')]);

    let mut scores = Vec::<usize>::new();

    for line in lines {
        let mut stack = Vec::<char>::new();
        let mut corrupted = false;
        for c in line {
            if map_open.contains_key(c) {
                stack.push(*c);
                continue;
            }

            let expected_open = map_close.get(c).unwrap();
            let top_stack = stack.last().unwrap();

            if expected_open != top_stack {
                corrupted = true;
                break;
            }

            stack.remove(stack.len() - 1);
        }

        let mut sum_points = 0;
        if !corrupted {
            stack.reverse();
            for c in stack {
                sum_points *= 5;
                sum_points += points.get(&c).unwrap();
            }
            scores.push(sum_points);
        }
    }
    scores.sort();
    scores[scores.len() / 2].to_string()
}

fn main() {
    let now = Instant::now();

    let input_file_path = helper::get_input_file(false);
    let input = helper::load_input(input_file_path);

    let lines = input
        .split("\n")
        .map(|x| {
            x.split("")
                .filter(|y| *y != "")
                .map(|y| y.chars().into_iter().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let now_p1 = Instant::now();
    let part_01_output = part_01(&lines);
    println!(
        "Part 1 result: {}   Took: {} microseconds",
        part_01_output,
        now_p1.elapsed().as_micros()
    );

    let now_p2 = Instant::now();
    let part_02_output = part_02(&lines);
    println!(
        "Part 2 result: {}   Took: {} microseconds",
        part_02_output,
        now_p2.elapsed().as_micros()
    );

    println!("Done in {} microseconds", now.elapsed().as_micros());
}
