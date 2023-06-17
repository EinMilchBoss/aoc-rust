mod error;
mod instruction;

use snafu::prelude::*;

use std::{ops::Deref, str::FromStr};

use self::error::AssembunnyParseSnafu;
pub use self::{error::AssembunnyParseError, instruction::*};

#[derive(Debug, Clone, PartialEq)]
pub struct Assembunny(pub Vec<Instruction>);

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

#[cfg(test)]
mod assembunny_tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assembunny::instruction::{Argument, InstructionParseError, RegisterId};

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
