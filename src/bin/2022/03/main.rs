use std::{collections::{HashSet, HashMap}, hash::Hash, path::Iter};

use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("03");

fn solve_first(input: &str) -> String {
    // split to half
    // get unique char from first
    // check for every unique char of first if also in second
    //   true: add to vec of char
    // map chars to int and sum
    let compartments: Vec<_> = input.lines()
        .map(|line| {
            let half = line.len() / 2;
            (&line[..half], &line[half..])
        })
        .collect();

    let unique_compartments: Vec<_> = compartments
        .iter()
        .map(|(first, second)| {
            let first: HashSet<_> = first.chars().collect();
            let second: HashSet<_> = second.chars().collect();
            (first, second)
        })
        .collect();

    let duplicates: Vec<_> = unique_compartments
        .iter()
        .map(|(first, second)| first.intersection(second).next().unwrap())
        .collect();

    let priority: Vec<_> = duplicates
        .iter()
        .map(|char| match char {
            'a'..='z' => **char as u32 - 'a' as u32 + 1,
            'A'..='Z' => **char as u32 - 'A' as u32 + 27,
            _ => panic!("Char {char} couldn't be parsed to priority"),
        })
        .collect();

        /*
    dbg!(&compartments);


    dbg!(&unique_compartments);

    dbg!(&duplicates);

    dbg!(&priority);
     */


    priority.iter().sum::<u32>().to_string()
}

fn solve_second(input: &str) -> String {
// split to half
    // get unique char from first
    // check for every unique char of first if also in second
    //   true: add to vec of char
    // map chars to int and sum
    let line_chars: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let groups: Vec<_> = line_chars
        .chunks(3)
        .collect();

    let unique_groups: Vec<_> = groups
        .iter()
        .map(|chunk| {
            if let [first, second, third] = chunk {
                let first: HashSet<_> = HashSet::from_iter(first);
                let second: HashSet<_> = HashSet::from_iter(second);
                let third: HashSet<_> = HashSet::from_iter(third);
                (first, second, third)
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

    let priority: Vec<_> = duplicates
        .iter()
        .map(|char| match char {
            'a'..='z' => **char as u32 - 'a' as u32 + 1,
            'A'..='Z' => **char as u32 - 'A' as u32 + 27,
            _ => panic!("Char {char} couldn't be parsed to priority"),
        })
        .collect();

    /*dbg!(&groups);


    dbg!(&unique_groups);

    dbg!(&duplicates);

    dbg!(&priority);
    */


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