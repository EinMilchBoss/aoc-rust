mod code_solver;
mod instruction;

use code_solver::{ButtonLocation, ButtonNumber};
use instruction::{CodeInstructions, CodeInstructionsParseError};
use util::std::*;

fn main() {
    let input = read_file(InputFile::Actual, Year("2016"), Day("02"))
        .expect("Input file could not be read.");
    let input = parse_input(&input);

    println!("Part 1: {}", part_1(&input));
}

fn parse_input(input: &str) -> Vec<CodeInstructions> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .unwrap_or_else(|error: CodeInstructionsParseError| panic!("{}", error))
        })
        .collect()
}

fn part_1(input: &[CodeInstructions]) -> String {
    let mut button_numbers = Vec::new();
    let mut button_location = ButtonLocation::at_start();
    for code_instruction in input {
        let ButtonNumber(code_number) = code_instruction.solve_code_number(&mut button_location);
        button_numbers.push(
            char::from_digit(code_number.into(), 10)
                .expect("A `ButtonNumber` could not be parsed into a `char`."),
        );
    }
    button_numbers.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    use rstest::{fixture, rstest};

    #[fixture]
    fn input_lines() -> Vec<String> {
        vec![String::from("UDRL"), String::from("LRDU")]
    }

    #[rstest]
    fn parse_input_test_ok(input_lines: Vec<String>) {
        let input = input_lines.join("\n");
        let expected = vec![
            CodeInstructions(vec![
                Instruction::Up,
                Instruction::Down,
                Instruction::Right,
                Instruction::Left,
            ]),
            CodeInstructions(vec![
                Instruction::Left,
                Instruction::Right,
                Instruction::Down,
                Instruction::Up,
            ]),
        ];

        assert_eq!(expected, parse_input(&input));
    }

    #[rstest]
    #[should_panic]
    fn parse_input_test_err(input_lines: Vec<String>) {
        let invalid_input = input_lines.join("*");

        parse_input(&invalid_input);
    }

    #[fixture]
    fn aoc_test_input() -> Vec<CodeInstructions> {
        vec![
            CodeInstructions(vec![Instruction::Up, Instruction::Left, Instruction::Left]),
            CodeInstructions(vec![
                Instruction::Right,
                Instruction::Right,
                Instruction::Down,
                Instruction::Down,
                Instruction::Down,
            ]),
            CodeInstructions(vec![
                Instruction::Left,
                Instruction::Up,
                Instruction::Right,
                Instruction::Down,
                Instruction::Left,
            ]),
            CodeInstructions(vec![
                Instruction::Up,
                Instruction::Up,
                Instruction::Up,
                Instruction::Up,
                Instruction::Down,
            ]),
        ]
    }

    #[rstest]
    fn part_1_test(aoc_test_input: Vec<CodeInstructions>) {
        assert_eq!(String::from("1985"), part_1(&aoc_test_input));
    }
}
