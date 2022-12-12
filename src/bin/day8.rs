use advent_of_code_2022::{read_file_and_get_input, Problem};

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

fn problem_2(_contents: &str) -> usize {
    0
}
