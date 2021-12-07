use helper::helper;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

struct Board {
    id: i32,
    map_unmarked: HashMap<i32, (usize, usize)>,
    map_marked: HashMap<i32, (usize, usize)>,

    drawn_by_row: [i32; 5],
    drawn_by_col: [i32; 5],
}

impl Board {
    fn create(id: i32, input: &str) -> Board {
        let rows = input
            .trim()
            .split("\n")
            .map(|x| {
                x.trim()
                    .split(" ")
                    .filter(|&y| y != "")
                    .map(|y| y.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut map: HashMap<i32, (usize, usize)> = HashMap::new();

        for row_id in 0..rows.len() {
            for col_id in 0..rows[0].len() {
                let num = rows[row_id][col_id];
                map.insert(num, (row_id, col_id));
            }
        }

        let x = Board {
            id,
            map_unmarked: map,
            map_marked: HashMap::new(),
            drawn_by_row: [0; 5],
            drawn_by_col: [0; 5],
        };

        x
    }

    fn is_winner(&self) -> bool {
        let col_max = self.drawn_by_col.iter().max().unwrap();
        let row_max = self.drawn_by_row.iter().max().unwrap();
        *row_max == 5 || *col_max == 5
    }

    fn draw_number(&mut self, number: &i32) {
        let pos = self.map_unmarked.remove(number).unwrap();
        self.map_marked.insert(*number, pos);

        self.drawn_by_row[pos.0] += 1;
        self.drawn_by_col[pos.1] += 1;
    }
}

fn main() {
    let now = Instant::now();

    let input_file_path = helper::get_input_file(false);
    let input = helper::load_input(input_file_path);

    let lines = input.split("\n\n").collect::<Vec<&str>>();
    let drawn_numbers = lines[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let boards = &lines[1..];
    let mut c = 0;
    let mut boards = boards
        .into_iter()
        .map(|b| {
            c += 1;
            Board::create(c, b)
        })
        .collect::<Vec<_>>();

    let now_p1 = Instant::now();
    let part_01_output = part_01(&drawn_numbers, &mut boards);
    println!(
        "Part 1 result: {}   Took: {} microseconds",
        part_01_output,
        now_p1.elapsed().as_micros()
    );

    let now_p2 = Instant::now();
    let part_02_output = part_02(&drawn_numbers, &mut boards);
    println!(
        "Part 2 result: {}   Took: {} microseconds",
        part_02_output,
        now_p2.elapsed().as_micros()
    );

    println!("Done in {} microseconds", now.elapsed().as_micros());
}

fn part_01(drawn_numbers: &Vec<i32>, boards: &mut Vec<Board>) -> String {
    for number in drawn_numbers {
        for board in boards.as_mut_slice() {
            if board.map_unmarked.contains_key(number) {
                board.draw_number(number);
                let is_winner = board.is_winner();
                if is_winner {
                    let unmarked_sum: i32 = board.map_unmarked.keys().map(|k| k).sum();
                    return (unmarked_sum * number).to_string();
                }
            }
        }
    }

    return 0.to_string();
}

fn part_02(drawn_numbers: &Vec<i32>, boards: &mut Vec<Board>) -> String {
    let mut winners: HashSet<i32> = HashSet::new();
    let boards_count = boards.len();

    for number in drawn_numbers {
        for board in boards.as_mut_slice() {
            if board.map_unmarked.contains_key(number) {
                board.draw_number(number);
                let is_winner = board.is_winner();
                if is_winner {
                    if !winners.contains(&board.id) && winners.len() + 1 == boards_count {
                        let unmarked_sum: i32 = board.map_unmarked.keys().map(|k| k).sum();
                        return (unmarked_sum * number).to_string();
                    }

                    winners.insert(board.id);

                }
            }
        }
    }

    return 0.to_string();
}
