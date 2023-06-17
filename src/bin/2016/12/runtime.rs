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

    pub fn register_mut(&mut self, register_id: RegisterId) -> &mut Word {
        self.registers.get_mut(register_id)
    }

    pub fn register_a(&self) -> Word {
        self.registers.get(RegisterId::A)
    }

    pub fn run_program(&mut self) {
        loop {
            if self.is_ip_past_last_instruction() {
                break;
            }
            self.execute();
        }
    }

    fn execute(&mut self) {
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
        *self.registers.get_mut(into) = self.argument_value(from);
        self.ip += 1;
    }

    fn execute_inc_instruction(&mut self, register_id: RegisterId) {
        self.registers.increment(register_id);
        self.ip += 1;
    }

    fn execute_dec_instruction(&mut self, register_id: RegisterId) {
        self.registers.decrement(register_id);
        self.ip += 1;
    }

    fn execute_jnz_instruction(&mut self, condition: Argument, jump_offset: Word) {
        if self.argument_value(condition) != 0 {
            let ip = self.ip as isize;
            let jump_offset = jump_offset as isize;
            self.ip = ip.saturating_add(jump_offset) as usize;
        } else {
            self.ip += 1
        }
    }

    fn argument_value(&self, argument: Argument) -> Word {
        match argument {
            Argument::Literal(value) => value,
            Argument::Reference(register_id) => self.registers.get(register_id),
        }
    }

    fn is_ip_past_last_instruction(&self) -> bool {
        let Assembunny(instructions) = &self.assembunny;
        self.ip >= instructions.len()
    }
}

#[cfg(test)]
mod runtime_tests {
    use rstest::{fixture, rstest};

    use crate::instruction::*;

    use super::*;

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

    #[rstest]
    #[case(Instruction::Cpy {
        from: Argument::Literal(5),
        into: RegisterId::A,
    })]
    #[case(Instruction::Cpy {
        from: Argument::Reference(RegisterId::B),
        into: RegisterId::A,
    })]
    fn execute_test_cpy(#[case] instruction: Instruction) {
        let mut runtime_environment = runtime_environment_with_instruction(instruction);
        *runtime_environment.registers.get_mut(RegisterId::B) = 5;

        runtime_environment.execute();

        assert_eq!(5, runtime_environment.registers.get(RegisterId::A));
    }

    #[test]
    fn execute_test_inc() {
        let mut runtime_environment =
            runtime_environment_with_instruction(Instruction::Inc(RegisterId::A));

        runtime_environment.execute();

        assert_eq!(1, runtime_environment.registers.get(RegisterId::A));
    }

    #[test]
    fn execute_test_dec() {
        let mut runtime_environment =
            runtime_environment_with_instruction(Instruction::Dec(RegisterId::A));

        runtime_environment.execute();

        assert_eq!(-1, runtime_environment.registers.get(RegisterId::A));
    }

    #[test]
    fn execute_test_jnz_condition_true() {
        let condition_register_id = RegisterId::A;
        let jump_offset = 5;
        let mut runtime_environment = runtime_environment_with_instruction(Instruction::Jnz {
            condition: Argument::Reference(condition_register_id),
            jump_offset,
        });
        *runtime_environment.registers.get_mut(condition_register_id) = 1;

        runtime_environment.execute();

        assert_eq!(jump_offset as usize, runtime_environment.ip);
    }

    #[test]
    fn execute_test_jnz_condition_false() {
        let mut runtime_environment = runtime_environment_with_instruction(Instruction::Jnz {
            condition: Argument::Reference(RegisterId::A),
            jump_offset: 5,
        });

        runtime_environment.execute();

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
    fn argument_value_test_argument_reference(#[case] register_id: RegisterId) {
        let mut runtime_environment = RuntimeEnvironment::load_assembunny(Assembunny(vec![]));
        *runtime_environment.registers.get_mut(register_id) = 5;

        let argument_value = runtime_environment.argument_value(Argument::Reference(register_id));

        assert_eq!(5, argument_value);
    }

    #[rstest]
    #[case(5)]
    #[case(-5)]
    fn argument_value_test_argument_literal(#[case] word: Word) {
        let mut runtime_environment = RuntimeEnvironment::load_assembunny(Assembunny(vec![]));
        *runtime_environment.registers.get_mut(RegisterId::A) = word;

        let argument_value = runtime_environment.argument_value(Argument::Reference(RegisterId::A));

        assert_eq!(word, argument_value);
    }

    #[rstest]
    #[case(6)]
    #[case(100)]
    fn is_ip_past_last_instruction_test_true(assembunny: Assembunny, #[case] ip: usize) {
        let runtime_environment = runtime_environment_with_assembunny_and_ip(assembunny, ip);

        assert!(runtime_environment.is_ip_past_last_instruction());
    }

    #[rstest]
    #[case(0)]
    #[case(4)]
    fn is_ip_past_last_instruction_test_false(assembunny: Assembunny, #[case] ip: usize) {
        let runtime_environment = runtime_environment_with_assembunny_and_ip(assembunny, ip);

        assert!(!runtime_environment.is_ip_past_last_instruction());
    }

    fn runtime_environment_with_assembunny_and_ip(
        assembunny: Assembunny,
        ip: usize,
    ) -> RuntimeEnvironment {
        RuntimeEnvironment {
            assembunny,
            registers: Registers::new(),
            ip,
        }
    }
}
