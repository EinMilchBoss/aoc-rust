use std::{str::FromStr, cmp::Ordering};

use num_derive::FromPrimitive;

#[derive(Clone, Copy, PartialEq, FromPrimitive)]
pub enum Command {
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