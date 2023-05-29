// 5 == 0 / 0
// Hoch, Runter, Rechts, Links bis 1 und -1 in beiden Dimensionen

use crate::instruction::Instruction;

static BUTTON_LAYOUT: [[u8; 3]; 3] = [[7, 8, 9], [4, 5, 6], [1, 2, 3]];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonNumber(pub u8);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonLocation {
    pub x: i8,
    pub y: i8,
}

impl ButtonLocation {
    pub fn at_start() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn button_number(&self) -> ButtonNumber {
        let y_offset = (self.y + 1) as usize;
        let x_offset = (self.x + 1) as usize;
        ButtonNumber(BUTTON_LAYOUT[y_offset][x_offset])
    }

    pub fn go_relative(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Up => self.y = Self::add_in_bounds(self.y, 1),
            Instruction::Down => self.y = Self::add_in_bounds(self.y, -1),
            Instruction::Right => self.x = Self::add_in_bounds(self.x, 1),
            Instruction::Left => self.x = Self::add_in_bounds(self.x, -1),
        };
    }

    fn add_in_bounds(position: i8, offset: i8) -> i8 {
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

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn button_location_at_start_test() {
        assert_eq!(ButtonLocation { x: 0, y: 0 }, ButtonLocation::at_start());
    }

    #[rstest]
    #[case(ButtonNumber(1), ButtonLocation { x: -1, y: 1 })]
    #[case(ButtonNumber(2), ButtonLocation { x: 0, y: 1 })]
    #[case(ButtonNumber(3), ButtonLocation { x: 1, y: 1 })]
    #[case(ButtonNumber(4), ButtonLocation { x: -1, y: 0 })]
    #[case(ButtonNumber(5), ButtonLocation { x: 0, y: 0 })]
    #[case(ButtonNumber(6), ButtonLocation { x: 1, y: 0 })]
    #[case(ButtonNumber(7), ButtonLocation { x: -1, y: -1 })]
    #[case(ButtonNumber(8), ButtonLocation { x: 0, y: -1 })]
    #[case(ButtonNumber(9), ButtonLocation { x: 1, y: -1 })]
    fn button_location_button_number_test(
        #[case] button_number: ButtonNumber,
        #[case] button_location: ButtonLocation,
    ) {
        assert_eq!(button_number, button_location.button_number());
    }

    #[rstest]
    #[case(ButtonLocation { x: 0, y: 1 }, Instruction::Up)]
    #[case(ButtonLocation { x: 0, y: -1 }, Instruction::Down)]
    #[case(ButtonLocation { x: 1, y: 0 }, Instruction::Right)]
    #[case(ButtonLocation { x: -1, y: 0 }, Instruction::Left)]
    fn button_location_go_relative_test_in_bounds(
        #[case] expected: ButtonLocation,
        #[case] instruction: Instruction,
    ) {
        let mut button_location = ButtonLocation::at_start();

        button_location.go_relative(instruction);

        assert_eq!(expected, button_location);
    }

    #[rstest]
    #[case(ButtonLocation { x: 0, y: 1 }, Instruction::Up)]
    #[case(ButtonLocation { x: 0, y: -1 }, Instruction::Down)]
    #[case(ButtonLocation { x: 1, y: 0 }, Instruction::Right)]
    #[case(ButtonLocation { x: -1, y: 0 }, Instruction::Left)]
    fn button_location_go_relative_test_out_of_bounds(
        #[case] expected: ButtonLocation,
        #[case] instruction: Instruction,
    ) {
        let mut button_location = ButtonLocation::at_start();

        button_location.go_relative(instruction);
        button_location.go_relative(instruction);

        assert_eq!(expected, button_location);
    }

    #[rstest]
    #[case(1, 1)]
    #[case(-1, -1)]
    fn button_location_add_in_bounds_test(#[case] position: i8, #[case] offset: i8) {
        assert_eq!(position, ButtonLocation::add_in_bounds(position, offset));
    }
}
