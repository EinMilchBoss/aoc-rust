use crate::coordinate::*;

#[derive(Clone, Copy, Debug)]
pub struct Knot {
    pub pos: Point,
}

impl Knot {
    pub fn at_start() -> Self {
        Knot { pos: Point(0, 0) }
    }

    pub fn shift(&mut self, shift: &Vector) {
        self.pos.0 += shift.0;
        self.pos.1 += shift.1;
    }

    pub fn follow(&mut self, other: &Knot) {
        let dif = other.pos - self.pos;
        if !Self::is_adjacent(&dif) {
            let approximation = dif.approximation();
            self.pos = self.pos + approximation;
        }
    }

    fn is_adjacent(dif: &Vector) -> bool {
        let adjacent_range = -1..=1;
        adjacent_range.contains(&dif.0) && adjacent_range.contains(&dif.1)
    }
}
