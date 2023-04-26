use crate::{
    instruction::{Command, Coordinate, CoordinatePair, Instruction},
    light_grids::{GRID_DIMENSION_SIZE, GRID_SIZE},
};

pub struct FixedLightGrid([bool; GRID_SIZE]);

impl FixedLightGrid {
    pub fn turned_off() -> Self {
        Self([false; GRID_SIZE])
    }

    fn get_mut(&mut self, coordinate: &Coordinate) -> &mut bool {
        let index = coordinate.y as usize * GRID_DIMENSION_SIZE + coordinate.x as usize;
        self.0
            .get_mut(index)
            .unwrap_or_else(|| panic!("Coordinate `{:?}` is out of bounds.", coordinate))
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        let CoordinatePair(from_coordinate, to_coordinate) = instruction.coordinate_pair;
        for y in from_coordinate.y..=to_coordinate.y {
            for x in from_coordinate.x..=to_coordinate.x {
                let current_coordinate = Coordinate { x, y };
                let light = self.get_mut(&current_coordinate);

                match instruction.command {
                    Command::On => *light = true,
                    Command::Off => *light = false,
                    Command::Toggle => *light = !*light,
                }
            }
        }
    }

    pub fn count_turned_on_lights(&self) -> usize {
        self.0.iter().fold(0usize, |acc, e| acc + *e as usize)
    }
}
