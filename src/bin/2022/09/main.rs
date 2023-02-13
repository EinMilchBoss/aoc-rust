use std::{collections::HashSet, str::FromStr};

use coordinate::Vector;
use knot::Knot;
use motion::Motion;
use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("09");

mod coordinate;
mod direction;
mod knot;
mod motion;

fn solve_first(input: &str) -> String {
    let mut tail_pos = HashSet::new();
    let mut head = Knot::at_start();
    let mut tail = Knot::at_start();
    for Motion { dir, count } in input.lines().map(|line| Motion::from_str(line).unwrap()) {
        let step = Vector::from_dir(dir);
        for _ in 0..count {
            head.shift(step);
            tail.follow(&head);
            tail_pos.insert(tail.pos);
        }
    }

    tail_pos.len().to_string()
}

fn solve_second(input: &str) -> String {
    let mut tail_pos = HashSet::new();
    let mut head = Knot::at_start();
    let mut tails = [Knot::at_start(); 9];
    let tail_indices = (0..9).collect::<Vec<usize>>();
    for Motion { dir, count } in input.lines().map(|line| Motion::from_str(line).unwrap()) {
        let step = Vector::from_dir(dir);
        for _ in 0..count {
            head.shift(step);
            tails[0].follow(&head);
            for indices in tail_indices.windows(2) {
                if let &[previous, current] = indices {
                    let followee = tails[previous];
                    tails[current].follow(&followee);
                }
            }
            tail_pos.insert(tails[8].pos);
        }
    }

    tail_pos.len().to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 13, solve_first(&example));
        println!("Second: Expected {} found {}.", 1, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
