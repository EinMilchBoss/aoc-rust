use std::{num::ParseIntError, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res, opt, recognize},
    error::{FromExternalError, ParseError},
    sequence::tuple,
    IResult,
};

use super::*;

pub fn parse_instruction<'a, E>(input: &'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    alt((
        parse_instruction_cpy,
        parse_instruction_inc,
        parse_instruction_dec,
        parse_instruction_jnz,
    ))(input)
}

fn parse_instruction_cpy<'a, E>(input: &'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map(
        tuple((
            tag("cpy"),
            char(' '),
            parse_argument,
            char(' '),
            parse_register_id,
        )),
        |(.., from, _, into)| Instruction::Cpy { from, into },
    )(input)
}

fn parse_instruction_inc<'a, E>(input: &'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map(
        tuple((tag("inc"), char(' '), parse_register_id)),
        |(.., register_id)| Instruction::Inc(register_id),
    )(input)
}

fn parse_instruction_dec<'a, E>(input: &'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map(
        tuple((tag("dec"), char(' '), parse_register_id)),
        |(.., register_id)| Instruction::Dec(register_id),
    )(input)
}

fn parse_instruction_jnz<'a, E>(input: &'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map(
        tuple((tag("jnz"), char(' '), parse_argument, char(' '), parse_word)),
        |(.., condition, _, jump_offset)| Instruction::Jnz {
            condition,
            jump_offset,
        },
    )(input)
}

fn parse_argument<'a, E>(input: &'a str) -> IResult<&'a str, Argument, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    alt((parse_argument_literal, parse_argument_reference))(input)
}

fn parse_argument_literal<'a, E>(input: &'a str) -> IResult<&'a str, Argument, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map(parse_word, Argument::Literal)(input)
}

fn parse_argument_reference<'a, E>(input: &'a str) -> IResult<&'a str, Argument, E>
where
    E: ParseError<&'a str>,
{
    map(parse_register_id, Argument::Reference)(input)
}

fn parse_word<'a, E>(input: &'a str) -> IResult<&'a str, Word, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(input)
}

fn parse_register_id<'a, E>(input: &'a str) -> IResult<&'a str, RegisterId, E>
where
    E: ParseError<&'a str>,
{
    let (input, char) = alt((char('a'), char('b'), char('c'), char('d')))(input)?;
    Ok((input, char.into()))
}

#[cfg(test)]
mod tests {
    use nom::{
        error::{convert_error, VerboseError},
        Finish, Parser,
    };
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    fn unwrap_verbose<'a, O>(
        mut parser: impl Parser<&'a str, O, VerboseError<&'a str>>,
        input: &'a str,
    ) -> (&'a str, O) {
        parser
            .parse(input)
            .finish()
            .unwrap_or_else(|error| panic!("{}", convert_error(input, error)))
    }

    #[rstest]
    #[case(Instruction::Cpy { from: Argument::Literal(1), into: RegisterId::A}, "cpy 1 a")]
    #[case(Instruction::Inc(RegisterId::D), "inc d")]
    #[case(Instruction::Dec(RegisterId::C), "dec c")]
    #[case(Instruction::Jnz { condition: Argument::Reference(RegisterId::B), jump_offset: -2}, "jnz b -2")]
    fn parse_instruction_test(#[case] expected: Instruction, #[case] input: &str) {
        let (remaining_input, actual) = unwrap_verbose(parse_instruction, input);

        assert_eq!(expected, actual);
        assert_eq!(remaining_input, "");
    }

    #[rstest]
    #[case(Argument::Reference(RegisterId::A), "a")]
    #[case(Argument::Reference(RegisterId::B), "b")]
    #[case(Argument::Reference(RegisterId::C), "c")]
    #[case(Argument::Reference(RegisterId::D), "d")]
    #[case(Argument::Literal(5), "5")]
    #[case(Argument::Literal(-5), "-5")]
    fn parse_argument_test(#[case] expected: Argument, #[case] input: &str) {
        let (remaining_input, actual) = unwrap_verbose(parse_argument, input);

        assert_eq!(expected, actual);
        assert_eq!(remaining_input, "");
    }

    #[rstest]
    #[case(0, "0")]
    #[case(5, "5")]
    #[case(10, "10")]
    #[case(-5, "-5")]
    #[case(-10, "-10")]
    fn parse_word_test(#[case] expected: Word, #[case] input: &str) {
        let (remaining_input, actual) = unwrap_verbose(parse_word, input);

        assert_eq!(expected, actual);
        assert_eq!(remaining_input, "");
    }

    #[rstest]
    #[case(RegisterId::A, "a")]
    #[case(RegisterId::B, "b")]
    #[case(RegisterId::C, "c")]
    #[case(RegisterId::D, "d")]
    fn parse_register_id_test_valid_register_ids(
        #[case] expected: RegisterId,
        #[case] input: &str,
    ) {
        let (remaining_input, actual) = unwrap_verbose(parse_register_id, input);

        assert_eq!(expected, actual);
        assert_eq!(remaining_input, "");
    }

    #[rstest]
    #[case("A")]
    #[case("e")]
    #[should_panic]
    fn parse_register_id_test_invalid_register_ids(#[case] input: &str) {
        parse_register_id::<VerboseError<&str>>(input).unwrap();
    }
}
