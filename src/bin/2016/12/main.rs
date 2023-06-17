mod assembunny;
mod registers;
mod runtime;

use assembunny::{Assembunny, AssembunnyParseError, RegisterId, Word};
use util::std::*;

use crate::runtime::RuntimeEnvironment;

fn main() {
    let input = read_file(InputFile::Actual, Year("2016"), Day("12"))
        .expect("Input file could not be read.");
    let assembunny = parse_aoc_input_or_panic(&input);

    println!("Part 1: {}", part_1(&assembunny));
    println!("Part 2: {}", part_2(&assembunny));
}

fn parse_aoc_input_or_panic(input: &str) -> Assembunny {
    input.parse().unwrap_or_else(|error: AssembunnyParseError| {
        panic!(
            "{}. Verbose error description: {}",
            error,
            error.verbose_error_description()
        );
    })
}

fn part_1(assembunny: &Assembunny) -> Word {
    let mut runtime_environment = RuntimeEnvironment::load_assembunny(assembunny.clone());
    runtime_environment.run_program();
    runtime_environment.register_value(RegisterId::A)
}

fn part_2(assembunny: &Assembunny) -> Word {
    let mut runtime_environment = RuntimeEnvironment::load_assembunny_with_initialized_registers(
        assembunny.clone(),
        [0, 0, 1, 0].into(),
    );
    runtime_environment.run_program();
    runtime_environment.register_value(RegisterId::A)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::assembunny::{Argument, Instruction};

    use super::*;

    #[fixture]
    fn aoc_assembunny() -> Assembunny {
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
    fn part_1_test(aoc_assembunny: Assembunny) {
        assert_eq!(42, part_1(&aoc_assembunny));
    }

    #[rstest]
    fn part_2_test(aoc_assembunny: Assembunny) {
        assert_eq!(42, part_2(&aoc_assembunny));
    }
}
