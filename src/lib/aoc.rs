use std::{fmt::Display, fs, io};

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
}

fn read_file(year: u16, day: u16, filename: &str) -> Result<String, ReadInputsError> {
    let file = format!("./res/{year:0>4}/{day:0>2}/{filename}");
    fs::read_to_string(&file).map_err(|source| ReadInputsError { source, file })
}

pub struct Part<'a, T> {
    number: u8,
    inputs: &'a Inputs,
    solve: fn(&str) -> T,
}

impl<'a, T> Part<'a, T>
where
    T: PartialEq + Display,
{
    pub fn one(inputs: &'a Inputs, solve: fn(&str) -> T) -> Self {
        Self {
            number: 1,
            inputs,
            solve,
        }
    }

    pub fn two(inputs: &'a Inputs, solve: fn(&str) -> T) -> Self {
        Self {
            number: 2,
            inputs,
            solve,
        }
    }

    pub fn test_protocol(&self, expected: T) -> String {
        let actual = (self.solve)(&self.inputs.example);
        let conclusion = if actual == expected { "PASS" } else { "FAIL" };

        let lines = [
            format!("--- PART {number} ---", number = self.number),
            format!("Expected:\n{expected}"),
            format!("Actual:\n{actual}"),
            format!("---- {conclusion} ----"),
        ];

        lines.join("\n")
    }

    pub fn run(&self) -> T {
        (self.solve)(&self.inputs.actual)
    }
}
