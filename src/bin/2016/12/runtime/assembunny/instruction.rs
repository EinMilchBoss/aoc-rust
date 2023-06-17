mod argument;
mod parsing;
mod register_id;

use std::str::FromStr;

use nom::Finish;

pub use self::{argument::Argument, parsing::InstructionParseError, register_id::RegisterId};

pub type Word = i32;

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

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        parsing::parse_instruction(string)
            .finish()
            .map(|(_, output)| output)
            .map_err(|error| InstructionParseError::with_parse_context(string, error))
    }
}
