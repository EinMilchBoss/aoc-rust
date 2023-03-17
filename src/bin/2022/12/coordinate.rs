pub struct Vector {
    pub dx: isize,
    pub dy: isize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn shift(&self, vec: &Vector) -> Option<Coordinate> {
        Some(Coordinate {
            x: self.x.checked_add_signed(vec.dx)?,
            y: self.y.checked_add_signed(vec.dy)?,
        })
    }
}
