use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use advent_of_code_2022::{read_file_and_get_input, Problem};

fn main() {
    let (problem, contents) = read_file_and_get_input();

    let answer = match problem {
        Problem::One => problem_1(&contents),
        Problem::Two => problem_2(&contents),
    };

    println!("Answer: {answer}");
}

fn size_of_dir(files: &HashMap<PathBuf, usize>, dir: &Path) -> usize {
    files
        .iter()
        .filter_map(|(k, v)| if k.starts_with(dir) { Some(*v) } else { None })
        .sum()
}

fn process_input(contents: &str) -> (HashMap<PathBuf, usize>, HashSet<PathBuf>) {
    let mut cwd = PathBuf::from("/");
    // map of files to their sizes
    let mut files = HashMap::new();
    let mut dirs = HashSet::new();
    dirs.insert(cwd.clone());
    for line in contents.lines() {
        if line.starts_with('$') {
            // command
            let line = line.strip_prefix('$').unwrap().trim_start();
            if line.starts_with("cd") {
                let dirname = line.split_whitespace().nth(1).unwrap();
                if dirname == ".." {
                    cwd.pop();
                } else if dirname == "/" {
                    cwd = PathBuf::from("/");
                } else {
                    cwd = cwd.join(dirname);
                    dirs.insert(cwd.clone());
                }
            }
        } else if line.starts_with("dir") {
            let name = line.split_whitespace().nth(1).unwrap();
            dirs.insert(cwd.join(name));
        } else {
            // create file
            let (size, name) = line.split_once(' ').unwrap();
            let size = size.parse().unwrap();
            let new_path = cwd.join(name);
            files.insert(new_path, size);
        }
    }
    (files, dirs)
}

fn problem_1(contents: &str) -> usize {
    let (files, dirs) = process_input(contents);

    dirs.iter()
        .filter_map(|dir| {
            let size = size_of_dir(&files, dir);
            if size <= 100000 {
                Some(size)
            } else {
                None
            }
        })
        .sum()
}

fn problem_2(contents: &str) -> usize {
    let (files, dirs) = process_input(contents);

    let target_size = 70000000 - 30000000;
    let current_size = size_of_dir(&files, Path::new("/"));

    dirs.iter()
        .filter_map(|dir| {
            let dir_size = size_of_dir(&files, dir);
            let new_size = current_size - dir_size;

            if new_size <= target_size {
                Some(dir_size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}
