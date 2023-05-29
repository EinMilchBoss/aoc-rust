use super::{Button, ButtonNumber};
use crate::instruction::Instruction;

static BUTTON_LAYOUT: [[char; 3]; 3] = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3']];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeypadButton {
    x: i8,
    y: i8,
}

impl KeypadButton {
    pub fn at_start() -> Self {
        Self { x: 0, y: 0 }
    }

    #[cfg(test)]
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    fn set_inside_bounds(position: i8, offset: i8) -> i8 {
        let new_position = position + offset;
        if new_position > 1 {
            1
        } else if new_position < -1 {
            -1
        } else {
            new_position
        }
    }
}

impl Button for KeypadButton {
    fn button_number(&self) -> ButtonNumber {
        let y_offset = (self.y + 1) as usize;
        let x_offset = (self.x + 1) as usize;
        ButtonNumber(BUTTON_LAYOUT[y_offset][x_offset])
    }

    fn follow_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Up => self.y = Self::set_inside_bounds(self.y, 1),
            Instruction::Down => self.y = Self::set_inside_bounds(self.y, -1),
            Instruction::Right => self.x = Self::set_inside_bounds(self.x, 1),
            Instruction::Left => self.x = Self::set_inside_bounds(self.x, -1),
        };
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn keypad_button_at_start_test() {
        assert_eq!(KeypadButton { x: 0, y: 0 }, KeypadButton::at_start());
    }

    #[test]
    fn keypad_button_new_test() {
        assert_eq!(KeypadButton { x: 1, y: -1 }, KeypadButton::new(1, -1));
    }

    #[rstest]
    #[case(1, 1)]
    #[case(-1, -1)]
    fn keypad_button_set_inside_bounds_test(#[case] position: i8, #[case] offset: i8) {
        assert_eq!(position, KeypadButton::set_inside_bounds(position, offset));
    }

    #[rstest]
    #[case(ButtonNumber('1'), KeypadButton { x: -1, y: 1 })]
    #[case(ButtonNumber('2'), KeypadButton { x: 0, y: 1 })]
    #[case(ButtonNumber('3'), KeypadButton { x: 1, y: 1 })]
    #[case(ButtonNumber('4'), KeypadButton { x: -1, y: 0 })]
    #[case(ButtonNumber('5'), KeypadButton { x: 0, y: 0 })]
    #[case(ButtonNumber('6'), KeypadButton { x: 1, y: 0 })]
    #[case(ButtonNumber('7'), KeypadButton { x: -1, y: -1 })]
    #[case(ButtonNumber('8'), KeypadButton { x: 0, y: -1 })]
    #[case(ButtonNumber('9'), KeypadButton { x: 1, y: -1 })]
    fn keypad_button_trait_button_button_number_test(
        #[case] button_number: ButtonNumber,
        #[case] button: KeypadButton,
    ) {
        assert_eq!(button_number, button.button_number());
    }

    #[rstest]
    #[case(KeypadButton { x: 0, y: 1 }, Instruction::Up)]
    #[case(KeypadButton { x: 0, y: -1 }, Instruction::Down)]
    #[case(KeypadButton { x: 1, y: 0 }, Instruction::Right)]
    #[case(KeypadButton { x: -1, y: 0 }, Instruction::Left)]
    fn keypad_button_trait_button_follow_instruction_test_in_bounds(
        #[case] expected: KeypadButton,
        #[case] instruction: Instruction,
    ) {
        let mut button = KeypadButton::at_start();

        button.follow_instruction(instruction);

        assert_eq!(expected, button);
    }

    #[rstest]
    #[case(KeypadButton { x: 0, y: 1 }, Instruction::Up)]
    #[case(KeypadButton { x: 0, y: -1 }, Instruction::Down)]
    #[case(KeypadButton { x: 1, y: 0 }, Instruction::Right)]
    #[case(KeypadButton { x: -1, y: 0 }, Instruction::Left)]
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
