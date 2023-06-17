use crate::{
    instruction::{Argument, Assembunny, Instruction, RegisterId, Word},
    registers::Registers,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeEnvironment {
    assembunny: Assembunny,
    registers: Registers,
    ip: usize,
}

impl RuntimeEnvironment {
    pub fn load_assembunny(assembunny: Assembunny) -> Self {
        Self {
            assembunny,
            registers: Registers::new(),
            ip: 0,
        }
    }

    pub fn load_assembunny_with_initialized_registers(
        assembunny: Assembunny,
        registers: Registers,
    ) -> Self {
        Self {
            assembunny,
            registers,
            ip: 0,
        }
    }

    pub fn register_value(&self, register_id: RegisterId) -> Word {
        self.registers.register_value(register_id)
    }

    pub fn run_program(&mut self) {
        loop {
            if self.is_program_running() {
                self.execute_next_instruction();
            } else {
                break;
            }
        }
    }

    fn execute_next_instruction(&mut self) {
        match *self.assembunny.get(self.ip).unwrap() {
            Instruction::Cpy { from, into } => self.execute_cpy_instruction(from, into),
            Instruction::Inc(register_id) => self.execute_inc_instruction(register_id),
            Instruction::Dec(register_id) => self.execute_dec_instruction(register_id),
            Instruction::Jnz {
                condition,
                jump_offset,
            } => self.execute_jnz_instruction(condition, jump_offset),
        }
    }

    fn execute_cpy_instruction(&mut self, from: Argument, into: RegisterId) {
        *self.registers.register_value_mut(into) = self.dereference_argument(from);
        self.jump_to_next_instruction();
    }

    fn execute_inc_instruction(&mut self, register_id: RegisterId) {
        self.registers.increment(register_id);
        self.jump_to_next_instruction();
    }

    fn execute_dec_instruction(&mut self, register_id: RegisterId) {
        self.registers.decrement(register_id);
        self.jump_to_next_instruction();
    }

    fn execute_jnz_instruction(&mut self, condition: Argument, jump_offset: Word) {
        if self.condition_is_not_zero(condition) {
            self.jump_to_offset(jump_offset);
        } else {
            self.jump_to_next_instruction();
        }
    }

    fn jump_to_offset(&mut self, jump_offset: Word) {
        let ip = self.ip as isize;
        let jump_offset = jump_offset as isize;
        self.ip = ip.saturating_add(jump_offset) as usize;
    }

    fn condition_is_not_zero(&self, condition: Argument) -> bool {
        self.dereference_argument(condition) != 0
    }

    fn jump_to_next_instruction(&mut self) {
        self.ip += 1;
    }

    fn dereference_argument(&self, argument: Argument) -> Word {
        match argument {
            Argument::Literal(value) => value,
            Argument::Reference(register_id) => self.registers.register_value(register_id),
        }
    }

    fn is_program_running(&self) -> bool {
        let Assembunny(instructions) = &self.assembunny;
        self.ip < instructions.len()
    }
}

#[cfg(test)]
mod runtime_environment_tests {
    use rstest::{fixture, rstest};

    use crate::instruction::*;

    use super::*;

    #[fixture]
    fn assembunny() -> Assembunny {
        Assembunny(vec![
            Instruction::Cpy {
                from: Argument::Literal(41),
                into: RegisterId::A,
            },
            Instruction::Inc(RegisterId::A),
            Instruction::Inc(RegisterId::A),
            Instruction::Dec(RegisterId::A),
            Instruction::Jnz {
                condition: Argument::Reference(RegisterId::A),
                jump_offset: 2,
            },
            Instruction::Dec(RegisterId::A),
        ])
    }

    #[test]
    fn run_program_test() {
        let mut runtime_environment = RuntimeEnvironment::load_assembunny(Assembunny(vec![
            Instruction::Inc(RegisterId::A),
            Instruction::Inc(RegisterId::B),
            Instruction::Inc(RegisterId::C),
            Instruction::Inc(RegisterId::D),
        ]));

        runtime_environment.run_program();

        assert_eq!(4, runtime_environment.ip);
    }

    #[test]
    fn execute_next_instruction_test_cpy_literal() {
        let mut runtime_environment = runtime_environment_with_instruction(Instruction::Cpy {
            from: Argument::Literal(5),
            into: RegisterId::A,
        });

        runtime_environment.execute_next_instruction();

        assert_eq!(
            5,
            runtime_environment.registers.register_value(RegisterId::A)
        );
    }

    #[test]
    fn execute_next_instruction_test_cpy_reference() {
        let mut runtime_environment = runtime_environment_with_instruction(Instruction::Cpy {
            from: Argument::Reference(RegisterId::B),
            into: RegisterId::A,
        });
        *runtime_environment
            .registers
            .register_value_mut(RegisterId::B) = 5;

        runtime_environment.execute_next_instruction();

        assert_eq!(
            5,
            runtime_environment.registers.register_value(RegisterId::A)
        );
    }

    #[test]
    fn execute_next_instruction_test_inc() {
        let mut runtime_environment =
            runtime_environment_with_instruction(Instruction::Inc(RegisterId::A));

        runtime_environment.execute_next_instruction();

        assert_eq!(
            1,
            runtime_environment.registers.register_value(RegisterId::A)
        );
    }

    #[test]
    fn execute_next_instruction_test_dec() {
        let mut runtime_environment =
            runtime_environment_with_instruction(Instruction::Dec(RegisterId::A));

        runtime_environment.execute_next_instruction();

        assert_eq!(
            -1,
            runtime_environment.registers.register_value(RegisterId::A)
        );
    }

    #[test]
    fn execute_next_instruction_test_jnz_condition_true() {
        let condition_register_id = RegisterId::A;
        let jump_offset = 5;
        let mut runtime_environment = runtime_environment_with_instruction(Instruction::Jnz {
            condition: Argument::Reference(condition_register_id),
            jump_offset,
        });
        *runtime_environment
            .registers
            .register_value_mut(condition_register_id) = 1;

        runtime_environment.execute_next_instruction();

        assert_eq!(jump_offset as usize, runtime_environment.ip);
    }

    #[test]
    fn execute_next_instruction_test_jnz_condition_false() {
        let mut runtime_environment = runtime_environment_with_instruction(Instruction::Jnz {
            condition: Argument::Reference(RegisterId::A),
            jump_offset: 5,
        });

        runtime_environment.execute_next_instruction();

        assert_eq!(1, runtime_environment.ip);
    }

    fn runtime_environment_with_instruction(instruction: Instruction) -> RuntimeEnvironment {
        RuntimeEnvironment {
            assembunny: Assembunny(vec![instruction]),
            registers: Registers::new(),
            ip: 0,
        }
    }

    #[rstest]
    #[case(RegisterId::A)]
    #[case(RegisterId::B)]
    #[case(RegisterId::C)]
    #[case(RegisterId::D)]
    fn dereference_argument_test_argument_reference(#[case] register_id: RegisterId) {
        let mut runtime_environment = RuntimeEnvironment::load_assembunny(Assembunny(vec![]));
        *runtime_environment
            .registers
            .register_value_mut(register_id) = 5;

        let argument_value =
            runtime_environment.dereference_argument(Argument::Reference(register_id));

        assert_eq!(5, argument_value);
    }

    #[rstest]
    #[case(5)]
    #[case(-5)]
    fn dereference_argument_test_argument_literal(#[case] word: Word) {
        let mut runtime_environment = RuntimeEnvironment::load_assembunny(Assembunny(vec![]));
        *runtime_environment
            .registers
            .register_value_mut(RegisterId::A) = word;

        let argument_value =
            runtime_environment.dereference_argument(Argument::Reference(RegisterId::A));

        assert_eq!(word, argument_value);
    }

    #[rstest]
    #[case(false, 6)]
    #[case(false, 100)]
    #[case(true, 0)]
    #[case(true, 4)]
    fn is_program_running_test(assembunny: Assembunny, #[case] expected: bool, #[case] ip: usize) {
        let runtime_environment = RuntimeEnvironment {
            assembunny,
            registers: Registers::new(),
            ip,
        };

        assert_eq!(expected, runtime_environment.is_program_running());
    }
}
