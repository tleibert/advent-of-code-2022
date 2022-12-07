use std::collections::HashSet;

use advent_of_code_2022::{read_file_and_get_input, Problem};

fn main() {
    let (problem, contents) = read_file_and_get_input();

    let answer = match problem {
        Problem::One => problem_1(&contents),
        Problem::Two => problem_2(&contents),
    };

    println!("Answer: {answer}");
}

fn is_unique(s: &str) -> bool {
    let chars: HashSet<_> = s.chars().collect();
    chars.len() == s.len()
}

fn problem_1(contents: &str) -> usize {
    let window_size = 4;

    for end in window_size..contents.len() {
        let start = end - window_size;
        if is_unique(&contents[start..end]) {
            return end;
        }
    }

    contents.len()
}

fn problem_2(contents: &str) -> usize {
    let window_size = 14;

    for end in window_size..contents.len() {
        let start = end - window_size;
        if is_unique(&contents[start..end]) {
            return end;
        }
    }

    contents.len()
}
