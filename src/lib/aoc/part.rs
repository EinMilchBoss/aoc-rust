use std::fmt::Display;

use crate::aoc::Inputs;

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
        let actual = (self.solve)(self.inputs.example());
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
        (self.solve)(self.inputs.actual())
    }
}
