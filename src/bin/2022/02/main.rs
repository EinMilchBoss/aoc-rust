use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::str::FromStr;

use util::std::*;
  
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const YEAR: Year = Year("2022");
const DAY: Day = Day("02");

#[derive(Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl FromStr for Outcome {
    type Err = String;
    
    fn from_str(outcome: &str) -> Result<Self, <Self as FromStr>::Err> {
        match outcome {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(format!("Outcome {} could not be parsed.", outcome)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, FromPrimitive)]
enum Command {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Command {
    type Err = String;
    
    fn from_str(command: &str) -> Result<Self, <Self as FromStr>::Err> {
        match command {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(format!("Command {} could not be parsed.", command)),
        }
    }
}

impl PartialOrd for Command {
    fn partial_cmp(&self, other: &Command) -> Option<Ordering> { 
        let difference = *self as i8 - *other as i8;
        
        if let -1 | 2 = difference {
            Some(Ordering::Less)
        } else if let 1 | -2 = difference {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

struct CommandRound {
    opponent: Command,
    player: Command,
}

impl CommandRound {
    fn new(opponent: &Command, player: &Command) -> Self {
        CommandRound {
            opponent: *opponent,
            player: *player,
        }
    }

    fn round_outcome(&self) -> Outcome {
        match self.player.partial_cmp(&self.opponent).unwrap() {
            Ordering::Greater => Outcome::Win,
            Ordering::Equal => Outcome::Draw,
            Ordering::Less => Outcome::Lose,
        }
    }
}

struct OutcomeRound {
    opponent: Command,
    outcome: Outcome,
}

impl OutcomeRound {
    fn new(opponent: &Command, outcome: &Outcome) -> Self {
        OutcomeRound {
            opponent: *opponent,
            outcome: *outcome,
        }
    }

    fn player_command(&self) -> Command {
        let mut player_value;
        let opponent_value = self.opponent as u8;
        match self.outcome {
            Outcome::Win => {
                player_value = opponent_value + 1;
                cycle(1..=3, &mut player_value);
                FromPrimitive::from_u8(player_value).unwrap()
            },
            Outcome::Lose => {
                player_value = opponent_value - 1;
                cycle(1..=3, &mut player_value);
                FromPrimitive::from_u8(player_value).unwrap()
            },
            Outcome::Draw => self.opponent,
        }
    }
}

fn cycle(range: RangeInclusive<u8>, value: &mut u8) {
    if *value > *range.end() {
        while !range.contains(value) {
            *value -= range.end();
        }
    } else if *value < *range.start() {
        while !range.contains(value) {
            *value += range.end();
        }
    }
}

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