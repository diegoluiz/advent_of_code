use helper::helper;
use itertools::{self, Itertools};
use std::collections::{HashMap, HashSet};

fn get_neighbours(node: (usize, usize), size: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    if node.0 > 0 {
        neighbours.push((node.0 - 1, node.1));
    }

    if node.0 < size - 1 {
        neighbours.push((node.0 + 1, node.1));
    }

    if node.1 > 0 {
        neighbours.push((node.0, node.1 - 1));
    }

    if node.1 < size - 1 {
        neighbours.push((node.0, node.1 + 1));
    }

    neighbours
}

fn process(nodes: &Vec<Vec<usize>>) -> String {
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut scores = HashMap::<(usize, usize), usize>::new();

    let mut to_visit = HashMap::<(usize, usize), usize>::new();
    let start = (0, 0);
    scores.insert(start, 0);
    to_visit.insert(start, 0);

    while to_visit.len() > 0 {
        let priority_queue = to_visit
            .iter()
            .sorted_by(|a, b| a.1.cmp(b.1))
            .map(|x| (*x.0, *x.1))
            .next()
            .unwrap();
        let node = priority_queue;
        to_visit.remove(&node.0);

        let s = scores.clone();
        let neighbours = get_neighbours(node.0, nodes.len());

        let not_visited_neighbours = neighbours
            .iter()
            .filter(|x| !visited.contains(x))
            .collect::<Vec<_>>();

        let current_score = s.get(&node.0).unwrap();
        for neighbour in not_visited_neighbours.iter() {
            let n = *neighbour.clone();
            let new_score = current_score + nodes[n.0][n.1];
            let new_score = new_score.min(*s.get(&n).unwrap_or(&std::usize::MAX));
            scores.insert(n, new_score);
            to_visit.insert(n, new_score);
        }

        visited.insert(node.0);
    }

    let score = *scores.get(&(nodes.len() - 1, nodes.len() - 1)).unwrap();
    score.to_string()
}

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<Vec<usize>>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_15".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir,is_sample_input);
        let input = helper::load_input(input_file_path);

        let nodes = input
            .split("\n")
            .map(|x| {
                x.split("")
                    .filter(|x| *x != "")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        ChallengeInput { data: nodes }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let nodes = input.data.clone();
        process(&nodes)
    }

    fn part_02(&self, input: &ChallengeInput) -> String {
        let nodes = input.data.clone();
        let mut max_nodes = nodes.clone();

        for i in 1..5 {
            let mut n = nodes.clone();
            for y in 0..n.len() {
                for x in 0..n.len() {
                    let mut v = nodes[y][x] + i;
                    if v >= 10 {
                        v += 1;
                    }
                    v = v % 10;
                    n[y][x] = v;
                }
            }

            for i in 0..n.len() {
                let line = n.get(i).unwrap().clone();
                max_nodes.push(line);
            }
        }

        for y in 0..max_nodes.len() {
            let s = max_nodes.get(y).unwrap().clone();
            for i in 1..5 {
                let mut ns = s.clone();
                for si in 0..ns.len() {
                    let mut v = s[si] + i;
                    if v >= 10 {
                        v += 1;
                    }
                    v = v % 10;
                    ns[si] = v;
                }
                max_nodes[y].append(&mut ns);
            }
        }

        process(&max_nodes)
    }
}
