use std::collections::HashSet;

use coordinate::*;
use grid::*;
use tile::*;
use util::std::*;

mod coordinate;
mod grid;
mod tile;

const YEAR: Year = Year("2022");
const DAY: Day = Day("12");

const LOWEST: u8 = b'a';
const HIGHEST: u8 = b'z';
const START: u8 = b'S';
const END: u8 = b'E';

#[allow(dead_code)]
fn status(grid: &Grid, next: &HashSet<Coordinate>, visited: &HashSet<Coordinate>) -> String {
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

fn bfs(
    grid: &Grid,
    current: &HashSet<Coordinate>,
    visited: &mut HashSet<Coordinate>,
    destinations: &HashSet<Coordinate>,
    counter: usize,
) -> usize {
    let next: &HashSet<_> = &current
        .iter()
        .flat_map(|coordinate| grid.valid_neighbors(coordinate))
        .filter(|coordinate| !visited.contains(coordinate))
        .collect();

    if next.intersection(destinations).count() >= 1 {
        return counter + 1;
    }

    visited.extend(next);
    bfs(grid, next, visited, destinations, counter + 1)
}

struct Distance(usize);

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

fn lowest_to_end(grid: &Grid) -> Distance {
    let starts = grid.tiles(|tile, coordinate| match tile {
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
