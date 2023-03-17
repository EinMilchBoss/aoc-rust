use std::fmt::Display;

use super::*;

static NEIGHBOR_OFFSETS: [Vector; 4] = [
    Vector { dx: 0, dy: 1 },
    Vector { dx: 0, dy: -1 },
    Vector { dx: 1, dy: 0 },
    Vector { dx: -1, dy: 0 },
];

pub struct Grid {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let width = input
            .lines()
            .next()
            .expect("First line does not have at least one byte of data.")
            .len();
        let height = input.lines().count();

        let tiles = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.bytes().enumerate().map(move |(x, byte)| match byte {
                    START => Tile::Start(Coordinate { x, y }),
                    END => Tile::End(Coordinate { x, y }),
                    LOWEST..=HIGHEST => Tile::Normal(Elevation(byte - LOWEST)),
                    _ => panic!(
                        "Encountered illegal byte {} while parsing (char: {}).",
                        byte, byte as char
                    ),
                })
            })
            .collect();

        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn start(&self) -> &Coordinate {
        self.tiles
            .iter()
            .find_map(|tile| match tile {
                Tile::Start(coordinate) => Some(coordinate),
                _ => None,
            })
            .expect("Start was not found in grid.")
    }

    pub fn end(&self) -> &Coordinate {
        self.tiles
            .iter()
            .find_map(|tile| match tile {
                Tile::End(coordinate) => Some(coordinate),
                _ => None,
            })
            .expect("End was not found in grid.")
    }

    pub fn tile(&self, coordinate: &Coordinate) -> Option<&Tile> {
        if !self.is_in_bounds(coordinate) {
            return None;
        }
        self.tiles.get(coordinate.y * self.width + coordinate.x)
    }

    pub fn tiles<F>(&self, mut f: F) -> Vec<Coordinate>
    where
        F: FnMut(&Tile, Coordinate) -> Option<Coordinate>,
    {
        (0..self.height)
            .flat_map(|y| {
                (0..self.width).map(move |x| {
                    let coordinate = Coordinate { x, y };
                    let tile = self.tile(&coordinate).unwrap_or_else(|| {
                        panic!("Coordinate {:?} could not be found in grid.", coordinate)
                    });
                    (tile, coordinate)
                })
            })
            .filter_map(|(tile, coordinate)| f(tile, coordinate))
            .collect()
    }

    pub fn valid_neighbors(&self, from: &Coordinate) -> Vec<Coordinate> {
        let from_tile = self
            .tile(from)
            .unwrap_or_else(|| panic!("Current tile at coordinate `{:?}` cannot be found.", from));

        NEIGHBOR_OFFSETS
            .iter()
            .filter_map(|offset| {
                let to = from.shift(offset)?;
                let to_tile = self.tile(&to)?;
                match from_tile.is_step_possible(to_tile) {
                    true => Some(to),
                    false => None,
                }
            })
            .collect()
    }

    fn is_in_bounds(&self, coordinate: &Coordinate) -> bool {
        (0..self.width).contains(&coordinate.x) && (0..self.height).contains(&coordinate.y)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            write!(f, "|")?;
            for x in 0..self.width {
                let tile = self.tiles.get(y * self.width + x).ok_or(std::fmt::Error)?;
                let elevation = match tile {
                    Tile::Normal(Elevation(elevation)) => format!("{:0>2}", elevation),
                    Tile::Start(_) => "SS".to_string(),
                    Tile::End(_) => "EE".to_string(),
                };
                write!(f, "{}|", elevation)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
