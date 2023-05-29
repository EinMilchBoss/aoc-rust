pub mod diffuse_keypad;
pub mod normal_keypad;

use crate::instruction::Instruction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonNumber(pub char);

pub trait Button {
    fn button_number(&self) -> ButtonNumber;
    fn follow_instruction(&mut self, instruction: Instruction);
}
