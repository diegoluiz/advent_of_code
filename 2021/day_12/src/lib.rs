use helper::helper;
use std::collections::{HashMap, HashSet};

fn get_graph(entries: &Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::<String, Vec<String>>::new();
    for entry in entries {
        let mut iter = entry.iter();
        let left = iter.next().unwrap();
        let right = iter.next().unwrap();

        if !graph.contains_key(right) {
            graph.insert(right.to_string(), Vec::new());
        }
        graph.get_mut(right).unwrap().push(left.to_string());
        graph.get_mut(right).unwrap().sort();

        if !graph.contains_key(left) {
            graph.insert(left.to_string(), Vec::new());
        }
        graph.get_mut(left).unwrap().push(right.to_string());
        graph.get_mut(left).unwrap().sort();
    }
    graph
}

fn bfs_rec_p1(
    node_name: &String,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    path: String,
) -> i32 {
    if node_name == "end" {
        // println!("{}", path);
        return 1;
    }

    let mut visited_clone = visited.clone();
    visited_clone.insert(node_name.to_string());
    let node_neighbours = graph.get(node_name).unwrap();

    let neighbours_to_visit = node_neighbours
        .iter()
        .filter(|x| !visited_clone.contains(*x) || *x.to_uppercase() == **x)
        .collect::<Vec<_>>();

    if neighbours_to_visit.len() == 0 {
        return 0;
    }

    let sum = neighbours_to_visit
        .iter()
        .map(|x| {
            let mut new_path = path.clone();
            new_path.push_str(",");
            new_path.push_str(x);
            let r = bfs_rec_p1(&x, &graph, &mut visited_clone, new_path);
            r
        })
        .sum();

    sum
}

fn bfs_rec_p2(
    node_name: &String,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    small_cabe_visited: bool,
    path: String,
) -> i32 {
    if node_name == "end" {
        // println!("{}", path);
        return 1;
    }

    let mut visited_clone = visited.clone();
    visited_clone.insert(node_name.to_string());
    let node_neighbours = graph.get(node_name).unwrap();

    let neighbours_to_visit = node_neighbours
        .iter()
        .filter(|x| {
            ((!visited_clone.contains(*x) || *x.to_uppercase() == **x) || !small_cabe_visited)
                && (*x != "start")
        })
        .collect::<Vec<_>>();

    if neighbours_to_visit.len() == 0 {
        return 0;
    }

    let sum = neighbours_to_visit
        .iter()
        .map(|x| {
            let small_cabe_visited_2 =
                small_cabe_visited || visited_clone.contains(*x) && *x.to_lowercase() == **x;
            let mut new_path = path.clone();
            new_path.push_str(",");
            new_path.push_str(x);
            let r = bfs_rec_p2(
                &x,
                &graph,
                &mut visited_clone,
                small_cabe_visited_2,
                new_path,
            );
            r
        })
        .sum();

    sum
}

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<Vec<String>>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_12".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let entries = input
            .split("\n")
            .map(|x| {
                x.split("-")
                    .filter(|y| *y != "")
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        ChallengeInput { data: entries }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let entries = input.data.clone();
        let graph = get_graph(&entries);

        let mut visited = HashSet::<String>::new();
        let start = "start".to_string();
        bfs_rec_p1(&start, &graph, &mut visited, String::from("start")).to_string()
    }

    fn part_02(&self, input: &ChallengeInput) -> String {
        let entries = input.data.clone();
        let graph = get_graph(&entries);

        let mut visited = HashSet::<String>::new();
        let start = "start".to_string();
        bfs_rec_p2(&start, &graph, &mut visited, false, String::from("start")).to_string()
    }
}
