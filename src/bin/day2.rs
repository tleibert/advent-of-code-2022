use std::str::FromStr;

use advent_of_code_2022::{read_file_and_get_input, Problem};

/// A possible move in rock paper scissors
#[derive(Eq, PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Loss,
    Tie,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Move {
    fn play_game(self, other: Self) -> GameResult {
        match (self, other) {
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => GameResult::Tie,
            (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper)
            | (Self::Rock, Self::Scissors) => GameResult::Win,
            _ => GameResult::Loss,
        }
    }

    // returns the move needed to achieve the desired result
    fn move_needed(result: GameResult, other: Move) -> Move {
        match result {
            GameResult::Win => match other {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            GameResult::Loss => match other {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            GameResult::Tie => other,
        }
    }
}

fn main() {
    let (problem, contents) = read_file_and_get_input();

    let result = match problem {
        Problem::One => problem_1(&contents),
        Problem::Two => problem_2(&contents),
    };

    println!("Answer: {result}");
}

fn problem_1(contents: &str) -> i32 {
    let mut score = 0;
    for line in contents.lines() {
        let (other, mine) = line.split_once(' ').unwrap();
        let other_move: Move = other.parse().unwrap();
        let my_move: Move = mine.parse().unwrap();
        match my_move {
            Move::Rock => score += 1,
            Move::Paper => score += 2,
            Move::Scissors => score += 3,
        }
        match my_move.play_game(other_move) {
            GameResult::Win => score += 6,
            GameResult::Tie => score += 3,
            GameResult::Loss => (),
        }
    }
    score
}

fn problem_2(contents: &str) -> i32 {
    let mut score = 0;
    for line in contents.lines() {
        let (other, result) = line.split_once(' ').unwrap();
        let other_move: Move = other.parse().unwrap();
        let result: GameResult = result.parse().unwrap();

        let my_move = Move::move_needed(result, other_move);
        match my_move {
            Move::Rock => score += 1,
            Move::Paper => score += 2,
            Move::Scissors => score += 3,
        }
        match my_move.play_game(other_move) {
            GameResult::Win => score += 6,
            GameResult::Tie => score += 3,
            GameResult::Loss => (),
        }
    }
    score
}
