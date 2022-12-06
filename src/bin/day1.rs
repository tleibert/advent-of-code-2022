use std::collections::BinaryHeap;
use std::env;

fn main() {
    // get the problem to run and the input file

    let problem: i32 = env::args().nth(1).unwrap().parse().unwrap();
    let filename: &str = &env::args().nth(2).unwrap();

    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let result = match problem {
        1 => problem_1(&contents),
        2 => problem_2(&contents),
        _ => panic!("Unknown problem"),
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
