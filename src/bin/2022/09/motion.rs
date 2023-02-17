use std::str::FromStr;

use crate::direction::Direction;

#[derive(PartialEq, Debug)]
pub struct Motion {
    pub dir: Direction,
    pub count: u8,
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let [dir, count] = string.split(' ').collect::<Vec<_>>().as_slice() {
            let dir = Direction::from_str(dir)?;
            let count: u8 = count.parse().map_err(|_| ())?;
            Ok(Motion { dir, count })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Ok(Motion {dir: Direction::Up, count: 1 }), Motion::from_str("U 1"))]
    #[case(Ok(Motion {dir: Direction::Down, count: 2 }), Motion::from_str("D 2"))]
    #[case(Ok(Motion {dir: Direction::Right, count: 3 }), Motion::from_str("R 3"))]
    #[case(Ok(Motion {dir: Direction::Left, count: 4 }), Motion::from_str("L 4"))]
    #[case(Err(()), Motion::from_str("X 5"))]
    #[case(Err(()), Motion::from_str("U -5"))]
    #[case(Err(()), Motion::from_str("U"))]
    #[case(Err(()), Motion::from_str("5"))]
    #[case(Err(()), Motion::from_str(""))]
    fn motion_from_str(#[case] expected: Result<Motion, ()>, #[case] actual: Result<Motion, ()>) {
        assert_eq!(expected, actual);
    }
}
