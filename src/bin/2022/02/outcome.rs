use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum Outcome {
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