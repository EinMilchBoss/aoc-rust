use crate::code_solver::{ButtonLocation, ButtonNumber};

use std::str::{self, FromStr};

use snafu::prelude::*;

#[derive(Debug, PartialEq, Snafu)]
#[snafu(display("String could not be parsed into `CodeInstructions` because of char '{}' at index {}.", source.invalid_char, invalid_char_index))]
pub struct CodeInstructionsParseError {
    source: InstructionParseError,
    parsed: String,
    invalid_char_index: usize,
}

#[derive(Debug, PartialEq, Snafu)]
#[snafu(display("Char '{}' could not be parsed into `Instruction`.", invalid_char))]
pub struct InstructionParseError {
    invalid_char: char,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeInstructions(Vec<Instruction>);

impl CodeInstructions {
    pub fn new(instructions: &[Instruction]) -> Self {
        Self(instructions.to_vec())
    }

    pub fn solve_code_number(&self, button_location: &mut ButtonLocation) -> ButtonNumber {
        let Self(instructions) = self;
        for instruction in instructions {
            button_location.go_relative(*instruction);
        }
        button_location.button_number()
    }
}

impl FromStr for CodeInstructions {
    type Err = CodeInstructionsParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        for (index, char) in string.chars().enumerate() {
            match char.try_into() {
                Ok(instruction) => instructions.push(instruction),
                Err(error) => {
                    return Err(CodeInstructionsParseError {
                        source: error,
                        parsed: String::from(&string[..index]),
                        invalid_char_index: index,
                    })
                }
            }
        }
        Ok(CodeInstructions(instructions))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Up,
    Down,
    Right,
    Left,
}

impl TryFrom<char> for Instruction {
    type Error = InstructionParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'R' => Ok(Self::Right),
            'L' => Ok(Self::Left),
            _ => Err(InstructionParseError {
                invalid_char: value,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn code_instructions_new_test() {
        let input = vec![
            Instruction::Up,
            Instruction::Down,
            Instruction::Left,
            Instruction::Right,
        ];

        let actual = CodeInstructions::new(&input);

        assert_eq!(CodeInstructions(input), actual);
    }

    #[test]
    fn code_instructions_solve_code_number_test_in_bounds() {
        let mut button_location = ButtonLocation::at_start();
        let code_instructions = CodeInstructions::new(&[
            Instruction::Up,
            Instruction::Down,
            Instruction::Right,
            Instruction::Left,
        ]);

        let returned = code_instructions.solve_code_number(&mut button_location);

        assert_eq!(ButtonNumber(5), returned);
        assert_eq!(ButtonLocation { x: 0, y: 0 }, button_location);
    }

    #[test]
    fn code_instructions_solve_code_number_test_out_of_bounds() {
        let mut button_location = ButtonLocation::at_start();
        let code_instructions = CodeInstructions::new(&[
            Instruction::Up,
            Instruction::Up,
            Instruction::Left,
            Instruction::Left,
        ]);

        let returned = code_instructions.solve_code_number(&mut button_location);

        assert_eq!(ButtonNumber(1), returned);
        assert_eq!(ButtonLocation { x: -1, y: 1 }, button_location);
    }

    #[test]
    fn code_instructions_trait_from_str_test_ok() {
        let expected = Ok(CodeInstructions(vec![
            Instruction::Up,
            Instruction::Down,
            Instruction::Right,
            Instruction::Left,
        ]));

        assert_eq!(expected, "UDRL".parse());
    }

    #[test]
    fn code_instructions_trait_from_str_test_err() {
        let expected: Result<CodeInstructions, _> = Err(CodeInstructionsParseError {
            source: InstructionParseError { invalid_char: 'X' },
            parsed: String::from("UDRL"),
            invalid_char_index: 4,
        });

        assert_eq!(expected, "UDRLXUDRL".parse());
    }

    #[rstest]
    #[case(Ok(Instruction::Up), 'U')]
    #[case(Ok(Instruction::Down), 'D')]
    #[case(Ok(Instruction::Right), 'R')]
    #[case(Ok(Instruction::Left), 'L')]
    #[case(Err(InstructionParseError { invalid_char: 'X' }), 'X')]
    fn instruction_trait_try_from_test(
        #[case] expected: Result<Instruction, InstructionParseError>,
        #[case] char_to_be_parsed: char,
    ) {
        assert_eq!(expected, char_to_be_parsed.try_into());
    }
}
