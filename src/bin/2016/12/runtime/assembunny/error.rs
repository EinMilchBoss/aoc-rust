use snafu::prelude::*;

use super::instruction::InstructionParseError;

#[derive(Debug, Clone, PartialEq, Snafu)]
#[snafu(display("Could not parse assembunny."), visibility(pub))]
pub struct AssembunnyParseError {
    source: InstructionParseError,
}

impl AssembunnyParseError {
    #[cfg(test)]
    pub fn initialize(source: InstructionParseError) -> Self {
        Self { source }
    }

    pub fn verbose_error_description(&self) -> &str {
        self.source.verbose_error_description()
    }
}
