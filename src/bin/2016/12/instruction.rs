mod error;
mod parser;

pub use error::{AssembunnyParseError, InstructionParseError};

use std::{ops::Deref, str::FromStr};

use nom::Finish;
use snafu::prelude::*;

use self::error::AssembunnyParseSnafu;

pub type Word = i32;

#[derive(Debug, Clone, PartialEq)]
pub struct Assembunny(pub Vec<Instruction>);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Cpy {
        from: Argument,
        into: RegisterId,
    },
    Inc(RegisterId),
    Dec(RegisterId),
    Jnz {
        condition: Argument,
        jump_offset: Word,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Argument {
    Literal(Word),
    Reference(RegisterId),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterId {
    A,
    B,
    C,
    D,
}

impl Deref for Assembunny {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Assembunny {
    type Err = AssembunnyParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let parsed_instructions = string
            .lines()
            .map(|line| line.parse().context(AssembunnyParseSnafu {}))
            .collect::<Result<_, _>>();

        match parsed_instructions {
            Ok(instructions) => Ok(Self(instructions)),
            Err(error) => Err(error),
        }
    }
}

impl FromStr for Instruction {
    type Err = error::InstructionParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        parser::parse_instruction(string)
            .finish()
            .map(|(_, output)| output)
            .map_err(|error| InstructionParseError::with_parse_context(string, error))
    }
}

impl From<char> for RegisterId {
    fn from(value: char) -> Self {
        match value {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            _ => panic!("Cannot parse `{}` to `RegisterId`.", value),
        }
    }
}

#[cfg(test)]
mod assembunny_tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn from_str_trait_from_str_test_ok() {
        let input = ["cpy 7 c", "inc d", "dec c", "jnz c -2"].join("\n");
        let expected = Ok(Assembunny(vec![
            Instruction::Cpy {
                from: Argument::Literal(7),
                into: RegisterId::C,
            },
            Instruction::Inc(RegisterId::D),
            Instruction::Dec(RegisterId::C),
            Instruction::Jnz {
                condition: Argument::Reference(RegisterId::C),
                jump_offset: -2,
            },
        ]));

        assert_eq!(expected, input.parse());
    }

    #[test]
    fn from_str_trait_from_str_test_err() {
        let input = ["cpy 7 c", "inc d", "dec 5", "jnz c -2"].join("\n");
        let verbose_error_description = [
            "0: at line 1, in Tag:",
            "dec 5",
            "^\n",
            "1: at line 1, in Alt:",
            "dec 5",
            "^",
            "\n",
        ]
        .join("\n");
        let expected: Result<Assembunny, _> = Err(AssembunnyParseError::initialize(
            InstructionParseError::initialize(&verbose_error_description),
        ));

        assert_eq!(expected, input.parse());
    }
}
