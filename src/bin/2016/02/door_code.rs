pub mod diffuse_keypad;
pub mod normal_keypad;

use std::ops::AddAssign;

use crate::instruction::Instruction;

pub trait Button {
    fn button_number(&self) -> ButtonNumber;
    fn follow_instruction(&mut self, instruction: Instruction);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonNumber(pub char);

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ButtonLocation {
    x: i8,
    y: i8,
}

impl ButtonLocation {
    pub fn at(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}

impl AddAssign for ButtonLocation {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn button_location_at_test() {
        assert_eq!(
            ButtonLocation { x: 10, y: -10 },
            ButtonLocation::at(10, -10)
        );
    }

    #[rstest]
    #[case(ButtonLocation::at(0, 2), ButtonLocation::at(0, 1))]
    #[case(ButtonLocation::at(0, -2), ButtonLocation::at(0, -1))]
    #[case(ButtonLocation::at(2, 0), ButtonLocation::at(1, 0))]
    #[case(ButtonLocation::at(-2, 0), ButtonLocation::at(-1, 0))]
    fn button_location_trait_add_assign_test(
        #[case] expected: ButtonLocation,
        #[case] mut summand: ButtonLocation,
    ) {
        summand += summand;

        assert_eq!(expected, summand);
    }
}
