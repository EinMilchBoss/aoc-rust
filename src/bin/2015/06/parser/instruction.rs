use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::{map_res, opt, recognize},
    multi::many0,
    sequence::pair,
    Finish, IResult,
};

use crate::instruction::{Command, Coordinate, CoordinatePair, Instruction};

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (input, x) = map_res(recognize(digit1), str::parse)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = map_res(recognize(digit1), str::parse)(input)?;
    Ok((input, Coordinate { x, y }))
}

fn parse_coordinate_pair(input: &str) -> IResult<&str, CoordinatePair> {
    let (input, from_coordinate) = parse_coordinate(input)?;
    let (input, _) = tag(" through ")(input)?;
    let (input, to_coordinate) = parse_coordinate(input)?;
    Ok((input, CoordinatePair(from_coordinate, to_coordinate)))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, command_value) = alt((tag("turn on"), tag("turn off"), tag("toggle")))(input)?;
    let (input, _) = char(' ')(input)?;
    Ok((input, command_value.into()))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (command, coordinate_pair)) = pair(parse_command, parse_coordinate_pair)(input)?;
    let (input, _) = opt(newline)(input)?;
    Ok((
        input,
        Instruction {
            command,
            coordinate_pair,
        },
    ))
}

pub fn parse_instructions(input: &str) -> Result<Vec<Instruction>, nom::error::Error<&str>> {
    let (_, instructions) = many0(parse_instruction)(input).finish()?;
    Ok(instructions)
}
