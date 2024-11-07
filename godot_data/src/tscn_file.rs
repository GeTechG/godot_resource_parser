use std::collections::{HashMap};
use std::fmt::Error;
use std::str::FromStr;
use nanoserde::{DeBin, DeJson, SerBin, SerJson};
use crate::values::GodotValue;

#[derive(Eq, PartialEq)]
#[cfg_attr(feature = "serjson", derive(SerJson))]
#[cfg_attr(feature = "serjsonpretty", derive(SerJson))]
#[cfg_attr(feature = "dejson", derive(DeJson))]
#[cfg_attr(feature = "serbin", derive(SerBin))]
#[cfg_attr(feature = "debin", derive(DeBin))]
pub enum TagType {
    GdScene,
    GdResource,
    ExtResource,
    SubResource,
    Node,
    Resource,
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
#[cfg_attr(feature = "serbin", derive(SerBin))]
#[cfg_attr(feature = "debin", derive(DeBin))]
pub struct Tag {
    pub _type: TagType,
    pub attrs: HashMap<String, GodotValue>,
    pub props: HashMap<String, GodotValue>,
}

#[cfg_attr(feature = "serjson", derive(SerJson))]
#[cfg_attr(feature = "serjsonpretty", derive(SerJson))]
#[cfg_attr(feature = "dejson", derive(DeJson))]
#[cfg_attr(feature = "serbin", derive(SerBin))]
#[cfg_attr(feature = "debin", derive(DeBin))]
pub struct TSCNFile {
    pub header: Tag,
    pub ext_resources: HashMap<String, Tag>,
    pub sub_resources: HashMap<String, Tag>,
    pub nodes: Vec<Tag>,
    pub connections: Vec<Tag>,
}
