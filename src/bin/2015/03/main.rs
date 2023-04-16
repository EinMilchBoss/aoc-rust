use std::{collections::HashSet, ops::Add};

use util::std::*;

const YEAR: Year = Year("2015");
const DAY: Day = Day("03");

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position(i32, i32);

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("Found unexpected char '{}' while parsing.", value),
        }
    }
}

struct Deliverer {
    current_pos: Position,
}

impl Deliverer {
    fn at_start() -> Self {
        Self {
            current_pos: Position(0, 0),
        }
    }

    fn go_deliver(&mut self, dir: Direction) {
        self.current_pos = match dir {
            Direction::Up => Position(0, 1) + self.current_pos,
            Direction::Down => Position(0, -1) + self.current_pos,
            Direction::Right => Position(1, 0) + self.current_pos,
            Direction::Left => Position(-1, 0) + self.current_pos,
        };
    }
}

fn solve_first(input: &str) -> String {
    let mut santa = Deliverer::at_start();

    input
        .chars()
        .map(|char| {
            santa.go_deliver(char.into());
            santa.current_pos
        })
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

fn solve_second(input: &str) -> String {
    let mut santa = Deliverer::at_start();
    let mut robo_santa = Deliverer::at_start();

    input
        .chars()
        .enumerate()
        .map(|(index, char)| {
            if index % 2 == 0 {
                santa.go_deliver(char.into());
                santa.current_pos
            } else {
                robo_santa.go_deliver(char.into());
                robo_santa.current_pos
            }
        })
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 4, solve_first(&example));
        println!("Second: Expected {} found {}.", 3, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
