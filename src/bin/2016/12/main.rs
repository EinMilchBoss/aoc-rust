mod instruction;
mod registers;
mod runtime;

use instruction::{Argument, AssembunnyParseError, RegisterId, Word};
use util::std::*;

use crate::runtime::Runtime;

fn main() {
    let input = read_file(InputFile::Actual, Year("2016"), Day("12"))
        .expect("Input file could not be read.");
    let assembunny = input.parse().unwrap_or_else(|error: AssembunnyParseError| {
        panic!(
            "{}. Verbose error description: {}",
            error,
            error.verbose_error_description()
        )
    });
    let runtime: Runtime = Runtime::load_assembunny(assembunny);
    dbg!(runtime);
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;

    use super::*;

    // #[test]
    // fn parse_runtime_test() {
    //     let input = ["cpy 10 a", "inc b", "dec c", "jnz d -1"].join("\n");
    //     let expected = Runtime {
    //         instructions: vec![
    //             Instruction::Cpy {
    //                 from: Argument::Literal(10),
    //                 into: Argument::Reference('a'),
    //             },
    //             Instruction::Inc('b'),
    //             Instruction::Dec('c'),
    //             Instruction::Jnz {
    //                 condition: Argument::Reference('d'),
    //                 jump_offset: -1,
    //             },
    //         ],
    //         registers: Registers(HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0)])),
    //         ip: 0,
    //     };

    //     assert_eq!(expected, input.parse().unwrap());
    // }
}
