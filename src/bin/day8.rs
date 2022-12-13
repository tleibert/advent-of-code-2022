use advent_of_code_2022::{read_file_and_get_input, Problem};

use take_until::TakeUntilExt;
/// Stores data in row-major order
struct Grid {
    data: Vec<usize>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(contents: &str) -> Self {
        let rows = contents.lines().count();
        let cols = contents.lines().next().unwrap().len();
        let data = contents
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
            .collect();

        Self { data, rows, cols }
    }

    fn get(&self, row: usize, col: usize) -> usize {
        self.data[col + row * self.cols]
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        let val = self.get(row, col);

        let east_visible = (0..col).rev().all(|col| self.get(row, col) < val);
        let west_visible = ((col + 1)..self.cols).all(|col| self.get(row, col) < val);
        let north_visible = (0..row).rev().all(|row| self.get(row, col) < val);
        let south_visible = ((row + 1)..self.rows).all(|row| self.get(row, col) < val);

        east_visible || west_visible || north_visible || south_visible
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        let our_height = self.get(row, col);

        let east_score = (0..col)
            .rev()
            .take_until(|&col| self.get(row, col) >= our_height)
            .count();

        let west_score = ((col + 1)..self.cols)
            .take_until(|&col| self.get(row, col) >= our_height)
            .count();

        let north_score = (0..row)
            .rev()
            .take_until(|&row| self.get(row, col) >= our_height)
            .count();

        let south_score = ((row + 1)..self.rows)
            .take_until(|&row| self.get(row, col) >= our_height)
            .count();

        east_score * west_score * north_score * south_score
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

fn problem_1(contents: &str) -> usize {
    // construct grid of numbers
    let nums = Grid::new(contents);
    let mut total = 0;
    for row in 0..nums.rows {
        for col in 0..nums.cols {
            if nums.is_visible(row, col) {
                total += 1;
            }
        }
    }
    total
}

fn problem_2(contents: &str) -> usize {
    let nums = Grid::new(contents);
    // disregard outer trees, since the product of their scores will always be zero
    (1..(nums.rows - 1))
        .map(|row| {
            (1..(nums.cols - 1))
                .map(|col| nums.scenic_score(row, col))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}
