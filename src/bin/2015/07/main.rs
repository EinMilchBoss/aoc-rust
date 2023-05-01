mod parser;

use std::collections::HashMap;

use parser::*;
use util::std::*;

const YEAR: Year = Year("2015");
const DAY: Day = Day("07");

fn solve_first(input: &str) -> String {
    let instructions = parse_instructions(input).unwrap();
    let mut wires: HashMap<Wire, Signal> = HashMap::new();
    for instruction in instructions {
        match instruction.command {
            Command::Set(new_signal) => {
                *wires.entry(instruction.destination).or_insert(new_signal) = new_signal
            }
            Command::And(left, right) => {
                let left = wires.get(&left).unwrap();
                let right = wires.get(&right).unwrap();
                let new_signal = Signal(left.0 & right.0);
                *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
            }
            Command::Or(left, right) => {
                let left = wires.get(&left).unwrap();
                let right = wires.get(&right).unwrap();
                let new_signal = Signal(left.0 | right.0);
                *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
            }
            Command::LeftShift(wire, shift) => {
                let wire = wires.get(&wire).unwrap();
                let new_signal = Signal(wire.0 << shift);
                *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
            }
            Command::RightShift(wire, shift) => {
                let wire = wires.get(&wire).unwrap();
                let new_signal = Signal(wire.0 >> shift);
                *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
            }
            Command::Not(wire) => {
                let wire = wires.get(&wire).unwrap();
                let new_signal = Signal(!wire.0);
                *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
            }
        };
    }

    dbg!(wires);

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
