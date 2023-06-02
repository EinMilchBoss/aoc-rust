use std::collections::HashMap;

use super::{Button, ButtonLocation, ButtonNumber};
use crate::instruction::Instruction;

static KEYPAD_LAYOUT: [(ButtonLocation, ButtonNumber); 9] = [
    (ButtonLocation::at(-1, 1), ButtonNumber('1')),
    (ButtonLocation::at(0, 1), ButtonNumber('2')),
    (ButtonLocation::at(1, 1), ButtonNumber('3')),
    (ButtonLocation::at(-1, 0), ButtonNumber('4')),
    (ButtonLocation::at(0, 0), ButtonNumber('5')),
    (ButtonLocation::at(1, 0), ButtonNumber('6')),
    (ButtonLocation::at(-1, -1), ButtonNumber('7')),
    (ButtonLocation::at(0, -1), ButtonNumber('8')),
    (ButtonLocation::at(1, -1), ButtonNumber('9')),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeypadButton {
    location: ButtonLocation,
}

impl KeypadButton {
    pub fn at_start() -> Self {
        Self {
            location: ButtonLocation::default(),
        }
    }

    #[cfg(test)]
    pub fn at_location(x: i8, y: i8) -> Self {
        Self {
            location: ButtonLocation::at(x, y),
        }
    }
}

impl Button for KeypadButton {
    fn button_number(&self) -> ButtonNumber {
        self.location.button_number(HashMap::from(KEYPAD_LAYOUT))
    }

    fn follow_instruction(&mut self, instruction: Instruction) {
        self.location =
            super::follow_instruction(self.location, instruction, super::is_in_3_times_3_grid);
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn keypad_button_at_start_test() {
        assert_eq!(
            KeypadButton {
                location: ButtonLocation { x: 0, y: 0 }
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
    #[case(ButtonNumber('1'), KeypadButton::at_location(-1, 1))]
    #[case(ButtonNumber('2'), KeypadButton::at_location(0, 1))]
    #[case(ButtonNumber('3'), KeypadButton::at_location(1, 1))]
    #[case(ButtonNumber('4'), KeypadButton::at_location(-1, 0))]
    #[case(ButtonNumber('5'), KeypadButton::at_location(0, 0))]
    #[case(ButtonNumber('6'), KeypadButton::at_location(1, 0))]
    #[case(ButtonNumber('7'), KeypadButton::at_location(-1, -1))]
    #[case(ButtonNumber('8'), KeypadButton::at_location(0, -1))]
    #[case(ButtonNumber('9'), KeypadButton::at_location(1, -1))]
    fn keypad_button_trait_button_button_number_test(
        #[case] button_number: ButtonNumber,
        #[case] button: KeypadButton,
    ) {
        assert_eq!(button_number, button.button_number());
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
        let mut button = KeypadButton::at_start();

        button.follow_instruction(instruction);

        assert_eq!(expected, button);
    }

    #[rstest]
    #[case(KeypadButton::at_location(0, 1), Instruction::Up)]
    #[case(KeypadButton::at_location(0, -1), Instruction::Down)]
    #[case(KeypadButton::at_location(1, 0), Instruction::Right)]
    #[case(KeypadButton::at_location(-1, 0), Instruction::Left)]
    fn keypad_button_trait_button_follow_instruction_test_out_of_bounds(
        #[case] expected: KeypadButton,
        #[case] instruction: Instruction,
    ) {
        let mut button = KeypadButton::at_start();

        button.follow_instruction(instruction);
        button.follow_instruction(instruction);

        assert_eq!(expected, button);
    }
}
