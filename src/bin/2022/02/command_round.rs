use std::cmp::Ordering;

use crate::command::Command;
use crate::outcome::Outcome;

pub struct CommandRound {
    pub opponent: Command,
    pub player: Command,
}

impl CommandRound {
    pub fn new(opponent: &Command, player: &Command) -> Self {
        CommandRound {
            opponent: *opponent,
            player: *player,
        }
    }

    pub fn round_outcome(&self) -> Outcome {
        match self.player.partial_cmp(&self.opponent).unwrap() {
            Ordering::Greater => Outcome::Win,
            Ordering::Equal => Outcome::Draw,
            Ordering::Less => Outcome::Lose,
        }
    }
}