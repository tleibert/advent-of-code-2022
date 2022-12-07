use std::{env, process::exit, str::FromStr};

#[derive(Clone, Copy)]
pub enum Problem {
    One,
    Two,
}

impl FromStr for Problem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            _ => Err(()),
        }
    }
}

fn usage_exit() -> ! {
    // print usage message
    println!(
        "Usage: {} [1,2] <path to input file>",
        env::args().next().unwrap()
    );
    exit(1);
}

pub fn read_file_and_get_input() -> (Problem, String) {
    let Some(problem) = env::args().nth(1)  else {
        usage_exit();
    };
    let Ok(problem) = problem.parse() else {
        println!("Invalid problem, must be 1 or 2");
        exit(1);
    };
    let Some(filename) = env::args().nth(2) else {
        usage_exit();
    };
    let Ok(contents) = std::fs::read_to_string(&filename) else {
        println!("Couldn't read file {filename}.");
        exit(1);
    };
    (problem, contents)
}
