use std::{collections::HashSet, fmt::Display};

use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("12");

const START: u8 = b'S';
const END: u8 = b'E';
const LOWEST: u8 = b'a';
const HIGHEST: u8 = b'z';

fn _print_movement(
    grid: &Grid,
    next: &HashSet<Coordinate>,
    visited: &HashSet<Coordinate>,
) -> String {
    let mut buffer = String::new();

    for y in 0..grid.height {
        buffer.push('|');
        for x in 0..grid.width {
            let current = Coordinate { x, y };
            if next.contains(&current) {
                buffer.push_str("*|");
            } else if visited.contains(&current) {
                buffer.push_str("#|");
            } else {
                buffer.push_str(".|");
            }
        }
        buffer.push('\n');
    }

    buffer
}

#[derive(Clone, Copy, Debug)]
struct Elevation(u8);

#[derive(Clone, Copy, Debug)]
enum Tile {
    Normal(Elevation),
    Start(Coordinate),
    End(Coordinate),
}

impl Tile {
    fn elevation(&self) -> Elevation {
        match self {
            Tile::Normal(elevation) => *elevation,
            Tile::Start(_) => Elevation(0),
            Tile::End(_) => Elevation(HIGHEST - LOWEST),
        }
    }

    fn is_step_possible(&self, to: &Tile) -> bool {
        let Elevation(from) = self.elevation();
        let Elevation(to) = to.elevation();
        !matches!(to.checked_sub(from), Some(difference) if difference > 1)
    }
}

struct Distance(usize);

struct Vector {
    dx: isize,
    dy: isize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn shift(&self, vec: &Vector) -> Option<Coordinate> {
        Some(Coordinate {
            x: self.x.checked_add_signed(vec.dx)?,
            y: self.y.checked_add_signed(vec.dy)?,
        })
    }
}

struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let width = input
            .lines()
            .take(1)
            .next()
            .expect("First line does not have at least one byte of data.")
            .len();
        let height = input.lines().count();

        let mut tiles = Vec::new();
        for (y, line) in input.lines().enumerate() {
            for (x, byte) in line.bytes().enumerate() {
                tiles.push(match byte {
                    START => Tile::Start(Coordinate { x, y }),
                    END => Tile::End(Coordinate { x, y }),
                    b'a'..=b'z' => Tile::Normal(Elevation(byte - b'a')),
                    _ => panic!(
                        "Encountered illegal byte {} while parsing (char: {}).",
                        byte, byte as char
                    ),
                });
            }
        }

        Self {
            tiles,
            width,
            height,
        }
    }

    fn start(&self) -> &Coordinate {
        self.tiles
            .iter()
            .find_map(|tile| {
                if let Tile::Start(coordinate) = tile {
                    Some(coordinate)
                } else {
                    None
                }
            })
            .expect("Start was not found in grid.")
    }

    fn end(&self) -> &Coordinate {
        self.tiles
            .iter()
            .find_map(|tile| {
                if let Tile::End(coordinate) = tile {
                    Some(coordinate)
                } else {
                    None
                }
            })
            .expect("End was not found in grid.")
    }

    fn is_in_bounds(&self, coordinate: &Coordinate) -> bool {
        (0..self.width).contains(&coordinate.x) && (0..self.height).contains(&coordinate.y)
    }

    fn tile(&self, coordinate: &Coordinate) -> Option<&Tile> {
        if !self.is_in_bounds(coordinate) {
            return None;
        }
        self.tiles.get(coordinate.y * self.width + coordinate.x)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            write!(f, "|")?;
            for x in 0..self.width {
                let tile = *self.tiles.get(y * self.width + x).ok_or(std::fmt::Error)?;
                let elevation = match tile {
                    Tile::Normal(Elevation(elevation)) => format!("{:0>2}", elevation),
                    Tile::Start(_) => "SS".to_owned(),
                    Tile::End(_) => "EE".to_owned(),
                };
                write!(f, "{}|", elevation)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn start_to_end(grid: &Grid) -> Distance {
    let start = grid.start();
    let end = grid.end();

    let offsets = [
        Vector { dx: 0, dy: 1 },
        Vector { dx: 0, dy: -1 },
        Vector { dx: 1, dy: 0 },
        Vector { dx: -1, dy: 0 },
    ];

    fn bfs(
        grid: &Grid,
        current: HashSet<Coordinate>,
        visited: HashSet<Coordinate>,
        destination: &Coordinate,
        offsets: &[Vector; 4],
        counter: usize,
    ) -> usize {
        let next = current
            .iter()
            .flat_map(|coordinate| {
                let from_tile = grid.tile(coordinate).unwrap_or_else(|| {
                    panic!(
                        "Current tile at coordinate `{:?}` cannot be found.",
                        coordinate
                    )
                });

                offsets.iter().filter_map(|direction| {
                    let shifted = coordinate.shift(direction)?;
                    let to_tile = grid.tile(&shifted)?;
                    match from_tile.is_step_possible(to_tile) {
                        true => Some(shifted),
                        false => None,
                    }
                })
            })
            .filter(|coordinate| !visited.contains(coordinate));

        if next.clone().any(|ref x| x == destination) {
            return counter + 1;
        }

        let next = next.collect();
        let visited = visited.union(&next).copied().collect();
        bfs(grid, next, visited, destination, offsets, counter + 1)
    }

    Distance(bfs(
        grid,
        HashSet::from([*start]),
        HashSet::from([*start]),
        end,
        &offsets,
        0,
    ))
}

fn solve_first(input: &str) -> String {
    let grid = Grid::parse(input);
    start_to_end(&grid).0.to_string()
}

fn solve_second(_input: &str) -> String {
    "".to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 31, solve_first(&example));
        println!("Second: Expected {} found {}.", 0, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
