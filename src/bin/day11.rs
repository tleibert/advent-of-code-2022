#![feature(binary_heap_into_iter_sorted)]
use std::{collections::BinaryHeap, str::FromStr};

use advent_of_code_2022::{read_file_and_get_input, Problem};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply(usize),
    Add(usize),
    Square,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, op_rh) = s.split_once('=').ok_or(())?;
        let op_rh = op_rh.trim();

        if op_rh == "old * old" {
            Ok(Self::Square)
        } else if op_rh.contains("+") {
            let (_, amount) = op_rh.split_once("+").ok_or(())?;
            let amount = amount.trim().parse().map_err(|_| ())?;
            Ok(Self::Add(amount))
        } else if op_rh.contains("*") {
            let (_, amount) = op_rh.split_once("*").ok_or(())?;
            let amount = amount.trim().parse().map_err(|_| ())?;
            Ok(Self::Multiply(amount))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Decision {
    modulus: usize,
    choice_a: usize,
    choice_b: usize,
}

impl Decision {
    fn new(modulus: usize, choice_a: usize, choice_b: usize) -> Self {
        Self {
            modulus,
            choice_a,
            choice_b,
        }
    }
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    decision: Decision,
    items_inspected: usize,
    id: usize,
}

impl Monkey {
    fn new(id: usize, items: Vec<usize>, operation: Operation, decision: Decision) -> Self {
        Self {
            items,
            operation,
            decision,
            items_inspected: 0,
            id,
        }
    }

    fn take_turn(&mut self, arr_a: &mut Vec<usize>, arr_b: &mut Vec<usize>) {
        for &item in &self.items {
            let new = self.apply_operation(item) / 3;

            if new % self.decision.modulus == 0 {
                arr_a.push(new);
            } else {
                arr_b.push(new);
            }

            self.items_inspected += 1;
        }
        self.items.clear();
    }

    fn apply_operation(&self, num: usize) -> usize {
        match self.operation {
            Operation::Multiply(amount) => num * amount,
            Operation::Add(amount) => num + amount,
            Operation::Square => num * num,
        }
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines
            .next()
            .ok_or(())?
            .trim_start_matches("Monkey ")
            .trim_end_matches(":")
            .parse()
            .map_err(|_| ())?;
        let items = lines
            .next()
            .ok_or(())?
            .trim()
            .trim_start_matches("Starting items: ");
        let items = items
            .split(", ")
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| ())?;
        let op = lines
            .next()
            .ok_or(())?
            .trim()
            .trim_start_matches("Operation: ");
        let op: Operation = op.parse()?;
        let test = lines
            .next()
            .ok_or(())?
            .trim()
            .trim_start_matches("Test: divisible by ")
            .parse()
            .map_err(|_| ())?;
        let option_a = lines
            .next()
            .ok_or(())?
            .trim()
            .trim_start_matches("If true: throw to monkey ")
            .parse()
            .map_err(|_| ())?;
        let option_b = lines
            .next()
            .ok_or(())?
            .trim()
            .trim_start_matches("If false: throw to monkey ")
            .parse()
            .map_err(|_| ())?;

        let decision = Decision::new(test, option_a, option_b);

        Ok(Monkey::new(id, items, op, decision))
    }
}

struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    common_multiple: usize,
}

impl MonkeyGroup {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let common_multiple = monkeys
            .iter()
            .map(|monkey| monkey.decision.modulus)
            .product();
        Self {
            monkeys,
            common_multiple,
        }
    }

    fn play_round(&mut self) {
        let mut array_a = Vec::new();
        let mut array_b = Vec::new();

        for i in 0..self.monkeys.len() {
            self.monkeys[i].take_turn(&mut array_a, &mut array_b);
            let monkey_a = self.monkeys[i].decision.choice_a;
            let monkey_b = self.monkeys[i].decision.choice_b;

            self.monkeys[monkey_a].items.extend(array_a.iter());
            self.monkeys[monkey_b].items.extend(array_b.iter());
            array_a.clear();
            array_b.clear();
        }
    }

    fn monkey_business(&self) -> usize {
        let monkey_scores: BinaryHeap<usize> =
            self.monkeys.iter().map(|m| m.items_inspected).collect();
        monkey_scores.into_iter_sorted().take(2).product()
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
    let mut monkeys = Vec::new();
    for mut lines in &contents.lines().chunks(7) {
        let line = lines.join("\n");
        let monkey: Monkey = line.parse().unwrap();
        monkeys.push(monkey);
    }

    let mut monkey_group = MonkeyGroup::new(monkeys);
    println!("Common multiple {}", monkey_group.common_multiple);

    for _ in 0..20 {
        monkey_group.play_round();
    }

    monkey_group.monkey_business()
}

fn problem_2(contents: &str) -> usize {
    0
}
