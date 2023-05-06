use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::{cut, map_res},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

type SignalSize = u16;

#[derive(Debug, PartialEq, Eq)]
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
        shift: SignalSize,
    },
    RightShift {
        value: CommandInput,
        shift: SignalSize,
    },
    Not(CommandInput),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandInput {
    Signal(Signal),
    Wire(Wire),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signal(pub SignalSize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wire(pub String);

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
    move |input: &str| separated_pair(parse_command_input, tag(separator), parse_u16)(input)
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
    let (input, value) = parse_u16(input)?;
    Ok((input, Signal(value)))
}

fn parse_wire(input: &str) -> IResult<&str, Wire, nom::error::VerboseError<&str>> {
    let (input, value) = alpha1(input)?;
    Ok((input, Wire(value.to_string())))
}

fn parse_u16(input: &str) -> IResult<&str, u16, nom::error::VerboseError<&str>> {
    map_res(digit1, str::parse)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_input() {
        let signal = CommandInput::Signal(Signal(15));
        assert_eq!(signal, parse_command_input("15").unwrap().1);

        let signal = CommandInput::Wire(Wire("abc".to_string()));
        assert_eq!(signal, parse_command_input("abc").unwrap().1);
    }

    #[test]
    fn test_parse_command_and() {
        let expected = Command::And(
            CommandInput::Signal(Signal(15)),
            CommandInput::Wire(Wire("ab".to_string())),
        );
        assert_eq!(expected, parse_command_and("15 AND ab").unwrap().1);

        let expected = Command::And(
            CommandInput::Wire(Wire("ab".to_string())),
            CommandInput::Signal(Signal(15)),
        );
        assert_eq!(expected, parse_command_and("ab AND 15").unwrap().1);

        let expected = Command::And(
            CommandInput::Wire(Wire("ab".to_string())),
            CommandInput::Wire(Wire("cd".to_string())),
        );
        assert_eq!(expected, parse_command_and("ab AND cd").unwrap().1);
    }

    #[test]
    fn test_parse_instruction() {
        let expected = Instruction {
            command: Command::And(
                CommandInput::Signal(Signal(15)),
                CommandInput::Wire(Wire("ab".to_string())),
            ),
            destination: Wire("bc".to_string()),
        };
        assert_eq!(expected, parse_instruction("15 AND ab -> bc").unwrap().1);
    }
}
