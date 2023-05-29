mod door_code;
mod instruction;

use util::std::*;

use door_code::normal_keypad;
use instruction::{CodeInstructions, CodeInstructionsParseError};

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
    let mut button = normal_keypad::KeypadButton::at_start();
    input
        .iter()
        .map(|code_instructions| code_instructions.solve_code_number(&mut button).0)
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;
    use crate::instruction::Instruction;

    #[fixture]
    fn input_lines() -> Vec<String> {
        vec![String::from("UDRL"), String::from("LRDU")]
    }

    #[rstest]
    fn parse_input_test_ok(input_lines: Vec<String>) {
        let input = input_lines.join("\n");
        let expected = vec![
            CodeInstructions::new(vec![
                Instruction::Up,
                Instruction::Down,
                Instruction::Right,
                Instruction::Left,
            ]),
            CodeInstructions::new(vec![
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
            CodeInstructions::new(vec![Instruction::Up, Instruction::Left, Instruction::Left]),
            CodeInstructions::new(vec![
                Instruction::Right,
                Instruction::Right,
                Instruction::Down,
                Instruction::Down,
                Instruction::Down,
            ]),
            CodeInstructions::new(vec![
                Instruction::Left,
                Instruction::Up,
                Instruction::Right,
                Instruction::Down,
                Instruction::Left,
            ]),
            CodeInstructions::new(vec![
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
