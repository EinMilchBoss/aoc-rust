#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_left(&mut self) {
        *self = match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        };
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_turn_left_test() {
        let mut direction = Direction::North;

        direction.turn_left();
        assert_eq!(Direction::West, direction);

        direction.turn_left();
        assert_eq!(Direction::South, direction);

        direction.turn_left();
        assert_eq!(Direction::East, direction);

        direction.turn_left();
        assert_eq!(Direction::North, direction);
    }

    #[test]
    fn direction_turn_right_test() {
        let mut direction = Direction::North;

        direction.turn_right();
        assert_eq!(Direction::East, direction);

        direction.turn_right();
        assert_eq!(Direction::South, direction);

        direction.turn_right();
        assert_eq!(Direction::West, direction);

        direction.turn_right();
        assert_eq!(Direction::North, direction);
    }
}
