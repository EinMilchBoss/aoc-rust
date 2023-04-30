mod instruction;
mod light_grids;

use light_grids::{FixedLuminationLightGrid, VariableLuminationLightGrid};
use util::std::*;

const YEAR: Year = Year("2015");
const DAY: Day = Day("06");

fn solve_first(input: &str) -> String {
    let instructions = instruction::parse_instructions(input).expect("Parsing failed.");
    let mut grid = FixedLuminationLightGrid::turned_off();
    for instruction in instructions {
        grid.execute_instruction(instruction);
    }
    grid.count_turned_on_lights().to_string()
}

fn solve_second(input: &str) -> String {
    let instructions = instruction::parse_instructions(input).expect("Parsing failed.");
    let mut grid = VariableLuminationLightGrid::turned_off();
    for instruction in instructions {
        grid.execute_instruction(instruction);
    }
    grid.count_turned_on_lights().to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!(
            "First: Expected {} found {}.",
            light_grids::GRID_SIZE - light_grids::GRID_DIMENSION_SIZE - 4,
            solve_first(&example)
        );
        println!(
            "Second: Expected {} found {}.",
            light_grids::GRID_SIZE + 2 * light_grids::GRID_DIMENSION_SIZE - 4,
            solve_second(&example)
        );
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
