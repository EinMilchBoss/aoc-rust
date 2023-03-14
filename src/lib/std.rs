use std::{error::Error, fmt::Display, fs};

pub struct Day<'a>(pub &'a str);
pub struct Year<'a>(pub &'a str);

pub enum InputFile {
    Example,
    Actual,
}

pub fn read_file(file: InputFile, year: Year, day: Day) -> Option<String> {
    let mut path = format!("./res/{}/{}/", year.0, day.0);
    let path_suffix = match file {
        InputFile::Example => "example.txt",
        InputFile::Actual => "input.txt",
    };
    path.push_str(path_suffix);
    match fs::read_to_string(&path) {
        Ok(content) => Some(content),
        Err(error) => {
            println!("File {} couldn't be read ({}).", &path, error.kind());
            None
        }
    }
}

#[derive(Debug)]
pub struct AocError;

impl Error for AocError {}

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An AocError occurred.")
    }
}
