use std::ops::{AddAssign, Sub};

use crate::direction::Direction;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Point(pub i16, pub i16);

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Vector(0, 1), Vector::from_dir(Direction::Up))]
    #[case(Vector(0, -1), Vector::from_dir(Direction::Down))]
    #[case(Vector(1, 0), Vector::from_dir(Direction::Right))]
    #[case(Vector(-1, 0), Vector::from_dir(Direction::Left))]
    fn vector_from_dir(#[case] expected: Vector, #[case] actual: Vector) {
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(Vector(0, 1), Vector(0, 15).approximation())]
    #[case(Vector(-1, 0), Vector(-15, 0).approximation())]
    #[case(Vector(0, -1), Vector(0, -1).approximation())]
    #[case(Vector(1, 0), Vector(1, 0).approximation())]
    #[case(Vector(0, 0), Vector(0, 0).approximation())]
    #[case(Vector(-1, 1), Vector(-15, 15).approximation())]
    fn vector_approximation(#[case] expected: Vector, #[case] actual: Vector) {
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(Point(0, 0), helper::point_add_assign(Point(0, 0), Vector(0, 0)))]
    #[case(Point(5, 5), helper::point_add_assign(Point(0, 0), Vector(5, 5)))]
    #[case(Point(5, 5), helper::point_add_assign(Point(5, 5), Vector(0, 0)))]
    #[case(Point(0, 0), helper::point_add_assign(Point(-5, -5), Vector(5, 5)))]
    fn point_add_assign(#[case] expected: Point, #[case] actual: Point) {
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(Vector(0, 0), Point(0, 0) - Point(0, 0))]
    #[case(Vector(-5, 0), Point(0, 0) - Point(5, 0))]
    #[case(Vector(0, -5), Point(0, 0) - Point(0, 5))]
    #[case(Vector(-5, -5), Point(0, 0) - Point(5, 5))]
    #[case(Vector(5, 5), Point(0, 0) - Point(-5, -5))]
    #[case(Vector(0, 0), Point(-5, -5) - Point(-5, -5))]
    fn point_sub(#[case] expected: Vector, #[case] actual: Vector) {
        assert_eq!(expected, actual);
    }

    mod helper {
        use super::*;

        pub fn point_add_assign(point: Point, vector: Vector) -> Point {
            let mut result = point;
            result.add_assign(vector);
            result
        }
    }
}
