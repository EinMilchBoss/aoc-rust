use crate::{
    direction::Direction,
    instruction::{Instruction, Steps},
    player,
    point::Point2D,
};

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct PathRememberingPlayer {
    position: Point2D,
    direction: Direction,
    visited_positions: HashSet<Point2D>,
    has_visited_position_twice: bool,
}

impl PathRememberingPlayer {
    pub fn at_start() -> Self {
        Self {
            position: player::START,
            direction: Direction::North,
            visited_positions: HashSet::from([player::START]),
            has_visited_position_twice: false,
        }
    }

    pub fn has_visited_position_twice(&self) -> bool {
        self.has_visited_position_twice
    }

    pub fn distance_from_start(&self) -> usize {
        player::distance_from_start(&self.position)
    }

    pub fn find_first_position_visited_twice(&mut self, instruction: &Instruction) {
        if self.has_visited_position_twice {
            return;
        }

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
        match self.direction {
            Direction::North => self.remember_visited_positions(steps, |position| position.y += 1),
            Direction::East => self.remember_visited_positions(steps, |position| position.x += 1),
            Direction::South => self.remember_visited_positions(steps, |position| position.y -= 1),
            Direction::West => self.remember_visited_positions(steps, |position| position.x -= 1),
        };
    }

    fn remember_visited_positions(&mut self, steps: Steps, step_update: impl Fn(&mut Point2D)) {
        for _ in 0..steps.0 {
            step_update(&mut self.position);
            let not_contained_yet = self.visited_positions.insert(self.position);
            if !not_contained_yet {
                self.has_visited_position_twice = true;
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn path_remembering_player_at_start_test() {
        let expected = PathRememberingPlayer {
            position: Point2D { x: 0, y: 0 },
            direction: Direction::North,
            visited_positions: HashSet::from([player::START]),
            has_visited_position_twice: false,
        };

        assert_eq!(expected, PathRememberingPlayer::at_start());
    }

    #[test]
    fn path_remembering_player_find_first_position_visited_twice_test_left() {
        let mut player = PathRememberingPlayer::at_start();
        let instruction = Instruction::Left(Steps(3));
        let expected = PathRememberingPlayer {
            position: Point2D { x: -3, y: 0 },
            direction: Direction::West,
            visited_positions: HashSet::from_iter([
                player::START,
                Point2D::from_cartesian(-1, 0),
                Point2D::from_cartesian(-2, 0),
                Point2D::from_cartesian(-3, 0),
            ]),
            has_visited_position_twice: false,
        };

        player.find_first_position_visited_twice(&instruction);

        assert_eq!(expected, player);
    }

    #[test]
    fn path_remembering_player_find_first_position_visited_twice_test_right() {
        let mut player = PathRememberingPlayer::at_start();
        let instruction = Instruction::Right(Steps(3));
        let expected = PathRememberingPlayer {
            position: Point2D { x: 3, y: 0 },
            direction: Direction::East,
            visited_positions: HashSet::from_iter([
                player::START,
                Point2D::from_cartesian(1, 0),
                Point2D::from_cartesian(2, 0),
                Point2D::from_cartesian(3, 0),
            ]),
            has_visited_position_twice: false,
        };

        player.find_first_position_visited_twice(&instruction);

        assert_eq!(expected, player);
    }

    #[test]
    fn path_remembering_player_find_first_position_visited_twice_test_after_duplicate_position() {
        let mut player = PathRememberingPlayer::at_start();
        player.has_visited_position_twice = true;
        let instruction = Instruction::Left(Steps(3));
        let expected = PathRememberingPlayer::at_start();

        player.find_first_position_visited_twice(&instruction);

        assert_eq!(expected.position, player.position);
        assert_eq!(expected.direction, player.direction);
        assert_eq!(expected.visited_positions, player.visited_positions);
        assert_eq!(true, player.has_visited_position_twice);
    }

    #[test]
    fn path_remembering_player_walk_test() {
        let mut player = PathRememberingPlayer::at_start();
        let mut expected_visited_positions = HashSet::from([player::START]);

        expected_visited_positions
            .extend([Point2D::from_cartesian(0, 1), Point2D::from_cartesian(0, 2)]);
        player.direction = Direction::North;
        player.walk(Steps(2));
        assert_eq!(Point2D::from_cartesian(0, 2), player.position);
        assert_eq!(expected_visited_positions, player.visited_positions);

        expected_visited_positions
            .extend([Point2D::from_cartesian(1, 2), Point2D::from_cartesian(2, 2)]);
        player.direction = Direction::East;
        player.walk(Steps(2));
        assert_eq!(Point2D::from_cartesian(2, 2), player.position);
        assert_eq!(expected_visited_positions, player.visited_positions);

        expected_visited_positions
            .extend([Point2D::from_cartesian(2, 1), Point2D::from_cartesian(2, 0)]);
        player.direction = Direction::South;
        player.walk(Steps(2));
        assert_eq!(Point2D::from_cartesian(2, 0), player.position);
        assert_eq!(expected_visited_positions, player.visited_positions);

        expected_visited_positions
            .extend([Point2D::from_cartesian(1, 0), Point2D::from_cartesian(0, 0)]);
        player.direction = Direction::West;
        player.walk(Steps(2));
        assert_eq!(Point2D::from_cartesian(0, 0), player.position);
        assert_eq!(expected_visited_positions, player.visited_positions);
    }

    #[test]
    fn path_remembering_player_remember_visited_positions_test_no_duplicate_position() {
        let mut player = PathRememberingPlayer::at_start();
        let expected = PathRememberingPlayer {
            position: Point2D { x: 0, y: 2 },
            direction: Direction::North,
            visited_positions: HashSet::from([
                player::START,
                Point2D::from_cartesian(0, 1),
                Point2D::from_cartesian(0, 2),
            ]),
            has_visited_position_twice: false,
        };

        player.remember_visited_positions(Steps(2), |player_position| player_position.y += 1);

        assert_eq!(expected, player);
    }

    #[test]
    fn path_remembering_player_remember_visited_positions_test_duplicate_position() {
        let visited_positions = HashSet::from([
            player::START,
            Point2D::from_cartesian(0, 1),
            Point2D::from_cartesian(0, 2),
            Point2D::from_cartesian(0, 3),
        ]);
        let mut player = PathRememberingPlayer::at_start();
        player.position = Point2D::from_cartesian(0, 3);
        player.visited_positions.extend(&visited_positions);

        player.remember_visited_positions(Steps(10), |player_position| player_position.y -= 1);

        assert_eq!(Point2D::from_cartesian(0, 2), player.position);
        assert_eq!(true, player.has_visited_position_twice);
    }
}
