use std::cmp::Ordering;
use std::str::FromStr;

use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("02");

#[derive(Clone, Copy, PartialEq)]
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
            _ => Err("Command {command} could not be parsed.".to_string()),
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

struct Round {
    opponent: Command,
    player: Command,
}

impl Round {
    fn new(opponent: &Command, player: &Command) -> Self {
        Round {
            opponent: *opponent,
            player: *player,
        }
    }

    fn points(&self) -> u8 {
        match self.player.partial_cmp(&self.opponent).unwrap() {
            Ordering::Greater => 6,
            Ordering::Equal => 3,
            Ordering::Less => 0,
        }
    }
}

fn solve_first(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut command_iter = line
                .split(' ')
                .map(|command| Command::from_str(command).expect("Command should be parsed."));
            
            let opponent = command_iter.next();
            let player = command_iter.next();
            if opponent.is_none() || player.is_none() {
                panic!("Command of opponent or player was None.");
            }

            let rounds = Round::new(&opponent.unwrap(), &player.unwrap());
            (rounds.points() + rounds.player as u8) as u16
        })
        .sum::<u16>()
        .to_string()
}

fn solve_second(_: &str) -> String {
    "".to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 15, solve_first(&example));
        println!("Second: Expected {} found {}.", 45_000, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}