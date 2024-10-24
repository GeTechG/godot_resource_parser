use std::collections::HashMap;
use nom::{
    bytes::complete::{tag},
    character::complete::{line_ending, not_line_ending},
    combinator::opt,
    IResult,
};
use nom::bytes::complete::take_until1;
use nom::character::complete;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{preceded, separated_pair};
use godot_data::project_file::{ProjectFile, GodotFileParameters};
use godot_data::values::GodotValue;
use crate::data::values::parse_godot_value;

fn parse_comment(input: &str) -> IResult<&str, String> {
    let (input, _) = tag(";")(input)?;
    let (input, text) = not_line_ending(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, text.trim().to_string()))
}

fn parse_parameter(input: &str) -> IResult<&str, (String, GodotValue)> {
    let (input, expression) = not_line_ending(input)?;
    let (input, _) = opt(line_ending)(input)?;
    let (_, (name, value)) = separated_pair(take_until1("="), tag("="), parse_godot_value)(expression)?;
    Ok((input, (name.trim().to_string(), value)))
}

fn parse_section(input: &str) -> IResult<&str, (String, GodotFileParameters)> {
    let (input, _) = tag("[")(input)?;
    let (input, name) = take_until1("]")(input)?;
    let (input, _) = tag("]")(input)?;
    let (input, _) = many0(line_ending)(input)?;
    let (input, parameters) = many0(parse_parameter)(input)?;
    let parameters_map = parameters.into_iter().collect::<HashMap<_, _>>();
    let (input, _) = many0(line_ending)(input)?;
    Ok((input, (
        name.trim().to_string(),
        parameters_map
    )))
}

pub fn parse_project_file(input: &str) -> IResult<&str, ProjectFile> {
    let (input, _) = many0(parse_comment)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, config_version) = preceded(tag("config_version="), complete::u32)(input)?;
    let (input, _) = many0(line_ending)(input)?;
    let (input, sections) = map(many0(parse_section), |section| section.into_iter().collect::<HashMap<_, _>>())(input)?;
    Ok((input, ProjectFile {
        config_version,
        sections,
    }))
}

#[cfg(test)]
mod tests {
    use godot_data::nanoserde::{SerBin, SerJson};
    use super::*;

    #[test]
    fn test_parse_comment() {
        let input = r#"; Engine configuration file.
; It's best edited using the editor UI and not directly,
; since the parameters that go here are not all obvious.
;
; Format:
;   [section] ; section goes between []
;   param=value ; assign values to parameters

config_version=5

[application]

config/name="test"
run/main_scene="res://game.tscn"
config/features=PackedStringArray("4.3", "Mobile")
config/icon="res://icon.svg"

[rendering]

renderer/rendering_method="mobile"
"#;
        let (_, godot_file) = parse_project_file(input).unwrap();
        println!("{:?}", godot_file.serialize_json());
    }
}