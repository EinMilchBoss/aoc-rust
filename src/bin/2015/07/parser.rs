use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::{cut, map_res},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

pub type SignalType = u16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub command: Command,
    pub destination: Wire,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Set(CommandInput),
    And(CommandInput, CommandInput),
    Or(CommandInput, CommandInput),
    LeftShift {
        value: CommandInput,
        shift: SignalType,
    },
    RightShift {
        value: CommandInput,
        shift: SignalType,
    },
    Not(CommandInput),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandInput {
    Signal(Signal),
    Wire(Wire),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signal(pub SignalType);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wire(pub String);

impl Instruction {
    pub fn new(command: Command, destination: Wire) -> Self {
        Self {
            command,
            destination,
        }
    }
}

impl Wire {
    pub fn from_id(id: &str) -> Self {
        Self(id.to_string())
    }
}

pub fn parse_instructions(input: &str) -> Result<Vec<Instruction>, nom::error::VerboseError<&str>> {
    separated_list0(newline, cut(parse_instruction))(input)
        .finish()
        .map(|(_, instructions)| instructions)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction, nom::error::VerboseError<&str>> {
    let (input, (command, destination)) = separated_pair(
        alt((
            parse_command_and,
            parse_command_or,
            parse_command_left_shift,
            parse_command_right_shift,
            parse_command_not,
            parse_command_set,
        )),
        tag(" -> "),
        parse_wire,
    )(input)?;
    Ok((
        input,
        Instruction {
            command,
            destination,
        },
    ))
}

fn parse_command_set(input: &str) -> IResult<&str, Command, nom::error::VerboseError<&str>> {
    let (input, command_input) = parse_command_input(input)?;
    Ok((input, Command::Set(command_input)))
}

fn parse_command_and(input: &str) -> IResult<&str, Command, nom::error::VerboseError<&str>> {
    let (input, (left, right)) = parse_separated_command_input_pair(" AND ")(input)?;
    Ok((input, Command::And(left, right)))
}

fn parse_command_or(input: &str) -> IResult<&str, Command, nom::error::VerboseError<&str>> {
    let (input, (left, right)) = parse_separated_command_input_pair(" OR ")(input)?;
    Ok((input, Command::Or(left, right)))
}

fn parse_command_left_shift(input: &str) -> IResult<&str, Command, nom::error::VerboseError<&str>> {
    let (input, (value, shift)) = parse_shift_command(" LSHIFT ")(input)?;
    Ok((input, Command::LeftShift { value, shift }))
}

fn parse_command_right_shift(
    input: &str,
) -> IResult<&str, Command, nom::error::VerboseError<&str>> {
    let (input, (value, shift)) = parse_shift_command(" RSHIFT ")(input)?;
    Ok((input, Command::RightShift { value, shift }))
}

fn parse_command_not(input: &str) -> IResult<&str, Command, nom::error::VerboseError<&str>> {
    let (input, command_input) = preceded(tag("NOT "), parse_command_input)(input)?;
    Ok((input, Command::Not(command_input)))
}

fn parse_separated_command_input_pair(
    separator: &str,
) -> impl Fn(&str) -> IResult<&str, (CommandInput, CommandInput), nom::error::VerboseError<&str>> + '_
{
    move |input: &str| {
        separated_pair(parse_command_input, tag(separator), parse_command_input)(input)
    }
}

fn parse_shift_command(
    separator: &str,
) -> impl Fn(&str) -> IResult<&str, (CommandInput, u16), nom::error::VerboseError<&str>> + '_ {
    move |input: &str| separated_pair(parse_command_input, tag(separator), parse_signal_type)(input)
}

fn parse_command_input(input: &str) -> IResult<&str, CommandInput, nom::error::VerboseError<&str>> {
    let (input, command_input) =
        alt((parse_command_input_signal, parse_command_input_wire))(input)?;
    Ok((input, command_input))
}

fn parse_command_input_signal(
    input: &str,
) -> IResult<&str, CommandInput, nom::error::VerboseError<&str>> {
    let (input, signal) = parse_signal(input)?;
    Ok((input, CommandInput::Signal(signal)))
}

fn parse_command_input_wire(
    input: &str,
) -> IResult<&str, CommandInput, nom::error::VerboseError<&str>> {
    let (input, wire) = parse_wire(input)?;
    Ok((input, CommandInput::Wire(wire)))
}

fn parse_signal(input: &str) -> IResult<&str, Signal, nom::error::VerboseError<&str>> {
    let (input, value) = parse_signal_type(input)?;
    Ok((input, Signal(value)))
}

fn parse_wire(input: &str) -> IResult<&str, Wire, nom::error::VerboseError<&str>> {
    let (input, value) = alpha1(input)?;
    Ok((input, Wire(value.to_string())))
}

fn parse_signal_type(input: &str) -> IResult<&str, SignalType, nom::error::VerboseError<&str>> {
    map_res(digit1, str::parse)(input)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        Instruction::new(Command::Set(CommandInput::Signal(Signal(123))), Wire::from_id("x")),
        "123 -> x"
    )]
    #[case(
        Instruction::new(
            Command::Set(CommandInput::Wire(Wire::from_id("x"))),
            Wire::from_id("y")
        ),
        "x -> y"
    )]
    #[case(
        Instruction::new(
            Command::And(CommandInput::Signal(Signal(123)), CommandInput::Signal(Signal(456))),
            Wire::from_id("z"),
        ),
        "123 AND 456 -> z"
    )]
    #[case(
        Instruction::new(
            Command::And(
                CommandInput::Wire(Wire::from_id("x")),
                CommandInput::Wire(Wire::from_id("y"))
            ),
            Wire::from_id("z"),
        ),
        "x AND y -> z"
    )]
    #[case(
        Instruction::new(
            Command::Or(CommandInput::Signal(Signal(123)), CommandInput::Signal(Signal(456))),
            Wire::from_id("z"),
        ),
        "123 OR 456 -> z"
    )]
    #[case(
        Instruction::new(
            Command::Or(
                CommandInput::Wire(Wire::from_id("x")),
                CommandInput::Wire(Wire::from_id("y"))
            ),
            Wire::from_id("z"),
        ),
        "x OR y -> z"
    )]
    #[case(
        Instruction::new(
            Command::LeftShift {
                value: CommandInput::Signal(Signal(123)),
                shift: 2
            },
            Wire::from_id("z"),
        ),
        "123 LSHIFT 2 -> z"
    )]
    #[case(
        Instruction::new(
            Command::LeftShift {
                value: CommandInput::Wire(Wire::from_id("x")),
                shift: 2
            },
            Wire::from_id("z"),
        ),
        "x LSHIFT 2 -> z"
    )]
    #[case(
        Instruction::new(
            Command::RightShift {
                value: CommandInput::Signal(Signal(123)),
                shift: 2
            },
            Wire::from_id("z"),
        ),
        "123 RSHIFT 2 -> z"
    )]
    #[case(
        Instruction::new(
            Command::RightShift {
                value: CommandInput::Wire(Wire::from_id("x")),
                shift: 2
            },
            Wire::from_id("z"),
        ),
        "x RSHIFT 2 -> z"
    )]
    #[case(
        Instruction::new(Command::Not(CommandInput::Signal(Signal(123))), Wire::from_id("z"),),
        "NOT 123 -> z"
    )]
    #[case(
        Instruction::new(
            Command::Not(CommandInput::Wire(Wire::from_id("x"))),
            Wire::from_id("z"),
        ),
        "NOT x -> z"
    )]
    fn parse_instruction_test_all_valid_cases(#[case] expected: Instruction, #[case] input: &str) {
        let (_, actual) = parse_instruction(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("123")]
    #[case("x")]
    #[case("NOT")]
    #[case("Invalid Input")]
    fn parse_instruction_test_if_err_for_invalid_case(#[case] input: &str) {
        assert_eq!(parse_instruction(input).is_err(), true);
    }

    #[test]
    fn parse_instructions_test_if_err_for_invalid_input() {
        let input = vec!["123 -> x", "Something invalid", "NOT x -> y"].join("\n");

        assert_eq!(parse_instructions(&input).is_err(), true);
    }
}
