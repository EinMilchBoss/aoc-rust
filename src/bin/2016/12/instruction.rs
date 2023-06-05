mod error;
mod parser;

use std::str::FromStr;

use nom::Finish;

pub type Word = i32;
pub type RegisterId = char;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Cpy {
        from: Argument,
        into: Argument,
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

impl FromStr for Instruction {
    type Err = error::InstructionParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        parser::parse_instruction(string)
            .finish()
            .map(|(_, output)| output)
            .map_err(|error| error::InstructionParseError::new(string, error))
    }
}
