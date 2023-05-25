#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl Point2D {
    pub const fn from_cartesian(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance_to(&self, other: Point2D) -> usize {
        let dx = (self.x - other.x).unsigned_abs();
        let dy = (self.y - other.y).unsigned_abs();
        dx + dy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_from_cartesian_test() {
        let point = Point2D::from_cartesian(5, -10);

        assert_eq!(5, point.x);
        assert_eq!(-10, point.y);
    }

    #[test]
    fn manhattan_distance_test_neg2pos_neg2neg() {
        let start_point = Point2D::from_cartesian(-12, -2);
        let end_point = Point2D::from_cartesian(5, -10);

        assert_eq!(25, start_point.manhattan_distance_to(end_point));
    }

    #[test]
    fn manhattan_distance_test_pos2neg_pos2pos() {
        let start_point = Point2D::from_cartesian(12, 2);
        let end_point = Point2D::from_cartesian(-5, 10);

        assert_eq!(25, start_point.manhattan_distance_to(end_point));
    }
}
