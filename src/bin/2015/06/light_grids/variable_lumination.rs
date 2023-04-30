use super::{GRID_DIMENSION_SIZE, GRID_SIZE};
use crate::instruction::{Command, Coordinate, Instruction};

pub struct VariableLuminationLightGrid([u8; GRID_SIZE]);

impl VariableLuminationLightGrid {
    pub fn turned_off() -> Self {
        Self([0; GRID_SIZE])
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        for current_coordinate in &instruction.coordinate_pair.area() {
            let light = self.get_mut(current_coordinate);
            match instruction.command {
                Command::On => *light += 1,
                Command::Off => *light = light.saturating_sub(1),
                Command::Toggle => *light += 2,
            }
        }
    }

    pub fn count_turned_on_lights(&self) -> usize {
        self.0.iter().fold(0usize, |acc, e| acc + *e as usize)
    }

    fn get_mut(&mut self, coordinate: &Coordinate) -> &mut u8 {
        let index = coordinate.y as usize * GRID_DIMENSION_SIZE + coordinate.x as usize;
        self.0
            .get_mut(index)
            .unwrap_or_else(|| panic!("Coordinate `{:?}` is out of bounds.", coordinate))
    }
}
