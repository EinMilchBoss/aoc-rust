use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::{map_res, opt, recognize},
    multi::many0,
    sequence::separated_pair,
    Finish, IResult,
};

use crate::instruction::{Command, Coordinate, CoordinatePair, Instruction};

pub fn parse_instructions(input: &str) -> Result<Vec<Instruction>, nom::error::Error<&str>> {
    let (_, instructions) = many0(parse_instruction)(input).finish()?;
    Ok(instructions)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (command, coordinate_pair)) =
        separated_pair(parse_command, char(' '), parse_coordinate_pair)(input)?;
    let (input, _) = opt(newline)(input)?;
    Ok((
        input,
        Instruction {
            command,
            coordinate_pair,
        },
    ))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, command_value) = alt((tag("turn on"), tag("turn off"), tag("toggle")))(input)?;
    Ok((input, command_value.into()))
}

fn parse_coordinate_pair(input: &str) -> IResult<&str, CoordinatePair> {
    let (input, (from_coordinate, to_coordinate)) =
        separated_pair(parse_coordinate, tag(" through "), parse_coordinate)(input)?;
    Ok((input, CoordinatePair(from_coordinate, to_coordinate)))
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (input, (x, y)) = separated_pair(parse_digit, char(','), parse_digit)(input)?;
    Ok((input, Coordinate { x, y }))
}

fn parse_digit(input: &str) -> IResult<&str, u16> {
    let (input, number) = map_res(recognize(digit1), str::parse)(input)?;
    Ok((input, number))
}
