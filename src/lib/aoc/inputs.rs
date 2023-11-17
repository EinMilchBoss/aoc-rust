use std::{fs, io};

const EXAMPLE_FILENAME: &str = "example.txt";
const ACTUAL_FILENAME: &str = "actual.txt";

#[derive(Debug, thiserror::Error)]
#[error("File `{file}` could not be read.")]
pub struct ReadInputsError {
    #[source]
    pub source: io::Error,
    pub file: String,
}

pub struct Inputs {
    example: String,
    actual: String,
}

impl Inputs {
    pub fn read(year: u16, day: u16) -> Result<Self, ReadInputsError> {
        let example = read_file(year, day, EXAMPLE_FILENAME)?;
        let input = read_file(year, day, ACTUAL_FILENAME)?;
        Ok(Self {
            example,
            actual: input,
        })
    }

    pub fn example(&self) -> &str {
        &self.example
    }

    pub fn actual(&self) -> &str {
        &self.actual
    }
}

fn read_file(year: u16, day: u16, filename: &str) -> Result<String, ReadInputsError> {
    let file = format!("./res/{year:0>4}/{day:0>2}/{filename}");
    fs::read_to_string(&file).map_err(|source| ReadInputsError { source, file })
}
