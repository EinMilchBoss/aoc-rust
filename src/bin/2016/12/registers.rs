use crate::instruction::{RegisterId, Word};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Registers([Word; 4]);

impl Registers {
    pub fn new() -> Self {
        Self([0, 0, 0, 0])
    }

    pub fn register_value(&self, register_id: RegisterId) -> Word {
        let Self(registers) = self;
        registers[register_id as usize]
    }

    pub fn register_value_mut(&mut self, register_id: RegisterId) -> &mut Word {
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

impl From<[Word; 4]> for Registers {
    fn from(register_values: [Word; 4]) -> Self {
        Self(register_values)
    }
}

#[cfg(test)]
mod registers_tests {
    use super::*;

    #[test]
    fn register_value_test() {
        let registers = Registers([5, 0, 0, 0]);

        assert_eq!(5, registers.register_value(RegisterId::A));
    }

    #[test]
    fn register_value_mut_test_unsigned() {
        let mut registers = Registers::new();

        *registers.register_value_mut(RegisterId::A) = 5;

        assert_eq!([5, 0, 0, 0], registers.0);
    }

    #[test]
    fn register_value_mut_test_signed() {
        let mut registers = Registers::new();

        *registers.register_value_mut(RegisterId::A) = -5;

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

    #[test]
    fn trait_from_test_from() {
        assert_eq!(Registers([1, 2, 3, 4]), [1, 2, 3, 4].into());
    }
}
