use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take, take_while1};
use nom::character::complete;
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::{many0, separated_list0};
use nom::sequence::delimited;
use nom::IResult;
use godot_data::values::GodotValue;

fn quotes_str(s: &str) -> IResult<&str, &str> {
    let (_, parts): (&str, Vec<&str>) = delimited(
        tag("\""),
        many0(alt((tag("\\\""), is_not("\\\"")))),
        tag("\""),
    )(s)?;
    let len = parts.join("").len();
    let (remain, out) = delimited(tag("\""), take(len), tag("\""))(s)?;
    Ok((remain, out))
}

fn mf64(s: &str) -> IResult<&str, f64> {
    let (remain, parsed_number) =
        take_while1(|c: char| c.is_ascii_digit() || c == '.' || c == '-')(s)?;
    if !parsed_number.contains('.') {
        return Err(nom::Err::Error(nom::error::Error::from_error_kind(
            s,
            nom::error::ErrorKind::Digit,
        )));
    }

    Ok((remain, parsed_number.parse().unwrap()))
}

fn boolean(s: &str) -> IResult<&str, bool> {
    alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(s)
}

fn parse_packed_string_array(input: &str) -> IResult<&str, Vec<String>> {
    let (input, _) = tag("PackedStringArray(")(input)?;
    let (input, list) = separated_list0(tag(", "), map(quotes_str, |s| s.to_string()))(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, list))
}

pub fn parse_godot_value(input: &str) -> IResult<&str, GodotValue> {
    alt((
        map(quotes_str, |s: &str| GodotValue::String(s.to_string())),
        map(mf64, |s: f64| GodotValue::Float(s)),
        map(complete::i64, |s: i64| GodotValue::Integer(s)),
        map(boolean, |s: bool| GodotValue::Boolean(s)),
        map(parse_packed_string_array, |s| GodotValue::PackedStringArray(s)),
    ))(input)
}