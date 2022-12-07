use std::collections::BinaryHeap;

use advent_of_code_2022::{read_file_and_get_input, Problem};

fn main() {
    // get the problem to run and the input file
    let (problem, contents) = read_file_and_get_input();
    let result = match problem {
        Problem::One => problem_1(&contents),
        Problem::Two => problem_2(&contents),
    };

    println!("Answer: {}", result);
}

fn calories_by_elves(contents: &str) -> BinaryHeap<i32> {
    let mut heap = BinaryHeap::new();
    let mut total = 0;
    for line in contents.lines() {
        match line {
            "" => {
                heap.push(total);
                total = 0;
            }
            _ => {
                total += line.parse::<i32>().unwrap();
            }
        };
    }
    heap.push(total);
    heap
}

fn problem_1(contents: &str) -> i32 {
    let heap = calories_by_elves(contents);
    *heap.peek().unwrap()
}

fn problem_2(contents: &str) -> i32 {
    let heap = calories_by_elves(contents);
    heap.into_sorted_vec().into_iter().rev().take(3).sum()
}
