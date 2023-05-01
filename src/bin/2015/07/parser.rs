use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::map_res,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Debug)]
pub struct Instruction {
    pub command: Command,
    pub destination: Wire,
}

#[derive(Debug)]
pub enum Command {
    Set(Signal),
    And(Wire, Wire),
    Or(Wire, Wire),
    LeftShift(Wire, u16),
    RightShift(Wire, u16),
    Not(Wire),
}

#[derive(Debug, Clone, Copy)]
pub struct Signal(pub u16);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Wire(pub String);

pub fn parse_instructions(input: &str) -> Result<Vec<Instruction>, nom::error::Error<&str>> {
    separated_list0(newline, parse_instruction)(input)
        .finish()
        .map(|(_, instructions)| instructions)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (command, destination)) = separated_pair(
        alt((
            parse_set,
            parse_and,
            parse_or,
            parse_left_shift,
            parse_right_shift,
            parse_not,
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

fn parse_set(input: &str) -> IResult<&str, Command> {
    let (input, signal) = parse_signal(input)?;
    Ok((input, Command::Set(signal)))
}

fn parse_and(input: &str) -> IResult<&str, Command> {
    let (input, (left, right)) = parse_gate_command(" AND ")(input)?;
    Ok((input, Command::And(left, right)))
}

fn parse_or(input: &str) -> IResult<&str, Command> {
    let (input, (left, right)) = parse_gate_command(" OR ")(input)?;
    Ok((input, Command::Or(left, right)))
}

fn parse_left_shift(input: &str) -> IResult<&str, Command> {
    let (input, (left, right)) = parse_shift_command(" LSHIFT ")(input)?;
    Ok((input, Command::LeftShift(left, right)))
}

fn parse_right_shift(input: &str) -> IResult<&str, Command> {
    let (input, (left, right)) = parse_shift_command(" RSHIFT ")(input)?;
    Ok((input, Command::RightShift(left, right)))
}

fn parse_not(input: &str) -> IResult<&str, Command> {
    let (input, wire) = preceded(tag("NOT "), parse_wire)(input)?;
    Ok((input, Command::Not(wire)))
}

fn parse_gate_command(separator: &str) -> impl Fn(&str) -> IResult<&str, (Wire, Wire)> + '_ {
    move |input: &str| separated_pair(parse_wire, tag(separator), parse_wire)(input)
}

fn parse_shift_command(separator: &str) -> impl Fn(&str) -> IResult<&str, (Wire, u16)> + '_ {
    move |input: &str| separated_pair(parse_wire, tag(separator), parse_u16)(input)
}

fn parse_signal(input: &str) -> IResult<&str, Signal> {
    let (input, value) = parse_u16(input)?;
    Ok((input, Signal(value)))
}

fn parse_wire(input: &str) -> IResult<&str, Wire> {
    let (input, value) = alpha1(input)?;
    Ok((input, Wire(value.to_string())))
}

fn parse_u16(input: &str) -> IResult<&str, u16> {
    map_res(digit1, str::parse)(input)
}
