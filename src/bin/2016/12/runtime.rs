use crate::{instruction::Assembunny, registers::Registers};

#[derive(Debug, Clone, PartialEq)]
pub struct Runtime {
    assembunny: Assembunny,
    registers: Registers,
    ip: usize,
}

impl Runtime {
    pub fn load_assembunny(assembunny: Assembunny) -> Self {
        Self {
            assembunny,
            registers: Registers::new(),
            ip: 0,
        }
    }
}
