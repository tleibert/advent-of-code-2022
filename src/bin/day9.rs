use std::{collections::HashSet, str::FromStr};

use advent_of_code_2022::{read_file_and_get_input, Problem};

struct Rope {
    positions: Vec<(i32, i32)>,
    seen_pos: HashSet<(i32, i32)>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

impl Rope {
    fn new(num_knots: usize) -> Self {
        assert!(num_knots > 1, "Must have at least two knots!");
        Self {
            positions: vec![(0, 0); num_knots],
            seen_pos: HashSet::new(),
        }
    }

    fn move_knot(&mut self, knot_id: usize) {
        assert!(knot_id > 0 && knot_id < self.positions.len());
        let (pos_1_x, pos_1_y) = self.positions[knot_id - 1];
        let (pos_2_x, pos_2_y) = self.positions[knot_id];
        let x_diff = pos_1_x - pos_2_x;
        let y_diff = pos_1_y - pos_2_y;

        let (knot_x, knot_y) = &mut self.positions[knot_id];
        match (x_diff, y_diff) {
            (x_diff, 0) if x_diff.abs() > 1 => *knot_x += x_diff.signum(),
            (0, y_diff) if y_diff.abs() > 1 => *knot_y += y_diff.signum(),
            (x_diff, y_diff) if x_diff.abs() > 1 || y_diff.abs() > 1 => {
                *knot_x += x_diff.signum();
                *knot_y += y_diff.signum();
            }
            _ => (),
        }
    }

    fn handle_unit_move(&mut self, direction: Direction) {
        let (head_x, head_y) = &mut self.positions[0];

        match direction {
            Direction::Down => *head_y -= 1,
            Direction::Up => *head_y += 1,
            Direction::Left => *head_x -= 1,
            Direction::Right => *head_x += 1,
        }

        for i in 1..self.positions.len() {
            self.move_knot(i);
        }

        self.seen_pos.insert(*self.positions.last().unwrap());
    }

    fn handle_move(&mut self, direction: Direction, amount: usize) {
        for _ in 0..amount {
            self.handle_unit_move(direction);
        }
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
    let mut rope = Rope::new(2);
    for line in contents.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let direction = direction.parse().unwrap();
        let amount = amount.parse().unwrap();
        rope.handle_move(direction, amount);
    }
    rope.seen_pos.len()
}

fn problem_2(contents: &str) -> usize {
    let mut rope = Rope::new(10);
    for line in contents.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let direction = direction.parse().unwrap();
        let amount = amount.parse().unwrap();
        rope.handle_move(direction, amount);
    }
    rope.seen_pos.len()
}
