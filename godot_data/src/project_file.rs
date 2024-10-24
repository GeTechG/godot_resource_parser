use std::collections::HashMap;
use nanoserde::{DeJson, SerJson, DeBin, SerBin};
use crate::values::GodotValue;

pub type GodotFileParameters = HashMap<String, GodotValue>;


#[cfg_attr(feature = "serjson", derive(SerJson))]
#[cfg_attr(feature = "serjsonpretty", derive(SerJson))]
#[cfg_attr(feature = "dejson", derive(DeJson))]
#[cfg_attr(feature = "serbin", derive(SerBin))]
#[cfg_attr(feature = "debin", derive(DeBin))]
pub struct ProjectFile {
    #[cfg_attr(feature = "minname", nserde(rename = "cv"))]
    pub config_version: u32,
    #[cfg_attr(feature = "minname", nserde(rename = "s"))]
    pub sections: HashMap<String, GodotFileParameters>
}