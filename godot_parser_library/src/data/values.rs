use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take, take_while1};
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::combinator::{map, opt};
use nom::error::ParseError;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use godot_data::values::GodotValue;

fn quotes_str(s: &str) -> IResult<&str, &str> {
    let (_, parts): (&str, Vec<&str>) = delimited(
        tag("\""),
        many0(alt((tag("\\\""), is_not("\\\"")))),
        tag("\""),
    )(s)?;
    let len = parts.join("").chars().count();
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

fn mf64_1(s: &str) -> IResult<&str, f64> {
    alt((mf64, map(complete::i64, |v: i64| v as f64)))(s)
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


fn node_path(s: &str) -> IResult<&str, String> {
    map(
        delimited(tag("NodePath(\""), is_not("\""), tag("\")")),
        |s: &str| s.to_string(),
    )(s)
}

fn vec2(s: &str) -> IResult<&str, (f64, f64)> {
    let (remain, (x, y)) = delimited(
        tag("Vector2("),
        separated_pair(mf64_1, tag(", "), mf64_1),
        tag(")"),
    )(s)?;
    Ok((remain, (x, y)))
}

fn rect2(s: &str) -> IResult<&str, (f64, f64, f64, f64)> {
    let (remain, (x1, _, y1, _, x2, _, y2)) = delimited(
        tag("Rect2("),
        tuple((
            mf64_1,
            tag(", "),
            mf64_1,
            tag(", "),
            mf64_1,
            tag(", "),
            mf64_1,
        )),
        tag(")"),
    )(s)?;

    Ok((remain, (x1, y1, x2, y2)))
}

fn ext_resource(s: &str) -> IResult<&str, String> {
    map(
        delimited(tag("ExtResource(\""), is_not("\""), tag("\")")),
        |s: &str| s.to_string(),
    )(s)
}

fn sub_resource(s: &str) -> IResult<&str, String> {
    map(
        delimited(tag("SubResource(\""), is_not("\""), tag("\")")),
        |s: &str| s.to_string(),
    )(s)
}

fn color(s: &str) -> IResult<&str, (f64, f64, f64, f64)> {
    let (remain, color) =
        delimited(tag("Color("), separated_list0(tag(", "), mf64_1), tag(")"))(s)?;
    Ok((
        remain,
        (
            color.first().cloned().unwrap_or(0.0),
            color.get(1).cloned().unwrap_or(0.0),
            color.get(2).cloned().unwrap_or(0.0),
            color.get(3).cloned().unwrap_or(0.0),
        ),
    ))
}

fn array(s: &str) -> IResult<&str, Vec<GodotValue>> {
    let (remain, _) = tag("[")(s)?;
    let (remain, list) = separated_list0(tag(", "), parse_godot_value)(remain)?;
    let (remain, _) = tag("]")(remain)?;
    Ok((remain, list))
}

fn dictionary(s: &str) -> IResult<&str, Vec<(String, GodotValue)>> {
    let (remain, _) = tag("{\n")(s)?;
    let (remain, list) = separated_list0(tag(",\n"), separated_pair(map(quotes_str, |s: &str| s.to_string()), tag(": "), parse_godot_value))(remain)?;
    let (remain, _) = tag("\n}")(remain)?;
    Ok((remain, list))
}

pub fn parse_godot_value(input: &str) -> IResult<&str, GodotValue> {
    alt((
        map(quotes_str, |s: &str| GodotValue::String(s.to_string())),
        map(mf64, |s: f64| GodotValue::Float(s)),
        map(complete::i64, |s: i64| GodotValue::Integer(s)),
        map(boolean, |s: bool| GodotValue::Boolean(s)),
        map(parse_packed_string_array, |s| GodotValue::PackedStringArray(s)),
        map(node_path, |s| GodotValue::NodePath(s)),
        map(vec2, |s| GodotValue::Vector2(s)),
        map(rect2, |s| GodotValue::Rect2(s)),
        map(ext_resource, |s| GodotValue::ExtResourceLink(s)),
        map(sub_resource, |s| GodotValue::SubResourceLink(s)),
        map(color, |s| GodotValue::Color(s)),
        map(array, |s| GodotValue::Array(s)),
        map(dictionary, |s| GodotValue::Dictionary(s)),
    ))(input)
}