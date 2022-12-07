use advent_of_code_2022::{read_file_and_get_input, Problem};

fn main() {
    let (problem, contents) = read_file_and_get_input();

    let answer = match problem {
        Problem::One => problem_1(&contents),
        Problem::Two => problem_2(&contents),
    };

    println!("Answer: {answer}");
}

fn problem_1(_contents: &str) -> i32 {
    0
}

fn problem_2(_contents: &str) -> i32 {
    0
}
