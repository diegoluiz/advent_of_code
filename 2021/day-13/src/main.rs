use helper::helper;
use std::{collections::HashSet, time::Instant};

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

fn part_01(dots: &Vec<Vec<usize>>, folds: &Vec<(char, usize)>) -> String {
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

fn part_02(dots: &Vec<Vec<usize>>, folds: &Vec<(char, usize)>) -> String {
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
    "".to_string()
}

fn main() {
    let now = Instant::now();

    let input_file_path = helper::get_input_file(false);
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

    let now_p1 = Instant::now();
    let part_01_output = part_01(&dots, &folds);
    println!(
        "Part 1 result: {}   Took: {} microseconds",
        part_01_output,
        now_p1.elapsed().as_micros()
    );

    let now_p2 = Instant::now();
    let part_02_output = part_02(&dots, &folds);
    println!(
        "Part 2 result: {}   Took: {} microseconds",
        part_02_output,
        now_p2.elapsed().as_micros()
    );

    println!("Done in {} microseconds", now.elapsed().as_micros());
}
