use crate::{
    direction::Direction,
    instruction::{Instruction, Steps},
    player,
    point::Point2D,
};

#[derive(Debug, PartialEq)]
pub struct PathFollowingPlayer {
    position: Point2D,
    direction: Direction,
}

impl PathFollowingPlayer {
    pub fn at_start() -> Self {
        Self {
            position: player::START,
            direction: Direction::North,
        }
    }

    pub fn distance_from_start(&self) -> usize {
        self.position.manhattan_distance_to(player::START)
    }

    pub fn follow_path(&mut self, instruction: &Instruction) {
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
    fn path_following_player_at_start_test() {
        let expected = PathFollowingPlayer {
            position: Point2D { x: 0, y: 0 },
            direction: Direction::North,
        };

        assert_eq!(expected, PathFollowingPlayer::at_start());
    }

    #[test]
    fn path_following_player_distance_from_start_test() {
        let mut player = PathFollowingPlayer::at_start();
        player.position = Point2D::from_cartesian(5, -10);

        assert_eq!(15, player.distance_from_start());
    }

    #[test]
    fn path_following_player_follow_path_test_left() {
        let mut player = PathFollowingPlayer::at_start();
        let instruction = Instruction::Left(Steps(10));
        let expected = PathFollowingPlayer {
            position: Point2D { x: -10, y: 0 },
            direction: Direction::West,
        };

        player.follow_path(&instruction);

        assert_eq!(expected, player);
    }

    #[test]
    fn path_following_player_follow_path_test_right() {
        let mut player = PathFollowingPlayer::at_start();
        let instruction = Instruction::Right(Steps(10));
        let expected = PathFollowingPlayer {
            position: Point2D { x: 10, y: 0 },
            direction: Direction::East,
        };

        player.follow_path(&instruction);

        assert_eq!(expected, player);
    }

    #[test]
    fn path_following_player_walk_test() {
        let mut player = PathFollowingPlayer::at_start();

        player.direction = Direction::North;
        player.walk(Steps(5));
        assert_eq!(Point2D::from_cartesian(0, 5), player.position);

        player.direction = Direction::East;
        player.walk(Steps(5));
        assert_eq!(Point2D::from_cartesian(5, 5), player.position);

        player.direction = Direction::South;
        player.walk(Steps(5));
        assert_eq!(Point2D::from_cartesian(5, 0), player.position);

        player.direction = Direction::West;
        player.walk(Steps(5));
        assert_eq!(Point2D::from_cartesian(0, 0), player.position);
    }
}
