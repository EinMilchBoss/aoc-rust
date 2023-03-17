use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Elevation(pub u8);

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Normal(Elevation),
    Start(Coordinate),
    End(Coordinate),
}

impl Tile {
    pub fn elevation(&self) -> Elevation {
        Elevation(match self {
            Tile::Normal(Elevation(elevation)) => *elevation,
            Tile::Start(_) => 0,
            Tile::End(_) => HIGHEST - LOWEST,
        })
    }

    pub fn is_step_possible(&self, to: &Tile) -> bool {
        let Elevation(from) = self.elevation();
        let Elevation(to) = to.elevation();
        !matches!(to.checked_sub(from), Some(difference) if difference > 1)
    }
}
