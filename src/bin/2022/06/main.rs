use std::collections::HashSet;

use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("06");

fn all_unique(elements: &Vec<char>) -> bool {
    let unique = HashSet::<char>::from_iter(elements.clone());
    elements.len() == unique.len()
}

fn solve(input: &str, window_size: usize) -> String {
    let chars = input.chars().enumerate().collect::<Vec<_>>();
    let start_window = chars
        .windows(window_size)
        .filter(|window| {
            let values = window.iter().map(|(_, value)| *value).collect::<Vec<_>>();
            all_unique(&values)
        })
        .next()
        .unwrap();
    let (index, _) = start_window.last().unwrap();
    (index + 1).to_string()
}

fn solve_first(input: &str) -> String {
    solve(input, 4)
}

fn solve_second(input: &str) -> String {
    solve(input, 14)
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", "7", solve_first(&example));
        println!(
            "Second: Expected {} found {}.",
            "19",
            solve_second(&example)
        );
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
