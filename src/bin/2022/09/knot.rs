use crate::coordinate::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Knot {
    pub pos: Point,
}

impl Knot {
    pub fn at_start() -> Self {
        Knot { pos: Point(0, 0) }
    }

    pub fn follow(&mut self, other: &Knot) {
        let dif = other.pos - self.pos;
        if !Self::is_adjacent(&dif) {
            let approximation = dif.approximation();
            self.pos += approximation;
        }
    }

    fn is_adjacent(dif: &Vector) -> bool {
        let adjacent_range = -1..=1;
        adjacent_range.contains(&dif.0) && adjacent_range.contains(&dif.1)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn knot_at_start() {
        assert_eq!(Knot { pos: Point(0, 0) }, Knot::at_start());
    }

    #[rstest]
    #[case(Point(0, 0), helper::knot_follow(Point(0, 0), Point(0, 0)))]
    #[case(Point(0, 0), helper::knot_follow(Point(0, 0), Point(1, 1)))]
    #[case(Point(0, 0), helper::knot_follow(Point(0, 0), Point(-1, -1)))]
    #[case(Point(0, 0), helper::knot_follow(Point(0, 0), Point(-1, 0)))]
    #[case(Point(0, 0), helper::knot_follow(Point(0, 0), Point(0, 1)))]
    #[case(Point(1, 1), helper::knot_follow(Point(0, 0), Point(1, 2)))]
    #[case(Point(-1, -1), helper::knot_follow(Point(0, 0), Point(-2, -1)))]
    fn knot_follow(#[case] expected: Point, #[case] actual: Point) {
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(Knot::is_adjacent(&Vector(1, 1)))]
    #[case(Knot::is_adjacent(&Vector(-1, -1)))]
    #[case(Knot::is_adjacent(&Vector(1, 0)))]
    #[case(Knot::is_adjacent(&Vector(0, -1)))]
    #[case(!Knot::is_adjacent(&Vector(2, 2)))]
    #[case(!Knot::is_adjacent(&Vector(-2, -2)))]
    #[case(!Knot::is_adjacent(&Vector(2, 0)))]
    #[case(!Knot::is_adjacent(&Vector(0, -2)))]
    fn knot_is_adjacent(#[case] actual: bool) {
        assert_eq!(true, actual);
    }

    mod helper {
        use super::*;

        pub fn knot_follow(follower_pos: Point, followee_pos: Point) -> Point {
            let mut follower = Knot { pos: follower_pos };
            let followee = Knot { pos: followee_pos };
            follower.follow(&followee);
            follower.pos
        }
    }
}
