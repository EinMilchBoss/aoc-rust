use std::{collections::HashSet, fmt::Display};

use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("12");

const START: u8 = b'S';
const END: u8 = b'E';
const LOWEST: u8 = b'a';
const HIGHEST: u8 = b'z';

static NEIGHBOR_OFFSETS: [Vector; 4] = [
    Vector { dx: 0, dy: 1 },
    Vector { dx: 0, dy: -1 },
    Vector { dx: 1, dy: 0 },
    Vector { dx: -1, dy: 0 },
];

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
        Elevation(match self {
            Tile::Normal(Elevation(elevation)) => *elevation,
            Tile::Start(_) => 0,
            Tile::End(_) => HIGHEST - LOWEST,
        })
    }

    fn is_step_possible(&self, to: &Tile) -> bool {
        let Elevation(from) = self.elevation();
        let Elevation(to) = to.elevation();
        !matches!(to.checked_sub(from), Some(difference) if difference > 1)
    }
}

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

    fn start(&self) -> &Coordinate {
        self.tiles
            .iter()
            .find_map(|tile| match tile {
                Tile::Start(coordinate) => Some(coordinate),
                _ => None,
            })
            .expect("Start was not found in grid.")
    }

    fn end(&self) -> &Coordinate {
        self.tiles
            .iter()
            .find_map(|tile| match tile {
                Tile::End(coordinate) => Some(coordinate),
                _ => None,
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

    fn valid_neighbors(&self, from: &Coordinate) -> Vec<Coordinate> {
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

struct Distance(usize);

fn bfs(
    grid: &Grid,
    current: &HashSet<Coordinate>,
    visited: &mut HashSet<Coordinate>,
    destinations: &HashSet<Coordinate>,
    counter: usize,
) -> usize {
    let next = current
        .iter()
        .flat_map(|coordinate| grid.valid_neighbors(coordinate))
        .filter(|coordinate| !visited.contains(coordinate));

    //next.interse
    // if next.clone().any(|ref x| x == destination) {
    //     return counter + 1;
    // }

    let next: &HashSet<_> = &next.collect();
    if next.intersection(destinations).count() >= 1 {
        return counter + 1;
    }

    visited.extend(next);
    bfs(grid, next, visited, destinations, counter + 1)
}

fn start_to_end(grid: &Grid) -> Distance {
    let start = grid.start();
    let end = grid.end();

    Distance(bfs(
        grid,
        &HashSet::from([*start]),
        &mut HashSet::from([*start]),
        &HashSet::from([*end]),
        0,
    ))
}

fn tiles<F>(grid: &Grid, mut f: F) -> Vec<Coordinate>
where
    F: FnMut(&Tile, Coordinate) -> Option<Coordinate>,
{
    (0..grid.height)
        .flat_map(|y| {
            (0..grid.width).map(move |x| {
                let coordinate = Coordinate { x, y };
                let tile = grid.tile(&coordinate).unwrap_or_else(|| {
                    panic!("Coordinate {:?} could not be found in grid.", coordinate)
                });
                //println!("{:?} {:?}", tile, coordinate);
                (tile, coordinate)
            })
        })
        .filter_map(|(tile, coordinate)| f(tile, coordinate))
        .collect()
}

fn lowest_to_end(grid: &Grid) -> Distance {
    let starts = tiles(grid, |tile, coordinate| match tile {
        Tile::Normal(Elevation(elevation)) if *elevation == 0 => Some(coordinate),
        Tile::Start(_) => Some(coordinate),
        _ => None,
    });
    let end = grid.end();

    Distance(bfs(
        grid,
        &HashSet::from_iter(starts.clone()),
        &mut HashSet::from_iter(starts),
        &HashSet::from([*end]),
        0,
    ))
}

fn solve_first(input: &str) -> String {
    let grid = Grid::parse(input);
    let Distance(distance) = start_to_end(&grid);
    distance.to_string()
}

fn solve_second(input: &str) -> String {
    let grid = Grid::parse(input);
    let Distance(distance) = lowest_to_end(&grid);
    distance.to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 31, solve_first(&example));
        println!("Second: Expected {} found {}.", 29, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
