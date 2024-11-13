use std::collections::HashMap;
use std::str::FromStr;
use nom::bytes::complete::{is_not, tag, take_while};
use nom::character::complete::newline;
use nom::combinator::{map_opt, opt};
use nom::error::ParseError;
use nom::IResult;
use nom::multi::{count, many0, separated_list0};
use nom::sequence::{separated_pair};
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
    let (remain, _) = opt(newline)(remain)?;
    Ok((remain, (String::from_str(key).unwrap(), val)))
}

fn read_tag(str: &str, set_tag: Option<TagType>) -> IResult<&str, Tag> {
    let (str, _) = tag("[")(str)?;
    let (str, tag_type) = map_opt(is_not(" ]"), |s| TagType::from_str(s).ok())(str)?;
    if set_tag.is_some() && set_tag.unwrap() != tag_type {
        return Err(nom::Err::Error(nom::error::Error::from_error_kind(
            "invalid tag type",
            nom::error::ErrorKind::Fail,
        )));
    }
    let (str, _) = opt(tag(" "))(str)?;

    let (str, attrs): (&str, Vec<TagPair>) = separated_list0(tag(" "), read_attribute)(str)?;
    let (str, _) = tag("]")(str)?;
    let (str, props): (&str, Vec<TagPair>) = {
        let (_, none_props) = opt(count(newline, 2))(str)?;
        if none_props.is_none() {
            let (remain, _) = newline(str)?;
            many0(read_property)(remain)?
        } else {
            (str, Vec::new())
        }
    };
    let (remain, _) = take_while(|c: char| c == '\n')(str)?;

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
    let mut connections = Vec::new();
    for tag in tags {
        match tag._type {
            TagType::Node => {
                nodes.push(tag)
            }
            TagType::Connection => {
                connections.push(tag)
            }
            _ => {}
        }
    }

    Ok((
        remain,
        TSCNFile {
            header,
            ext_resources,
            sub_resources,
            nodes,
            connections,
        },
    ))
}

pub fn parse_tres_file(str: &str) -> IResult<&str, TSCNFile> {
    let (remain, header) = read_tag(str, Some(TagType::GdResource))?;

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
        nodes.push(tag)
    }

    Ok((
        remain,
        TSCNFile {
            header,
            ext_resources,
            sub_resources,
            nodes,
            connections: Vec::new(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use godot_data::nanoserde::SerJson;
    use crate::tscn_tres_parser::{parse_tres_file, parse_tscn_file};

    #[test]
    fn test_parse_tscn() {
        let input = r#"[gd_scene format=3 uid="uid://lrpk7b420cd7"]

[node name="Game" type="Node2D"]

[node name="scores" type="Label" parent="."]
layout_mode = 1
anchors_preset = 5
anchor_left = 0.5
anchor_right = 0.5
offset_left = -118.0
offset_top = 75.0
offset_right = 120.0
offset_bottom = 168.0
grow_horizontal = 2
text = "ПРИВ"
label_settings = SubResource("LabelSettings_4h1rj")
autowrap_mode = 2
justification_flags = 2
clip_text = true
        "#;
        let (input, tscn) = parse_tscn_file(input).unwrap();
        println!("{}", tscn.serialize_json());
    }

    #[test]
    fn test_parse_tres() {
        let input = r#"[gd_resource type="AtlasTexture" load_steps=2 format=3 uid="uid://bcjbib14mot8s"]

[ext_resource type="Texture2D" uid="uid://bqov4kuchixhi" path="res://atlases/icons.png" id="1_nhero"]

[resource]
atlas = ExtResource("1_nhero")
region = Rect2(1, 1483, 245, 245)"#;
        let (input, tscn) = parse_tres_file(input).unwrap();
        println!("{}", tscn.serialize_json());
    }
}