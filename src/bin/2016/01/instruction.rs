use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Left(Steps),
    Right(Steps),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Steps(pub usize);

impl FromStr for Instruction {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut string_iter = string.chars();

        let first_char = string_iter.next().ok_or(())?;
        let steps = string_iter
            .collect::<String>()
            .parse::<usize>()
            .map_err(|_| ())?;

        match first_char {
            'L' => Ok(Self::Left(Steps(steps))),
            'R' => Ok(Self::Right(Steps(steps))),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_trait_from_str_test_ok() {
        let left = "L12";
        let right = "R34";

        assert_eq!(Ok(Instruction::Left(Steps(12))), left.parse());
        assert_eq!(Ok(Instruction::Right(Steps(34))), right.parse());
    }

    #[test]
    fn instruction_trait_from_str_test_err_direction() {
        let invalid_direction = "*12";

        assert_eq!(Err(()), invalid_direction.parse::<Instruction>());
    }

    #[test]
    fn instruction_trait_from_str_test_err_steps() {
        let negative_steps = "L-12";
        let gibberish_steps = "L_*!";
        let no_steps = "L";

        assert_eq!(Err(()), negative_steps.parse::<Instruction>());
        assert_eq!(Err(()), gibberish_steps.parse::<Instruction>());
        assert_eq!(Err(()), no_steps.parse::<Instruction>());
    }
}
