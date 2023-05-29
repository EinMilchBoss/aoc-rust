use std::str::FromStr;

use snafu::prelude::*;

use crate::door_code::{Button, ButtonLocation, ButtonNumber};

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
    #[cfg(test)]
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self(instructions)
    }

    pub fn solve_code_number<B>(&self, button: &mut B) -> ButtonNumber
    where
        B: Button,
    {
        let Self(instructions) = self;
        for instruction in instructions {
            button.follow_instruction(*instruction);
        }
        button.button_number()
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

impl Instruction {
    pub fn button_position_offset(&self) -> ButtonLocation {
        match self {
            Self::Up => ButtonLocation::at(0, 1),
            Self::Down => ButtonLocation::at(0, -1),
            Self::Right => ButtonLocation::at(1, 0),
            Self::Left => ButtonLocation::at(-1, 0),
        }
    }
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
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::door_code::normal_keypad;

    #[test]
    fn code_instructions_new_test() {
        let input = vec![Instruction::Up, Instruction::Down];
        let expected = CodeInstructions(input.clone());

        assert_eq!(expected, CodeInstructions::new(input));
    }

    #[test]
    fn code_instructions_solve_code_number_test_in_bounds() {
        let mut button = normal_keypad::KeypadButton::at_start();
        let code_instructions = CodeInstructions(vec![
            Instruction::Up,
            Instruction::Down,
            Instruction::Right,
            Instruction::Left,
        ]);

        let returned = code_instructions.solve_code_number(&mut button);

        assert_eq!(ButtonNumber('5'), returned);
        assert_eq!(normal_keypad::KeypadButton::at_start(), button);
    }

    #[test]
    fn code_instructions_solve_code_number_test_out_of_bounds() {
        let mut button = normal_keypad::KeypadButton::at_start();
        let code_instructions = CodeInstructions(vec![
            Instruction::Up,
            Instruction::Up,
            Instruction::Left,
            Instruction::Left,
        ]);

        let returned = code_instructions.solve_code_number(&mut button);

        assert_eq!(ButtonNumber('1'), returned);
        assert_eq!(normal_keypad::KeypadButton::at_location(-1, 1), button);
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
    #[case(ButtonLocation::at(0, 1), Instruction::Up)]
    #[case(ButtonLocation::at(0, -1), Instruction::Down)]
    #[case(ButtonLocation::at(1, 0), Instruction::Right)]
    #[case(ButtonLocation::at(-1, 0), Instruction::Left)]
    fn instruction_button_position_offset_test(
        #[case] expected: ButtonLocation,
        #[case] instruction: Instruction,
    ) {
        assert_eq!(expected, instruction.button_position_offset());
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
