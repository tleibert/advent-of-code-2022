#![feature(iter_array_chunks)]

use std::collections::{HashMap, HashSet};

use advent_of_code_2022::{read_file_and_get_input, Problem};

fn main() {
    let (problem, contents) = read_file_and_get_input();
    let mut priority_map = HashMap::new();
    for (i, letter) in ('a'..='z').enumerate() {
        priority_map.insert(letter, (i + 1) as i32);
    }
    for (i, letter) in ('A'..='Z').enumerate() {
        priority_map.insert(letter, (i + 27) as i32);
    }

    let result = match problem {
        Problem::One => problem_1(&contents, &priority_map),
        Problem::Two => problem_2(&contents, &priority_map),
    };

    println!("Answer: {result}");
}

fn problem_1(contents: &str, priority_map: &HashMap<char, i32>) -> i32 {
    let mut total = 0;

    for line in contents.lines() {
        // split line in half
        let (h1, h2) = line.split_at(line.len() / 2);
        let h1: HashSet<_> = h1.chars().collect();
        let h2: HashSet<_> = h2.chars().collect();

        for item in h1.intersection(&h2) {
            total += priority_map[item];
        }
    }

    total
}

fn problem_2(contents: &str, priority_map: &HashMap<char, i32>) -> i32 {
    let mut total = 0;

    for lines in contents.lines().array_chunks::<3>() {
        let contents: Vec<HashSet<char>> =
            lines.iter().map(|line| line.chars().collect()).collect();

        let common_items = contents
            .into_iter()
            .reduce(|accum, item| accum.intersection(&item).copied().collect())
            .unwrap();

        for item in common_items {
            total += priority_map[&item];
        }
    }

    total
}
