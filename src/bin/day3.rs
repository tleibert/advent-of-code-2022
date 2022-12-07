#![feature(iter_array_chunks)]

use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let problem: i32 = env::args().nth(1).unwrap().parse()?;
    let filename: &str = &env::args().nth(2).unwrap();

    let contents = std::fs::read_to_string(filename)?;
    let mut priority_map = HashMap::new();
    for (i, letter) in ('a'..='z').enumerate() {
        priority_map.insert(letter, (i + 1) as i32);
    }
    for (i, letter) in ('A'..='Z').enumerate() {
        priority_map.insert(letter, (i + 27) as i32);
    }

    let result = match problem {
        1 => problem_1(&contents, &priority_map),
        2 => problem_2(&contents, &priority_map),
        _ => panic!("Unknown problem"),
    };

    println!("Answer: {}", result);

    let bruh = "abcdefgh";
    let (h1, h2) = bruh.split_at(bruh.len() / 2);
    println!("Half 1: {h1}, Half 2: {h2}");

    Ok(())
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
