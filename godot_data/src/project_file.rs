use crate::values::GodotValue;
use std::collections::HashMap;

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};
#[cfg(any(feature = "serjson", feature = "serjsonpretty", feature = "dejson"))]
use nanoserde::{SerJson, DeJson};
#[cfg(any(feature = "serron", feature = "deron"))]
use nanoserde::{SerRon, DeRon};

pub type GodotFileParameters = HashMap<String, GodotValue>;


#[cfg_attr(feature = "serjson", derive(SerJson))]
#[cfg_attr(feature = "serjsonpretty", derive(SerJson))]
#[cfg_attr(feature = "dejson", derive(DeJson))]
#[cfg_attr(feature = "serbin", derive(Encode))]
#[cfg_attr(feature = "debin", derive(Decode))]
#[cfg_attr(feature = "serron", derive(SerRon))]
#[cfg_attr(feature = "deron", derive(DeRon))]
pub struct ProjectFile {
    #[cfg_attr(feature = "minname", nserde(rename = "cv"))]
    pub config_version: u32,
    #[cfg_attr(feature = "minname", nserde(rename = "s"))]
    pub sections: HashMap<String, GodotFileParameters>
}