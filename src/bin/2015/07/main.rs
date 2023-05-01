mod parser;

use util::std::*;

const YEAR: Year = Year("2015");
const DAY: Day = Day("07");

fn solve_first(input: &str) -> String {
    dbg!(parser::parse_instructions(input).unwrap());

    "".to_string()
}

fn solve_second(input: &str) -> String {
    "".to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 65412, solve_first(&example));
        println!("Second: Expected {} found {}.", 0, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
