use std::ops::RangeInclusive;

use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("04");

fn contains_all(outer: &RangeInclusive<u8>, inner: &RangeInclusive<u8>) -> bool {
    outer.start() <= inner.start() && outer.end() >= inner.end()
}

fn contains_any(outer: &RangeInclusive<u8>, inner: &mut RangeInclusive<u8>) -> bool {
    inner.any(|part| outer.contains(&part))
}

fn parse(input: &str) -> Vec<(RangeInclusive<u8>, RangeInclusive<u8>)> {
    input
        .lines()
        .map(|line| {
            let mut ranges = line.split(',').map(|range| {
                let mut sides = range.split('-').map(|side| side.parse::<u8>().unwrap());
                let start = sides.next().unwrap();
                let end = sides.next().unwrap();
                start..=end
            });
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect()
}

fn solve_first(input: &str) -> String {
    parse(input)
        .iter()
        .cloned()
        .filter(|(first, second)| contains_all(first, second) || contains_all(second, first))
        .count()
        .to_string()
}

fn solve_second(input: &str) -> String {
    parse(input)
        .iter()
        .cloned()
        .filter(|range| {
            let (ref first, ref mut second) = range.clone();
            contains_any(first, second)
        })
        .count()
        .to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 2, solve_first(&example));
        println!("Second: Expected {} found {}.", 4, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
