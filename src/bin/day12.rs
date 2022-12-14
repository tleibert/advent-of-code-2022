use std::collections::HashMap;

use advent_of_code_2022::{read_file_and_get_input, Problem};

use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};

fn main() {
    let (problem, contents) = read_file_and_get_input();

    let answer = match problem {
        Problem::One => problem_1(&contents),
        Problem::Two => problem_2(&contents),
    };

    println!("Answer: {answer}");
}

/// returns a graph, and also the index of the goal
fn create_graph(
    contents: &str,
) -> (
    DiGraph<(usize, usize), i32>,
    HashMap<(usize, usize), NodeIndex>,
    HashMap<(usize, usize), char>,
    (usize, usize),
    (usize, usize),
) {
    let num_lines = contents.lines().count();
    let num_cols = contents.lines().next().unwrap().len();

    let mut graph = DiGraph::with_capacity(num_lines * num_cols, num_lines * num_cols * 4);
    let mut temp_map = HashMap::with_capacity(num_lines * num_cols);
    let mut char_map = HashMap::with_capacity(num_lines * num_cols);

    let mut start = (0, 0);
    let mut goal = (0, 0);
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let new_node = graph.add_node((x, y));
            temp_map.insert((x, y), new_node);
            char_map.insert((x, y), c);

            if c == 'E' {
                println!("Found E at {x}, {y}");
                goal = (x, y);
                char_map.insert((x, y), 'z');
            }

            if c == 'S' {
                println!("Found S at {x}, {y}");
                start = (x, y);
                char_map.insert((x, y), 'a');
            }
        }
    }

    let mut to_examine = Vec::with_capacity(4);

    for y in 0..num_lines {
        for x in 0..num_cols {
            let our_idx = &(x, y);

            let source = temp_map[our_idx];
            let c = char_map[our_idx];

            if x > 0 {
                to_examine.push((x - 1, y));
            }
            if x < num_cols - 1 {
                to_examine.push((x + 1, y));
            }
            if y > 0 {
                to_examine.push((x, y - 1));
            }
            if y < num_lines - 1 {
                to_examine.push((x, y + 1));
            }

            for other_idx in &to_examine {
                let other_c = char_map[other_idx];
                let source_num = c as i32;
                let other_num = other_c as i32;

                // can go to other square if it is at most one higher than us
                if other_num - source_num <= 1 {
                    let dest_node = temp_map[other_idx];
                    graph.add_edge(source, dest_node, 1);
                }
            }

            to_examine.clear();
        }
    }

    (graph, temp_map, char_map, start, goal)
}

fn problem_1(contents: &str) -> i32 {
    let (graph, idx_map, _, start, goal) = create_graph(contents);

    let start = idx_map[&start];
    let end = idx_map[&goal];
    let node_map = dijkstra(&graph, start, Some(end), |_| 1);

    node_map[&end]
}

fn problem_2(contents: &str) -> i32 {
    let (graph, idx_map, char_map, _, goal) = create_graph(contents);

    let end = idx_map[&goal];
    char_map
        .iter()
        .filter_map(|(k, &v)| match v {
            'a' => Some(k),
            _ => None,
        })
        .filter_map(|idx| {
            let start = idx_map[idx];
            let node_map = dijkstra(&graph, start, Some(end), |_| 1);
            node_map.get(&end).cloned()
        })
        .min()
        .unwrap()
}
