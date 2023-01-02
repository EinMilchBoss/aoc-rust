use std::collections::HashSet;

use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("03");

fn split_in_half(string: &str) -> (&str, &str) {
    let half = string.len() / 2;
    (&string[..half], &string[half..])
}

fn unique_chars(string: &str) -> HashSet<char> {
    string.chars().collect()
}

fn priority(char: &char) -> u32 {
    match char {
        'a'..='z' => *char as u32 - 'a' as u32 + 1,
        'A'..='Z' => *char as u32 - 'A' as u32 + 27,
        _ => panic!("Char {char} couldn't be parsed to priority"),
    }
}

fn solve_first(input: &str) -> String {
    let compartments: Vec<_> = input.lines().map(split_in_half).collect();

    let unique_compartments: Vec<_> = compartments
        .iter()
        .map(|(first, second)| (unique_chars(first), unique_chars(second)))
        .collect();

    let duplicates: Vec<_> = unique_compartments
        .iter()
        .map(|(first, second)| *first.intersection(second).next().unwrap())
        .collect();

    let priority: Vec<_> = duplicates.iter().map(priority).collect();

    priority.iter().sum::<u32>().to_string()
}

fn solve_second(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect::<Vec<_>>();

    let groups: Vec<_> = lines.chunks(3).collect();

    let unique_groups: Vec<_> = groups
        .iter()
        .map(|chunk| {
            if let [first, second, third] = chunk {
                (
                    unique_chars(first),
                    unique_chars(second),
                    unique_chars(third),
                )
            } else {
                panic!("Chunk could not be split into 3 elements.");
            }
        })
        .collect();

    let duplicates: Vec<_> = unique_groups
        .iter()
        .map(|(first, second, third)| {
            let duplicates: HashSet<_> = first.intersection(second).cloned().collect();
            let duplicate = duplicates.intersection(third).next().unwrap();
            *duplicate
        })
        .collect();

    let priority: Vec<_> = duplicates.iter().map(priority).collect();

    priority.iter().sum::<u32>().to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 157, solve_first(&example));
        println!("Second: Expected {} found {}.", 70, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
