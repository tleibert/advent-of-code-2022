use std::{collections::VecDeque, str::FromStr};

use advent_of_code_2022::{read_file_and_get_input, Problem};

#[derive(Debug, Clone, Copy)]
enum Operation {
    AddX(i32),
    NoOp,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::NoOp)
        } else if s.starts_with("addx") {
            let (_, amount) = s.split_once(' ').ok_or(())?;
            let amount = amount.parse().map_err(|_| ())?;
            Ok(Self::AddX(amount))
        } else {
            Err(())
        }
    }
}

struct Cpu {
    counter: i32,
    x_reg: i32,
    ops: VecDeque<Operation>,
    is_adding: bool,
}

impl Cpu {
    fn new() -> Self {
        Self {
            counter: 1,
            x_reg: 1,
            ops: VecDeque::new(),
            is_adding: false,
        }
    }

    fn add_op(&mut self, op: Operation) {
        self.ops.push_back(op)
    }

    fn run_one_clock(&mut self) {
        match self.ops.front() {
            Some(&Operation::AddX(amount)) => {
                if self.is_adding {
                    self.counter += 1;
                    self.is_adding = false;
                    self.x_reg += amount;
                    self.ops.pop_front();
                } else {
                    self.counter += 1;
                    self.is_adding = true;
                }
            }
            Some(&Operation::NoOp) => {
                self.counter += 1;
                self.ops.pop_front();
            }
            None => (),
        }
    }

    fn signal_strength(&self) -> i32 {
        self.counter * self.x_reg
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

fn problem_1(contents: &str) -> i32 {
    let mut cpu = Cpu::new();
    let mut strengths = Vec::new();

    for line in contents.lines() {
        let op = line.parse().unwrap();
        cpu.add_op(op);
    }

    for i in 1..=220 {
        let strength = cpu.signal_strength();
        if i % 40 == 20 {
            strengths.push(strength);
        }
        cpu.run_one_clock();
    }

    strengths.iter().sum()
}

fn problem_2(contents: &str) -> i32 {
    let mut cpu = Cpu::new();

    for line in contents.lines() {
        let op = line.parse().unwrap();
        cpu.add_op(op);
    }

    let mut draw_pos = 0;
    let width = 40;
    while !cpu.ops.is_empty() {
        let x_pos = cpu.x_reg;
        if x_pos.abs_diff(draw_pos) <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        draw_pos += 1;

        // print new line if needed
        if draw_pos == width {
            draw_pos = 0;
            println!();
        }

        cpu.run_one_clock();
    }

    0
}
