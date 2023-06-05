use crate::{instruction::Instruction, registers::Registers};

#[derive(Debug, Clone, PartialEq)]
pub struct Runtime {
    instructions: Vec<Instruction>,
    registers: Registers,
    ip: usize,
}
