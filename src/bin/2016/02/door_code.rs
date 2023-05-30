pub mod diffuse_keypad;
pub mod normal_keypad;

use std::{collections::HashMap, ops::AddAssign};

use crate::instruction::Instruction;

pub trait Button {
    fn button_number(&self) -> ButtonNumber;
    fn follow_instruction(&mut self, instruction: Instruction);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonNumber(pub char);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ButtonLocation {
    x: i8,
    y: i8,
}

impl ButtonLocation {
    pub const fn at(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    pub fn button_number(&self, keypad_layout: HashMap<Self, ButtonNumber>) -> ButtonNumber {
        keypad_layout[self]
    }
}

impl AddAssign for ButtonLocation {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn is_in_3_times_3_grid(location: ButtonLocation) -> bool {
    let valid_range = -1..=1;
    valid_range.contains(&location.x) && valid_range.contains(&location.y)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[test]
    fn button_location_at_test() {
        assert_eq!(
            ButtonLocation { x: 10, y: -10 },
            ButtonLocation::at(10, -10)
        );
    }

    #[fixture]
    fn keypad_layout() -> Vec<(ButtonLocation, ButtonNumber)> {
        vec![(ButtonLocation::at(0, 0), ButtonNumber('0'))]
    }

    #[rstest]
    fn button_location_button_number_test_ok(keypad_layout: Vec<(ButtonLocation, ButtonNumber)>) {
        let location = ButtonLocation::at(0, 0);

        let button_number = location.button_number(HashMap::from_iter(keypad_layout));

        assert_eq!(ButtonNumber('0'), button_number);
    }

    #[rstest]
    #[should_panic]
    fn button_location_button_number_test_err(keypad_layout: Vec<(ButtonLocation, ButtonNumber)>) {
        let location = ButtonLocation::at(5, -5);

        location.button_number(HashMap::from_iter(keypad_layout));
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

    #[rstest]
    #[case(ButtonLocation::at(0, 0))]
    #[case(ButtonLocation::at(0, 1))]
    #[case(ButtonLocation::at(0, -1))]
    #[case(ButtonLocation::at(1, 0))]
    #[case(ButtonLocation::at(-1, 0))]
    #[case(ButtonLocation::at(1, 1))]
    #[case(ButtonLocation::at(1, -1))]
    #[case(ButtonLocation::at(-1, 1))]
    #[case(ButtonLocation::at(-1, -1))]
    fn keypad_button_is_in_3_times_3_grid_test_true(#[case] location: ButtonLocation) {
        assert!(super::is_in_3_times_3_grid(location));
    }

    #[rstest]
    #[case(ButtonLocation::at(0, 2))]
    #[case(ButtonLocation::at(0, -2))]
    #[case(ButtonLocation::at(2, 0))]
    #[case(ButtonLocation::at(-2, 0))]
    fn keypad_button_is_in_3_times_3_grid_test_false(#[case] location: ButtonLocation) {
        assert!(!super::is_in_3_times_3_grid(location));
    }
}
