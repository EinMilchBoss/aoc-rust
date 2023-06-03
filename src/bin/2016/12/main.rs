use std::collections::HashMap;

use util::std::*;

type Word = i32;
type RegisterId = char;

#[derive(Debug, Clone, PartialEq)]
struct Runtime {
    instructions: Vec<Instruction>,
    registers: Registers,
    ip: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Cpy {
        from: Argument,
        into: Argument,
    },
    Inc(RegisterId),
    Dec(RegisterId),
    Jnz {
        condition: Argument,
        jump_offset: Word,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Argument {
    Literal(Word),
    Reference(RegisterId),
}

#[derive(Debug, Clone, PartialEq)]
struct Registers(HashMap<RegisterId, Word>);

impl Registers {
    fn init() -> Self {
        Self(HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0)]))
    }
}

fn main() {
    let input = read_file(InputFile::Actual, Year("2016"), Day("12"))
        .expect("Input file could not be read.");
    let runtime = parse_runtime(&input);

    dbg!(runtime);
}

fn parse_runtime(input: &str) -> Runtime {
    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("cpy") => {
                    let arguments: Vec<_> = parts.collect();
                    if let &[first, second] = arguments.as_slice() {
                        Instruction::Cpy {
                            from: parse_argument(first),
                            into: parse_argument(second),
                        }
                    } else {
                        panic!(
                            "Expected 2 arguments, {} were given. Arguments: {:?}.",
                            arguments.len(),
                            arguments
                        )
                    }
                }
                Some("inc") => {
                    let argument = parts
                        .next()
                        .expect("Expected 1 argument, 0 were given.")
                        .chars()
                        .next()
                        .unwrap();
                    Instruction::Inc(parse_register_id(argument))
                }
                Some("dec") => {
                    let argument = parts
                        .next()
                        .expect("Expected 1 argument, 0 were given.")
                        .chars()
                        .next()
                        .unwrap();
                    Instruction::Dec(parse_register_id(argument))
                }
                Some("jnz") => {
                    let arguments: Vec<_> = parts.collect();
                    if let &[first, second] = arguments.as_slice() {
                        Instruction::Jnz {
                            condition: parse_argument(first),
                            jump_offset: parse_word(second),
                        }
                    } else {
                        panic!(
                            "Expected 2 arguments, {} were given. Arguments: {:?}.",
                            arguments.len(),
                            arguments
                        )
                    }
                }
                Some(keyword) => panic!("Line `{}` contains invalid keyword `{}`.", line, keyword),
                None => panic!("Line `{}` does not contain a keyword.", line),
            }
        })
        .collect();

    Runtime {
        instructions,
        registers: Registers::init(),
        ip: 0,
    }
}

fn parse_word(string: &str) -> Word {
    match string.parse() {
        Ok(word) => word,
        _ => panic!("Could not parse `{}` into a `Word`.", string),
    }
}

fn parse_instruction(arguments: &[&str]) -> Instruction {
    if let &[first, second] = arguments {
        Instruction::Cpy {
            from: parse_argument(first),
            into: parse_argument(second),
        }
    } else {
        panic!(
            "Expected 2 arguments, {} were given. Arguments: {:?}.",
            arguments.len(),
            arguments
        )
    }
}

fn parse_argument(string: &str) -> Argument {
    match string.parse::<Word>() {
        Ok(word) => Argument::Literal(word),
        _ => parse_reference_argument(string.chars().next()),
    }
}

fn parse_reference_argument(register_id: Option<RegisterId>) -> Argument {
    match register_id {
        Some(register_id) => Argument::Reference(parse_register_id(register_id)),
        None => panic!("Cannot parse argument because there are no characters to parse from."),
    }
}

fn parse_register_id(register_id: char) -> RegisterId {
    if is_valid_register_id(register_id) {
        register_id
    } else {
        panic!("Given register ID `{}` is invalid.", register_id);
    }
}

fn is_valid_register_id(register_id: RegisterId) -> bool {
    ['a', 'b', 'c', 'd'].contains(&register_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_runtime_test() {
        let input = ["cpy 10 a", "inc b", "dec c", "jnz d -1"].join("\n");
        let expected = Runtime {
            instructions: vec![
                Instruction::Cpy {
                    from: Argument::Literal(10),
                    into: Argument::Reference('a'),
                },
                Instruction::Inc('b'),
                Instruction::Dec('c'),
                Instruction::Jnz {
                    condition: Argument::Reference('d'),
                    jump_offset: -1,
                },
            ],
            registers: Registers(HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0)])),
            ip: 0,
        };

        assert_eq!(expected, parse_runtime(&input));
    }
}
