use std::str::FromStr;

use crate::direction::Direction;

pub struct Motion {
    pub dir: Direction,
    pub count: u8,
}

impl Motion {
    pub fn new(dir: Direction, count: u8) -> Self {
        Motion { dir, count }
    }
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let [dir, count] = string.split(' ').collect::<Vec<_>>().as_slice() {
            let dir = Direction::from_str(dir).unwrap();
            let count = count.parse().unwrap();
            Ok(Motion::new(dir, count))
        } else {
            Err(())
        }
    }
}
