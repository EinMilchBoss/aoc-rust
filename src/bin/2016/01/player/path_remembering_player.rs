use super::START;
use crate::{
    direction::Direction,
    instruction::{Instruction, Steps},
    point::Point2D,
};

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct PathRememberingPlayer {
    position: Point2D,
    direction: Direction,
    visited_positions: HashSet<Point2D>,
}

impl PathRememberingPlayer {
    pub fn at_start() -> Self {
        Self {
            position: Point2D::from_cartesian(0, 0),
            direction: Direction::North,
            visited_positions: HashSet::new(),
        }
    }

    pub fn distance_from_start(&self) -> usize {
        self.position.manhattan_distance_to(START)
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Left(steps) => {
                self.direction.turn_left();
                self.walk(*steps);
            }
            Instruction::Right(steps) => {
                self.direction.turn_right();
                self.walk(*steps);
            }
        };
    }

    fn walk(&mut self, steps: Steps) {
        let steps = steps.0 as isize;
        match self.direction {
            Direction::North => self.position.y += steps,
            Direction::East => self.position.x += steps,
            Direction::South => self.position.y -= steps,
            Direction::West => self.position.x -= steps,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_remembering_player() {}
}
