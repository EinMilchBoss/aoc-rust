use util::std::*;

const YEAR: Year = Year("2015");
const DAY: Day = Day("01");

fn parse_char(char: char) -> i32 {
    match char {
        '(' => 1,
        ')' => -1,
        _ => panic!("Char {} is not a bracket.", char),
    }
}

fn solve_first(input: &str) -> String {
    input.chars().map(parse_char).sum::<i32>().to_string()
}

fn solve_second(input: &str) -> String {
    let mut floor = 0;
    let index = input
        .chars()
        .position(|char| {
            floor += parse_char(char);
            floor < 0
        })
        .expect("At no point did Santa go into the basement.");

    (index + 1).to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", -3, solve_first(&example));
        println!("Second: Expected {} found {}.", 1, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
