use std::ops::RangeInclusive;

use advent_of_code_2022::{read_file_and_get_input, Problem};

fn main() {
    let (problem, contents) = read_file_and_get_input();

    let answer = match problem {
        Problem::One => problem_1(&contents),
        Problem::Two => problem_2(&contents),
    };

    println!("Answer: {answer}");
}

fn str_to_range(string: &str) -> (i32, i32) {
    let (start, stop) = string.split_once('-').unwrap();
    let start: i32 = start.parse().unwrap();
    let stop: i32 = stop.parse().unwrap();
    (start, stop)
}

fn range_contains_entirely(ours: (i32, i32), other: (i32, i32)) -> bool {
    ours.0 <= other.0 && ours.1 >= other.1
}

fn range_contains(ours: (i32, i32), other: (i32, i32)) -> bool {
    (ours.0 <= other.0 && ours.1 >= other.0) || (ours.0 <= other.1 && ours.1 >= other.1)
}

fn problem_1(contents: &str) -> i32 {
    contents
        .lines()
        .filter(|line| {
            let (h1, h2) = line.split_once(',').unwrap();
            let r1 = str_to_range(h1);
            let r2 = str_to_range(h2);
            range_contains_entirely(r1, r2) || range_contains_entirely(r2, r1)
        })
        .count() as i32
}

fn problem_2(contents: &str) -> i32 {
    contents
        .lines()
        .filter(|line| {
            let (h1, h2) = line.split_once(',').unwrap();
            let r1 = str_to_range(h1);
            let r2 = str_to_range(h2);
            range_contains(r1, r2) || range_contains(r2, r1)
        })
        .count() as i32
}
