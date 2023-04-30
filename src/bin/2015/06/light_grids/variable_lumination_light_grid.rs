use super::{GRID_DIMENSION_SIZE, GRID_SIZE};
use crate::instruction::{Command, Coordinate, CoordinatePair, Instruction};

pub struct VariableLuminationLightGrid([u8; GRID_SIZE]);

impl VariableLuminationLightGrid {
    pub fn turned_off() -> Self {
        Self([0; GRID_SIZE])
    }

    fn get_mut(&mut self, coordinate: &Coordinate) -> Option<&mut u8> {
        let index = coordinate.y as usize * GRID_DIMENSION_SIZE + coordinate.x as usize;
        self.0.get_mut(index)
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
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

    pub fn count_turned_on_lights(&self) -> usize {
        self.0.iter().fold(0usize, |acc, e| acc + *e as usize)
    }
}
