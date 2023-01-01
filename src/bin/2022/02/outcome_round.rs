use std::ops::RangeInclusive;

use crate::command::Command;
use crate::outcome::Outcome;

use num_traits::FromPrimitive;

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

pub struct OutcomeRound {
    pub opponent: Command,
    pub outcome: Outcome,
}

impl OutcomeRound {
    pub fn new(opponent: &Command, outcome: &Outcome) -> Self {
        OutcomeRound {
            opponent: *opponent,
            outcome: *outcome,
        }
    }

    pub fn player_command(&self) -> Command {
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