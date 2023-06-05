use nom::error::{convert_error, VerboseError};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(display("Could not parse instruction. Further information: {}", self.description))]
pub struct InstructionParseError {
    description: String,
}

impl InstructionParseError {
    pub fn new(input: &str, error: VerboseError<&str>) -> Self {
        Self {
            description: convert_error(input, error),
        }
    }
}
