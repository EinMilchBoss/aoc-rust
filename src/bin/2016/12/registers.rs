use crate::instruction::{RegisterId, Word};

#[derive(Debug, Clone, PartialEq)]
pub struct Registers([Word; 4]);

impl Registers {
    pub fn new() -> Self {
        Self([0, 0, 0, 0])
    }

    pub fn get(&self, register_id: RegisterId) -> Word {
        let Self(registers) = self;
        registers[register_id as usize]
    }

    pub fn get_mut(&mut self, register_id: RegisterId) -> &mut Word {
        let Self(registers) = self;
        &mut registers[register_id as usize]
    }

    pub fn increment(&mut self, register_id: RegisterId) {
        let Self(registers) = self;
        registers[register_id as usize] += 1;
    }

    pub fn decrement(&mut self, register_id: RegisterId) {
        let Self(registers) = self;
        registers[register_id as usize] -= 1;
    }
}

#[cfg(test)]
mod registers_test {
    use super::*;

    #[test]
    fn get_test() {
        let registers = Registers([5, 0, 0, 0]);

        assert_eq!(5, registers.get(RegisterId::A));
    }

    #[test]
    fn get_mut_test_unsigned() {
        let mut registers = Registers::new();

        *registers.get_mut(RegisterId::A) = 5;

        assert_eq!([5, 0, 0, 0], registers.0);
    }

    #[test]
    fn get_mut_test_signed() {
        let mut registers = Registers::new();

        *registers.get_mut(RegisterId::A) = -5;

        assert_eq!([-5, 0, 0, 0], registers.0);
    }

    #[test]
    fn increment_test() {
        let mut registers = Registers::new();

        registers.increment(RegisterId::A);

        assert_eq!([1, 0, 0, 0], registers.0);
    }

    #[test]
    fn decrement_test() {
        let mut registers = Registers::new();

        registers.decrement(RegisterId::A);

        assert_eq!([-1, 0, 0, 0], registers.0);
    }
}
