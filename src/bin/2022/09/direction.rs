use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Ok(Direction::Up), Direction::from_str("U"))]
    #[case(Ok(Direction::Down), Direction::from_str("D"))]
    #[case(Ok(Direction::Right), Direction::from_str("R"))]
    #[case(Ok(Direction::Left), Direction::from_str("L"))]
    #[case(Err(()), Direction::from_str("X"))]
    fn direction_from_str(
        #[case] expected: Result<Direction, ()>,
        #[case] actual: Result<Direction, ()>,
    ) {
        assert_eq!(expected, actual);
    }
}
