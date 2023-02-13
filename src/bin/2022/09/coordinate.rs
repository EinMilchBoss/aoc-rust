use std::ops::{AddAssign, Sub};

use crate::direction::Direction;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Point(pub i16, pub i16);

#[derive(Copy, Clone, Debug)]
pub struct Vector(pub i16, pub i16);

impl Vector {
    pub fn from_dir(dir: Direction) -> Vector {
        match dir {
            Direction::Up => Vector(0, 1),
            Direction::Down => Vector(0, -1),
            Direction::Right => Vector(1, 0),
            Direction::Left => Vector(-1, 0),
        }
    }

    pub fn approximation(&self) -> Vector {
        Vector(Self::approximate(self.0), Self::approximate(self.1))
    }

    fn approximate(delta: i16) -> i16 {
        delta.checked_div(delta.abs()).unwrap_or(0)
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}
