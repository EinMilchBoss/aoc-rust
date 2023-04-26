use util::std::*;

const YEAR: Year = Year("2015");
const DAY: Day = Day("06");

const GRID_DIMENSION_SIZE: usize = 1_000;
const GRID_SIZE: usize = GRID_DIMENSION_SIZE * GRID_DIMENSION_SIZE;

mod parse;

#[derive(Debug)]
struct Coordinate {
    x: u16,
    y: u16,
}

#[derive(Debug)]
pub struct CoordinatePair(Coordinate, Coordinate);

#[derive(Debug)]
pub enum Command {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
pub struct Instruction {
    pub command: Command,
    pub coordinate_pair: CoordinatePair,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value {
            "turn on" => Self::On,
            "turn off" => Self::Off,
            "toggle" => Self::Toggle,
            _ => panic!("String `{}` could not be parsed to a command.", value),
        }
    }
}

struct FixedLightGrid {
    lights: [bool; GRID_SIZE],
}

impl FixedLightGrid {
    fn get_mut(&mut self, coordinate: &Coordinate) -> Option<&mut bool> {
        let index = coordinate.y as usize * GRID_DIMENSION_SIZE + coordinate.x as usize;
        self.lights.get_mut(index)
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        let CoordinatePair(from_coordinate, to_coordinate) = instruction.coordinate_pair;
        for y in from_coordinate.y..=to_coordinate.y {
            for x in from_coordinate.x..=to_coordinate.x {
                let current_coordinate = Coordinate { x, y };
                let light = self.get_mut(&current_coordinate).unwrap_or_else(|| {
                    panic!(
                        "Coordinate `{:?}` was out of bounds for grid.",
                        current_coordinate
                    )
                });
                match instruction.command {
                    Command::On => *light = true,
                    Command::Off => *light = false,
                    Command::Toggle => *light = !*light,
                }
            }
        }
    }

    fn count_turned_on_lights(&self) -> usize {
        self.lights.iter().fold(0usize, |acc, e| acc + *e as usize)
    }
}

struct VariableLightGrid([u8; GRID_SIZE]);

impl VariableLightGrid {
    // operator overload for indexing ???
    fn get_mut(&mut self, coordinate: &Coordinate) -> Option<&mut u8> {
        let index = coordinate.y as usize * GRID_DIMENSION_SIZE + coordinate.x as usize;
        self.0.get_mut(index)
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        let CoordinatePair(from_coordinate, to_coordinate) = instruction.coordinate_pair;
        for y in from_coordinate.y..=to_coordinate.y {
            for x in from_coordinate.x..=to_coordinate.x {
                let current_coordinate = Coordinate { x, y };
                let light = self.get_mut(&current_coordinate).unwrap_or_else(|| {
                    panic!(
                        "Coordinate `{:?}` was out of bounds for grid.",
                        current_coordinate
                    )
                });
                match instruction.command {
                    Command::On => *light += 1,
                    Command::Off => *light = light.saturating_sub(1),
                    Command::Toggle => *light += 2,
                }
            }
        }
    }

    fn count_turned_on_lights(&self) -> usize {
        self.0.iter().fold(0usize, |acc, e| acc + *e as usize)
    }
}

fn solve_first(input: &str) -> String {
    let instructions = parse::parse(input);
    let mut grid = FixedLightGrid {
        lights: [false; GRID_SIZE],
    };
    instructions
        .into_iter()
        .for_each(|instruction| grid.execute_instruction(instruction));
    grid.count_turned_on_lights().to_string()
}

fn solve_second(input: &str) -> String {
    let instructions = parse::parse(input);
    // constructor "turned off"
    let mut grid = VariableLightGrid([0; GRID_SIZE]);
    instructions
        .into_iter()
        .for_each(|instruction| grid.execute_instruction(instruction));
    grid.count_turned_on_lights().to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!(
            "First: Expected {} found {}.",
            GRID_SIZE - GRID_DIMENSION_SIZE - 4,
            solve_first(&example)
        );
        println!(
            "Second: Expected {} found {}.",
            GRID_SIZE + 2 * GRID_DIMENSION_SIZE - 4,
            solve_second(&example)
        );
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
