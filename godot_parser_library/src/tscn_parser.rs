use std::collections::HashMap;
use std::str::FromStr;
use nom::bytes::complete::{is_not, tag, take_while};
use nom::character::complete;
use nom::character::complete::newline;
use nom::combinator::map_opt;
use nom::error::ParseError;
use nom::IResult;
use nom::multi::{count, many0, separated_list0};
use nom::sequence::{delimited, separated_pair};
use godot_data::tscn_file::{TSCNFile, Tag, TagType};
use godot_data::values::GodotValue;
use crate::data::values::parse_godot_value;

type TagPair = (String, GodotValue);

fn read_attribute(v: &str) -> IResult<&str, TagPair> {
    let (remain, (key, val)) = separated_pair(is_not("="), tag("="), parse_godot_value)(v)?;
    Ok((remain, (String::from_str(key).unwrap(), val)))
}

fn read_property(str: &str) -> IResult<&str, TagPair> {
    let (remain, (key, val)) = separated_pair(is_not("\n "), tag(" = "), parse_godot_value)(str)?;
    let (remain, _) = newline(remain)?;
    Ok((remain, (String::from_str(key).unwrap(), val)))
}

fn read_tag(str: &str, set_tag: Option<TagType>) -> IResult<&str, Tag> {
    let map_tag = map_opt(is_not(" "), |s| TagType::from_str(s).ok());
    let parse_tag_name_and_attrs = separated_pair(map_tag, tag(" "), is_not("]"));
    let (remain, (tag_type, attrs_str)): (&str, (TagType, &str)) = delimited(
        complete::char('['),
        parse_tag_name_and_attrs,
        complete::char(']'),
    )(str)?;
    if set_tag.is_some() && set_tag.unwrap() != tag_type {
        return Err(nom::Err::Error(nom::error::Error::from_error_kind(
            "invalid tag type",
            nom::error::ErrorKind::Fail,
        )));
    }

    let (_, attrs): (&str, Vec<TagPair>) = separated_list0(tag(" "), read_attribute)(attrs_str)?;
    let (remain, props): (&str, Vec<TagPair>) = {
        if tag_type != TagType::ExtResource && tag_type != TagType::GdScene {
            let (remain, _) = newline(remain)?;
            many0(read_property)(remain)?
        } else {
            (remain, Vec::new())
        }
    };
    let (remain, _) = take_while(|c: char| c == '\n')(remain)?;

    Ok((
        remain,
        Tag {
            _type: tag_type,
            attrs: attrs.into_iter().collect(),
            props: props.into_iter().collect(),
        },
    ))
}

fn read_tag_parse(str: &str) -> IResult<&str, Tag> {
    read_tag(str, None)
}

pub fn parse_tscn_file(str: &str) -> IResult<&str, TSCNFile> {
    let (remain, header) = read_tag(str, Some(TagType::GdScene))?;

    let load_steps = header.attrs.get("load_steps")
        .and_then(|v| if let GodotValue::Integer(n) = v { Some(*n) } else { None })
        .unwrap_or(1);
    let (remain, ext_resources_vec_tags): (&str, Vec<Tag>) =
        count(read_tag_parse, (load_steps - 1) as usize)(remain)?;
    let mut ext_resources = HashMap::new();
    let mut sub_resources = HashMap::new();
    for tag in ext_resources_vec_tags {
        match tag._type {
            TagType::ExtResource => {
                if let Some(GodotValue::String(id)) = tag.attrs.get("id") {
                    ext_resources.entry(id.clone()).or_insert(tag);
                }
            }
            TagType::SubResource => {
                if let Some(GodotValue::String(id)) = tag.attrs.get("id") {
                    sub_resources.entry(id.clone()).or_insert(tag);
                }
            }
            _ => {}
        }
    }
    let (remain, tags): (&str, Vec<Tag>) = many0(read_tag_parse)(remain)?;
    let mut nodes = Vec::new();
    for tag in tags {
        if let TagType::Node = tag._type {
            nodes.push(tag)
        }
    }

    Ok((
        remain,
        TSCNFile {
            header,
            ext_resources,
            sub_resources,
            nodes,
        },
    ))
}

#[cfg(test)]
mod tests {
    use godot_data::nanoserde::SerJson;
    use crate::tscn_parser::parse_tscn_file;

    #[test]
    fn test_parse_tscn() {
        let input = r#"[gd_scene format=3 uid="uid://lrpk7b420cd7"]

[node name="Game" type="Node2D"]
        "#;
        let (input, tscn) = parse_tscn_file(input).unwrap();
        println!("{}", tscn.serialize_json());
    }
}