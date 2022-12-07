use advent_of_code_2022::{read_file_and_get_input, Problem};
use regex::Regex;

#[derive(Debug)]
struct CrateStack {
    crates: Vec<Vec<char>>,
}

impl CrateStack {
    fn from_initial(input: &str) -> Self {
        let relevant_lines: Vec<&str> = input.lines().take_while(|line| !line.is_empty()).collect();

        let indices: Vec<usize> = relevant_lines[relevant_lines.len() - 1]
            .char_indices()
            .filter_map(|(idx, c)| if c.is_numeric() { Some(idx) } else { None })
            .collect();

        let mut new = Self {
            crates: vec![vec![]; indices.len()],
        };

        for line in relevant_lines[0..(relevant_lines.len() - 1)].iter().rev() {
            for (col, idx) in indices.iter().enumerate() {
                match line.chars().nth(*idx) {
                    Some(c) if c.is_alphabetic() => new.add_crate(col, c),
                    _ => continue,
                }
            }
        }

        new
    }

    fn add_crate(&mut self, col: usize, mark: char) {
        self.crates[col].push(mark)
    }

    fn move_one_crate(&mut self, from: usize, to: usize) {
        let c = self.crates[from - 1].pop().unwrap();
        self.crates[to - 1].push(c);
    }

    fn move_n_crates(&mut self, from: usize, to: usize, count: usize) {
        let mut temp = Vec::with_capacity(count);
        for _ in 0..count {
            let c = self.crates[from - 1].pop().unwrap();
            temp.push(c);
        }

        for _ in 0..count {
            let c = temp.pop().unwrap();
            self.crates[to - 1].push(c);
        }
    }

    fn get_top_crates(&self) -> Vec<char> {
        self.crates
            .iter()
            .map(|stack| *stack.last().unwrap_or(&' '))
            .collect()
    }
}

fn main() {
    let (problem, contents) = read_file_and_get_input();

    let answer = match problem {
        Problem::One => problem_1(&contents),
        Problem::Two => problem_2(&contents),
    };

    println!("Answer: {answer}");
}

fn problem_1(contents: &str) -> String {
    let mut crates = CrateStack::from_initial(contents);
    let line_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for cap in line_regex.captures_iter(contents) {
        // split out count, source, and destination from line that looks like
        // "move x from y to z"
        let count = cap[1].parse().unwrap();
        let source = cap[2].parse().unwrap();
        let dest = cap[3].parse().unwrap();

        for _ in 0..count {
            crates.move_one_crate(source, dest);
        }
    }

    crates.get_top_crates().into_iter().collect()
}

fn problem_2(contents: &str) -> String {
    let mut crates = CrateStack::from_initial(contents);
    let line_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for cap in line_regex.captures_iter(contents) {
        // split out count, source, and destination from line that looks like
        // "move x from y to z"
        let count = cap[1].parse().unwrap();
        let source = cap[2].parse().unwrap();
        let dest = cap[3].parse().unwrap();

        crates.move_n_crates(source, dest, count);
    }

    crates.get_top_crates().into_iter().collect()
}
