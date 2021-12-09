use helper::helper;
use std::{time::Instant, collections::HashSet};

fn main() {
    let now = Instant::now();

    let input_file_path = helper::get_input_file(false);
    let input = helper::load_input(input_file_path);

    let entries = input
        .split("\n")
        .map(|x| {
            x.split("")
                .filter(|y| *y != "")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let now_p1 = Instant::now();
    let part_01_output = part_01(&entries);
    println!(
        "Part 1 result: {}   Took: {} microseconds",
        part_01_output,
        now_p1.elapsed().as_micros()
    );

    let now_p2 = Instant::now();
    let part_02_output = part_02(&entries);
    println!(
        "Part 2 result: {}   Took: {} microseconds",
        part_02_output,
        now_p2.elapsed().as_micros()
    );

    println!("Done in {} microseconds", now.elapsed().as_micros());
}

fn part_01(entries: &Vec<Vec<i32>>) -> String {
    let mut local_mins: Vec<i32> = Vec::new();

    for (i_y, row) in entries.iter().enumerate() {
        for (i_x, number_ref) in row.iter().enumerate() {
            let number = *number_ref;
            let x = i_x as i32;
            let y = i_y as i32;
            let nt = get_neighbour(entries, x, y - 1);
            let nb = get_neighbour(entries, x, y + 1);
            let nl = get_neighbour(entries, x - 1, y);
            let nr = get_neighbour(entries, x + 1, y);

            if number < nt && number < nb && number < nl && number < nr {
                local_mins.push(number + 1)
            }
        }
    }
    local_mins.iter().sum::<i32>().to_string()
}

fn part_02(entries: &Vec<Vec<i32>>) -> String {
    let mut local_mins: Vec<(i32, i32)> = Vec::new();

    for (i_y, row) in entries.iter().enumerate() {
        for (i_x, number_ref) in row.iter().enumerate() {
            let number = *number_ref;
            let x = i_x as i32;
            let y = i_y as i32;
            let nt = get_neighbour(entries, x, y - 1);
            let nb = get_neighbour(entries, x, y + 1);
            let nl = get_neighbour(entries, x - 1, y);
            let nr = get_neighbour(entries, x + 1, y);

            if number < nt && number < nb && number < nl && number < nr {
                local_mins.push((y, x))
            }
        }
    }

    let mut basin_sizes = Vec::<i32>::new();
    for basin_bottom in local_mins{
        let mut nodes = HashSet::<(i32,i32)>::new();
        get_neighbours_count(entries, &mut nodes, basin_bottom.0, basin_bottom.1);
        basin_sizes.push(nodes.len() as i32);
    }

    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[..3].iter().fold(1, |acc, curr| acc * curr).to_string()
}

fn get_neighbours_count(entries: &Vec<Vec<i32>>, nodes: &mut HashSet<(i32,i32)>, y: i32, x: i32) {
    if nodes.contains(&(y,x)) {
        return;
    }
    nodes.insert((y,x));

    let mut neighbours = Vec::<(i32,i32)>::new();
    if get_neighbour(entries, x, y - 1) < 9 { neighbours.push((y-1,x))}
    if get_neighbour(entries, x, y + 1) < 9 { neighbours.push((y+1,x))}
    if get_neighbour(entries, x - 1, y) < 9 { neighbours.push((y,x-1))}
    if get_neighbour(entries, x + 1, y) < 9 { neighbours.push((y,x+1))}

    for neighbour in neighbours.iter() {
        get_neighbours_count(entries, nodes, neighbour.0, neighbour.1);
    }
}

fn get_neighbour(entries: &Vec<Vec<i32>>, i_x: i32, i_y: i32) -> i32 {
    if i_x < 0 || i_y < 0 {
        return 10;
    }
    let x = i_x as usize;
    let y = i_y as usize;

    if y > entries.len() || x > entries[0].len() {
        return 10;
    }

    let row = entries.get(y);
    match row {
        Some(row) => {
            let neighbour = row.get(x);
            match neighbour {
                Some(x) => *x,
                _ => 10,
            }
        }
        _ => 10,
    }
}
