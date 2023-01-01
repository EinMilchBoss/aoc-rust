use std::str::FromStr;

use outcome::Outcome;
use command::Command;
use outcome_round::OutcomeRound;
use command_round::CommandRound;
use util::std::*;

mod outcome;
mod command;
mod outcome_round;
mod command_round;

const YEAR: Year = Year("2022");
const DAY: Day = Day("02");

fn solve_first(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut iter = line
                .split(' ')
                .map(|command| Command::from_str(command).unwrap());
            
            let opponent = iter.next();
            let player = iter.next();
            if opponent.is_none() || player.is_none() {
                panic!("Command of opponent or player was None.");
            }

            let rounds = CommandRound::new(&opponent.unwrap(), &player.unwrap());
            rounds.round_outcome() as u32 + rounds.player as u32
        })
        .sum::<u32>()
        .to_string()
}

fn solve_second(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut iter = line
                .split(' ');

            let opponent = Command::from_str(iter.next().unwrap());
            let outcome = Outcome::from_str(iter.next().unwrap());
            if opponent.is_err() || outcome.is_err() {
                panic!("Command of opponent or outcome was Err.");
            }

            let rounds = OutcomeRound::new(&opponent.unwrap(), &outcome.unwrap());
            rounds.player_command() as u32 + rounds.outcome as u32
        })
        .sum::<u32>()
        .to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 15, solve_first(&example));
        println!("Second: Expected {} found {}.", 12, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}