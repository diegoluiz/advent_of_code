use helper::helper;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
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

pub struct Challenge {}
pub struct ChallengeInput {
    data: (Vec<i32>, Vec<Board>),
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_04".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let lines = input.split("\n\n").collect::<Vec<&str>>();
        let drawn_numbers = lines[0]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let boards = &lines[1..];
        let mut c = 0;
        let boards = boards
            .into_iter()
            .map(|b| {
                c += 1;
                Board::create(c, b)
            })
            .collect::<Vec<_>>();

        ChallengeInput {
            data: (drawn_numbers, boards),
        }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let drawn_numbers = input.data.0.clone();
        let mut boards = input.data.1.clone();

        for number in drawn_numbers {
            for board in boards.as_mut_slice() {
                if board.map_unmarked.contains_key(&number) {
                    board.draw_number(&number);
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

    fn part_02(&mut self, input: &ChallengeInput) -> String {
        let drawn_numbers = input.data.0.clone();
        let mut boards = input.data.1.clone();

        let mut winners: HashSet<i32> = HashSet::new();
        let boards_count = boards.len();

        for number in drawn_numbers {
            for board in boards.as_mut_slice() {
                if board.map_unmarked.contains_key(&number) {
                    board.draw_number(&number);
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
}
