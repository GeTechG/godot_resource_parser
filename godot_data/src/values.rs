use nanoserde::{DeBin, DeJson, SerBin, SerJson};
use godot_macros::SerJsonEnumPretty;

#[cfg_attr(feature = "serjson", derive(SerJson))]
#[cfg_attr(feature = "serjsonpretty", derive(SerJsonEnumPretty))]
#[cfg_attr(feature = "dejson", derive(DeJson))]
#[cfg_attr(feature = "serbin", derive(SerBin))]
#[cfg_attr(feature = "debin", derive(DeBin))]
pub enum GodotValue {
    #[cfg_attr(feature = "minname", nserde(rename = "s"))]
    String(String),
    #[cfg_attr(feature = "minname", nserde(rename = "i"))]
    Integer(i64),
    #[cfg_attr(feature = "minname", nserde(rename = "f"))]
    Float(f64),
    #[cfg_attr(feature = "minname", nserde(rename = "b"))]
    Boolean(bool),
    #[cfg_attr(feature = "minname", nserde(rename = "psa"))]
    PackedStringArray(Vec<String>),
}