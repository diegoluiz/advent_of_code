use helper::helper;
use std::time::Instant;

fn get_neighbour(row: i32, col: i32, len: i32) -> Vec<(usize, usize)> {
    let mut o = Vec::<(i32, i32)>::new();

    o.push((row - 1, col - 1));
    o.push((row - 1, col));
    o.push((row - 1, col + 1));

    o.push((row, col - 1));
    o.push((row, col + 1));

    o.push((row + 1, col - 1));
    o.push((row + 1, col));
    o.push((row + 1, col + 1));

    o.iter()
        .filter(|x| x.0 >= 0 && x.0 < len && x.1 >= 0 && x.1 < len)
        .map(|x| (x.0 as usize, x.1 as usize))
        .collect()
}

fn part_01(entries_ori: &Vec<Vec<i32>>) -> String {
    let mut entries = entries_ori.clone();
    let len = entries.len();
    let mut fire_count = 0;

    for _ in 0..100 {
        for row in 0..len {
            for col in 0..len {
                entries[row][col] += 1;
            }
        }

        let mut fired = true;
        while fired {
            fired = false;
            for row in 0..len {
                for col in 0..len {
                    if entries[row][col] == 10 {
                        fire_count += 1;
                        fired = true;
                        entries[row][col] = 0;
                        let neighbours = get_neighbour(row as i32, col as i32, len as i32);
                        for i in neighbours {
                            if entries[i.0][i.1] != 10 && entries[i.0][i.1] != 0 {
                                entries[i.0][i.1] += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fire_count.to_string()
}

fn part_02(entries_ori: &Vec<Vec<i32>>) -> String {
    let mut entries = entries_ori.clone();
    let len = entries.len();

    let mut tick = 0;
    loop {
        for row in 0..len {
            for col in 0..len {
                entries[row][col] += 1;
            }
        }

        let mut fired = true;
        let mut fire_count = 0;
        while fired {
            fired = false;
            for row in 0..len {
                for col in 0..len {
                    if entries[row][col] == 10 {
                        fire_count += 1;
                        fired = true;
                        entries[row][col] = 0;
                        let neighbours = get_neighbour(row as i32, col as i32, len as i32);
                        for i in neighbours {
                            if entries[i.0][i.1] != 10 && entries[i.0][i.1] != 0 {
                                entries[i.0][i.1] += 1;
                            }
                        }
                    }
                }
            }
        }

        if fire_count == len * len {
            return (tick + 1).to_string();
        }
        tick += 1;
    }
}

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
