use std::fmt::Display;

use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("12");

const START: char = 'S';
const END: char = 'E';

#[derive(Clone, Copy, Debug)]
struct Elevation(u8);

impl Elevation {
    fn is_step_possible(&self, to: Elevation) -> bool {
        to.0 - self.0 <= 1
    }
}

// impl TryFrom<char> for Elevation {
//     type Error = AocError;

//     fn try_from(value: char) -> Result<Self, Self::Error> {
//         let base = b'a';
//         match value {
//             'S' => Ok(Elevation(b'a' - base)),
//             'E' => Ok(Elevation(b'z' - base)),
//             'a'..='z' => Ok(Elevation(value as u8 - base)),
//             _ => Err(AocError),
//         }
//     }
// }

// struct Position {
//     x: isize,
//     y: isize,
// }

// impl Position {
//     fn new(x: isize, y: isize) -> Self {
//         Position { x, y }
//     }
// }

// impl Add for Position {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self::Output::new(self.x + rhs.x, self.y + rhs.y)
//     }
// }

#[derive(Clone, Copy, Debug)]
enum Tile {
    Normal(Elevation),
    Start(Coordinate),
    End(Coordinate),
}

// impl From<char> for Tile {
//     fn from(value: char) -> Self {
//         match value {
//             'S' => Tile::Start,
//             'E' => Tile::End,
//             _ => Tile::Normal,
//         }
//     }
// }

struct Distance(usize);

// impl Tile {
//     fn new(elev: Elevation, pos: Position, ttype: Tile) -> Self {
//         Tile { elev, pos, ttype }
//     }
// }

// fn get_coord(grid: &Vec<Vec<Elevation>>, pos: &Position) -> Option<Elevation> {
//     if pos.y < 0 || pos.x < 0 {
//         return None;
//     }

//     let column = grid.get(pos.y as usize)?;
//     column.get(pos.x as usize)?
// }

// fn shortest_distance(grid: &Vec<Vec<Elevation>>, start: Position, end: Position) -> Distance {
//     // gerade, nächste, gesehen
//     // gerade ist Start
//     //
//     let start_elev = get_coord(grid, &start).unwrap();
//     let nexts = vec![];
//     let next = [
//         start + Position::new(0, 1),
//         start + Position::new(0, -1),
//         start + Position::new(1, 0),
//         start + Position::new(-1, 0),
//     ];
//     if (start_elev.is_step_possible(next)) {
//         nexts.push(next)
//     }

//     fn bfs(
//         grid: &Vec<Vec<Tile>>,
//         end: &Tile,
//         current: &Tile,
//         next: Vec<Tile>,
//         step: usize,
//     ) -> usize {
//         if (current == end) {
//             return step;
//         }

//         grid.into_iter().position();

//         0
//     }

//     Distance(bfs(grid, end, start, vec![start], 0))
// }

struct Vector {
    dx: isize,
    dy: isize,
}

#[derive(Clone, Copy, Debug)]
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
                    b'S' => Tile::Start(Coordinate { x, y }),
                    b'E' => Tile::End(Coordinate { x, y }),
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

fn start_to_end(grid: &Grid) -> usize {
    // get start
    // get up down left right
    // check if step for directions is possible
    // save next field coords in a HashSet (no duplicates!)
    // pass hashset to next recursive call
    // continue until end found
    // each recursive call increments the step by 1
    // save the visited coords in a hashset

    let start = grid.start();
    let end = grid.end();

    let dirs = [
        Vector { dx: 0, dy: 1 },
        Vector { dx: 0, dy: -1 },
        Vector { dx: 1, dy: 0 },
        Vector { dx: -1, dy: 0 },
    ];

    let coordinates: Vec<_> = dirs
        .iter()
        .filter_map(|dir| start.shift(dir))
        .filter(|coordinate| grid.is_in_bounds(coordinate))
        .filter_map(|coordinate| grid.tile(&coordinate))
        .collect();

    println!("{:#?}", coordinates);

    0
}

fn solve_first(input: &str) -> String {
    // elevation a - z
    // S is start; E is end
    // S == a; E == z
    // as FEW steps as possible (bfs)
    // up down left right are possible directions
    // only possible if elevation difference is <= 1

    //let griddy = Grid::parse("");
    let grid = Grid::parse(input);
    println!("{}", &grid);
    start_to_end(&grid);
    //shortest_distance(&grid, &start, &end);

    "".to_string()
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
        //println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
