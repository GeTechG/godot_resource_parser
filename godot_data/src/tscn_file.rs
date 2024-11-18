use crate::values::GodotValue;
use std::collections::HashMap;
use std::fmt::Error;
use std::str::FromStr;

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};
#[cfg(any(feature = "serjson", feature = "serjsonpretty", feature = "dejson"))]
use nanoserde::{SerJson, DeJson};
#[cfg(any(feature = "serron", feature = "deron"))]
use nanoserde::{SerRon, DeRon};

#[derive(Eq, PartialEq)]
#[cfg_attr(feature = "serjson", derive(SerJson))]
#[cfg_attr(feature = "serjsonpretty", derive(SerJson))]
#[cfg_attr(feature = "dejson", derive(DeJson))]
#[cfg_attr(feature = "serbin", derive(Encode))]
#[cfg_attr(feature = "debin", derive(Decode))]
#[cfg_attr(feature = "serron", derive(SerRon))]
#[cfg_attr(feature = "deron", derive(DeRon))]
pub enum TagType {
    #[cfg_attr(feature = "minname", nserde(rename = "GS"))]
    GdScene,
    #[cfg_attr(feature = "minname", nserde(rename = "GR"))]
    GdResource,
    #[cfg_attr(feature = "minname", nserde(rename = "ER"))]
    ExtResource,
    #[cfg_attr(feature = "minname", nserde(rename = "SR"))]
    SubResource,
    #[cfg_attr(feature = "minname", nserde(rename = "N"))]
    Node,
    #[cfg_attr(feature = "minname", nserde(rename = "R"))]
    Resource,
    #[cfg_attr(feature = "minname", nserde(rename = "C"))]
    Connection,
}

impl FromStr for TagType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gd_scene" => Ok(TagType::GdScene),
            "gd_resource" => Ok(TagType::GdResource),
            "ext_resource" => Ok(TagType::ExtResource),
            "sub_resource" => Ok(TagType::SubResource),
            "node" => Ok(TagType::Node),
            "resource" => Ok(TagType::Resource),
            "connection" => Ok(TagType::Connection),
            _ => Err(Error),
        }
    }
}


#[cfg_attr(feature = "serjson", derive(SerJson))]
#[cfg_attr(feature = "serjsonpretty", derive(SerJson))]
#[cfg_attr(feature = "dejson", derive(DeJson))]
#[cfg_attr(feature = "serbin", derive(Encode))]
#[cfg_attr(feature = "debin", derive(Decode))]
#[cfg_attr(feature = "serron", derive(SerRon))]
#[cfg_attr(feature = "deron", derive(DeRon))]
pub struct Tag {
    #[cfg_attr(feature = "minname", nserde(rename = "t"))]
    pub _type: TagType,
    #[cfg_attr(feature = "minname", nserde(rename = "a"))]
    pub attrs: HashMap<String, GodotValue>,
    #[cfg_attr(feature = "minname", nserde(rename = "p"))]
    pub props: HashMap<String, GodotValue>,
}

#[cfg_attr(feature = "serjson", derive(SerJson))]
#[cfg_attr(feature = "serjsonpretty", derive(SerJson))]
#[cfg_attr(feature = "dejson", derive(DeJson))]
#[cfg_attr(feature = "serbin", derive(Encode))]
#[cfg_attr(feature = "debin", derive(Decode))]
#[cfg_attr(feature = "serron", derive(SerRon))]
#[cfg_attr(feature = "deron", derive(DeRon))]
pub struct TSCNFile {
    #[cfg_attr(feature = "minname", nserde(rename = "h"))]
    pub header: Tag,
    #[cfg_attr(feature = "minname", nserde(rename = "er"))]
    pub ext_resources: HashMap<String, Tag>,
    #[cfg_attr(feature = "minname", nserde(rename = "sr"))]
    pub sub_resources: HashMap<String, Tag>,
    #[cfg_attr(feature = "minname", nserde(rename = "n"))]
    pub nodes: Vec<Tag>,
    #[cfg_attr(feature = "minname", nserde(rename = "c"))]
    pub connections: Vec<Tag>,
}
