use nom::error::{convert_error, VerboseError};
use snafu::prelude::*;

#[derive(Debug, Clone, PartialEq, Snafu)]
#[snafu(display("Could not parse assembunny."), visibility(pub))]
pub struct AssembunnyParseError {
    source: InstructionParseError,
}

#[derive(Debug, Clone, PartialEq, Snafu)]
#[snafu(display("Could not parse instruction"))]
pub struct InstructionParseError {
    verbose_error_description: String,
}

impl AssembunnyParseError {
    #[cfg(test)]
    pub fn initialize(source: InstructionParseError) -> Self {
        Self { source }
    }

    pub fn verbose_error_description(&self) -> &str {
        &self.source.verbose_error_description
    }
}

impl InstructionParseError {
    #[cfg(test)]
    pub fn initialize(verbose_error_description: &str) -> Self {
        Self {
            verbose_error_description: verbose_error_description.to_string(),
        }
    }

    pub fn with_parse_context(input: &str, error: VerboseError<&str>) -> Self {
        Self {
            verbose_error_description: convert_error(input, error),
        }
    }
}
