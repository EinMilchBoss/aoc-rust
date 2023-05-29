use super::{Button, ButtonLocation, ButtonNumber};
use crate::instruction::Instruction;

static BUTTON_LAYOUT: [[char; 3]; 3] = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3']];

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

    fn trim_to_bounds(&mut self) {
        match &mut self.location {
            ButtonLocation { x, .. } if *x > 1 => *x = 1,
            ButtonLocation { x, .. } if *x < -1 => *x = -1,
            ButtonLocation { y, .. } if *y > 1 => *y = 1,
            ButtonLocation { y, .. } if *y < -1 => *y = -1,
            _ => (),
        }
    }
}

impl Button for KeypadButton {
    fn button_number(&self) -> ButtonNumber {
        let y_index = (self.location.y + 1) as usize;
        let x_index = (self.location.x + 1) as usize;
        ButtonNumber(BUTTON_LAYOUT[y_index][x_index])
    }

    fn follow_instruction(&mut self, instruction: Instruction) {
        self.location += instruction.button_position_offset();
        self.trim_to_bounds();
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
    #[case(KeypadButton::at_location(0, 1), KeypadButton::at_location(0, 2))]
    #[case(KeypadButton::at_location(0, -1), KeypadButton::at_location(0, -2))]
    #[case(KeypadButton::at_location(1, 0), KeypadButton::at_location(2, 0))]
    #[case(KeypadButton::at_location(-1, 0), KeypadButton::at_location(-2, 0))]
    fn keypad_button_trim_to_bounds_test(
        #[case] expected: KeypadButton,
        #[case] mut button: KeypadButton,
    ) {
        button.trim_to_bounds();

        assert_eq!(expected, button);
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
