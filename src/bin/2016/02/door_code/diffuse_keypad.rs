use std::collections::HashMap;

use crate::instruction::Instruction;

use super::{Button, ButtonLocation, ButtonNumber};

static KEYPAD_LAYOUT: [(ButtonLocation, ButtonNumber); 13] = [
    (ButtonLocation::at(0, 2), ButtonNumber('1')),
    (ButtonLocation::at(-1, 1), ButtonNumber('2')),
    (ButtonLocation::at(0, 1), ButtonNumber('3')),
    (ButtonLocation::at(1, 1), ButtonNumber('4')),
    (ButtonLocation::at(-2, 0), ButtonNumber('5')),
    (ButtonLocation::at(-1, 0), ButtonNumber('6')),
    (ButtonLocation::at(0, 0), ButtonNumber('7')),
    (ButtonLocation::at(1, 0), ButtonNumber('8')),
    (ButtonLocation::at(2, 0), ButtonNumber('9')),
    (ButtonLocation::at(-1, -1), ButtonNumber('A')),
    (ButtonLocation::at(0, -1), ButtonNumber('B')),
    (ButtonLocation::at(1, -1), ButtonNumber('C')),
    (ButtonLocation::at(0, -2), ButtonNumber('D')),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeypadButton {
    location: ButtonLocation,
}

impl KeypadButton {
    pub fn at_start() -> Self {
        Self {
            location: ButtonLocation::at(-2, 0),
        }
    }

    #[cfg(test)]
    pub fn at_location(x: i8, y: i8) -> Self {
        Self {
            location: ButtonLocation::at(x, y),
        }
    }

    fn is_in_bounds(location: ButtonLocation) -> bool {
        Self::is_corner(location) || super::is_in_3_times_3_grid(location)
    }

    fn is_corner(location: ButtonLocation) -> bool {
        let corner_range = [-2, 2];
        let is_x_dimension_corner = corner_range.contains(&location.x) && location.y == 0;
        let is_y_dimension_corner = corner_range.contains(&location.y) && location.x == 0;
        is_x_dimension_corner || is_y_dimension_corner
    }
}

impl Button for KeypadButton {
    fn button_number(&self) -> ButtonNumber {
        self.location.button_number(HashMap::from(KEYPAD_LAYOUT))
    }

    fn follow_instruction(&mut self, instruction: Instruction) {
        self.location = super::follow_instruction(self.location, instruction, |location| {
            Self::is_in_bounds(location)
        });
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn keypad_button_at_start_test() {
        assert_eq!(
            KeypadButton {
                location: ButtonLocation { x: -2, y: 0 }
            },
            KeypadButton::at_start()
        );
    }

    #[test]
    fn keypad_button_at_location_test() {
        assert_eq!(
            KeypadButton {
                location: ButtonLocation { x: 1, y: -1 }
            },
            KeypadButton::at_location(1, -1)
        );
    }

    #[rstest]
    #[case(ButtonLocation::at(0, 2))]
    #[case(ButtonLocation::at(0, -2))]
    #[case(ButtonLocation::at(2, 0))]
    #[case(ButtonLocation::at(-2, 0))]
    #[case(ButtonLocation::at(0, 0))]
    #[case(ButtonLocation::at(0, 1))]
    #[case(ButtonLocation::at(0, -1))]
    #[case(ButtonLocation::at(1, 0))]
    #[case(ButtonLocation::at(-1, 0))]
    #[case(ButtonLocation::at(1, 1))]
    #[case(ButtonLocation::at(-1, -1))]
    fn keypad_button_is_in_bounds_test_true(#[case] location: ButtonLocation) {
        assert!(KeypadButton::is_in_bounds(location));
    }

    #[rstest]
    #[case(ButtonLocation::at(1, 2))]
    #[case(ButtonLocation::at(-1, -2))]
    #[case(ButtonLocation::at(2, 1))]
    #[case(ButtonLocation::at(-2, -1))]
    fn keypad_button_is_in_bounds_test_false(#[case] location: ButtonLocation) {
        assert!(!KeypadButton::is_in_bounds(location));
    }

    #[rstest]
    #[case(ButtonLocation::at(0, 2))]
    #[case(ButtonLocation::at(0, -2))]
    #[case(ButtonLocation::at(2, 0))]
    #[case(ButtonLocation::at(-2, 0))]
    fn keypad_button_is_corner_test_true(#[case] location: ButtonLocation) {
        assert!(KeypadButton::is_corner(location));
    }

    #[rstest]
    #[case(ButtonLocation::at(0, 0))]
    #[case(ButtonLocation::at(1, 2))]
    #[case(ButtonLocation::at(-1, -2))]
    #[case(ButtonLocation::at(2, 1))]
    #[case(ButtonLocation::at(-2, -1))]
    fn keypad_button_is_corner_test_false(#[case] location: ButtonLocation) {
        assert!(!KeypadButton::is_corner(location));
    }

    #[rstest]
    #[case(KeypadButton::at_location(0, 1), Instruction::Up)]
    #[case(KeypadButton::at_location(0, -1), Instruction::Down)]
    #[case(KeypadButton::at_location(1, 0), Instruction::Right)]
    #[case(KeypadButton::at_location(-1, 0), Instruction::Left)]
    fn keypad_button_trait_button_follow_instruction_test_in_bounds(
        #[case] expected: KeypadButton,
        #[case] instruction: Instruction,
    ) {
        let mut button = KeypadButton::at_location(0, 0);

        button.follow_instruction(instruction);

        assert_eq!(expected, button);
    }

    #[rstest]
    #[case(KeypadButton::at_location(0, 2), Instruction::Up)]
    #[case(KeypadButton::at_location(0, -2), Instruction::Down)]
    #[case(KeypadButton::at_location(2, 0), Instruction::Right)]
    #[case(KeypadButton::at_location(-2, 0), Instruction::Left)]
    fn keypad_button_trait_button_follow_instruction_test_corners(
        #[case] expected: KeypadButton,
        #[case] instruction: Instruction,
    ) {
        let mut button = KeypadButton::at_location(0, 0);

        button.follow_instruction(instruction);
        button.follow_instruction(instruction);

        assert_eq!(expected, button);
    }

    #[rstest]
    #[case(KeypadButton::at_location(0, 2), Instruction::Up)]
    #[case(KeypadButton::at_location(0, -2), Instruction::Down)]
    #[case(KeypadButton::at_location(2, 0), Instruction::Right)]
    #[case(KeypadButton::at_location(-2, 0), Instruction::Left)]
    fn keypad_button_trait_button_follow_instruction_test_out_of_bounds(
        #[case] expected: KeypadButton,
        #[case] instruction: Instruction,
    ) {
        let mut button = KeypadButton::at_location(0, 0);

        button.follow_instruction(instruction);
        button.follow_instruction(instruction);
        button.follow_instruction(instruction);

        assert_eq!(expected, button);
    }

    #[rstest]
    #[case(ButtonNumber('1'), KeypadButton::at_location(0, 2))]
    #[case(ButtonNumber('2'), KeypadButton::at_location(-1, 1))]
    #[case(ButtonNumber('3'), KeypadButton::at_location(0, 1))]
    #[case(ButtonNumber('4'), KeypadButton::at_location(1, 1))]
    #[case(ButtonNumber('5'), KeypadButton::at_location(-2, 0))]
    #[case(ButtonNumber('6'), KeypadButton::at_location(-1, 0))]
    #[case(ButtonNumber('7'), KeypadButton::at_location(0, 0))]
    #[case(ButtonNumber('8'), KeypadButton::at_location(1, 0))]
    #[case(ButtonNumber('9'), KeypadButton::at_location(2, 0))]
    #[case(ButtonNumber('A'), KeypadButton::at_location(-1, -1))]
    #[case(ButtonNumber('B'), KeypadButton::at_location(0, -1))]
    #[case(ButtonNumber('C'), KeypadButton::at_location(1, -1))]
    #[case(ButtonNumber('D'), KeypadButton::at_location(0, -2))]
    fn keypad_button_trait_button_button_number_test(
        #[case] button_number: ButtonNumber,
        #[case] button: KeypadButton,
    ) {
        assert_eq!(button_number, button.button_number());
    }
}
