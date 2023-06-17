mod instruction;
mod registers;
mod runtime;

use instruction::{Assembunny, AssembunnyParseError, RegisterId, Word};
use util::std::*;

use crate::runtime::RuntimeEnvironment;

fn main() {
    let input = read_file(InputFile::Actual, Year("2016"), Day("12"))
        .expect("Input file could not be read.");
    let assembunny = input.parse().unwrap_or_else(|error: AssembunnyParseError| {
        panic!(
            "{}. Verbose error description: {}",
            error,
            error.verbose_error_description()
        );
    });

    println!("Part 1: {}", part_1(&assembunny));
    println!("Part 2: {}", part_2(&assembunny));
}

fn part_1(assembunny: &Assembunny) -> Word {
    // execute instruction of the ip
    // - manipulate ip to the next instruction
    // -
    let mut runtime_environment = RuntimeEnvironment::load_assembunny(assembunny.clone());
    runtime_environment.run_assembunny();
    runtime_environment.register_a()
}

fn part_2(assembunny: &Assembunny) -> Word {
    // execute instruction of the ip
    // - manipulate ip to the next instruction
    // -
    let mut runtime_environment = RuntimeEnvironment::load_assembunny(assembunny.clone());
    *runtime_environment.register_mut(RegisterId::C) = 1;
    runtime_environment.run_assembunny();
    runtime_environment.register_a()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::instruction::*;

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
