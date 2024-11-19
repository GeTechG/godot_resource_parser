use godot_macros::SerJsonEnumPretty;

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};
#[cfg(any(feature = "serjson", feature = "serjsonpretty", feature = "dejson"))]
use nanoserde::{SerJson, DeJson};
#[cfg(any(feature = "serron", feature = "deron"))]
use nanoserde::{SerRon, DeRon};

#[cfg_attr(feature = "serjson", derive(SerJson))]
#[cfg_attr(feature = "serjsonpretty", derive(SerJsonEnumPretty))]
#[cfg_attr(feature = "dejson", derive(DeJson))]
#[cfg_attr(feature = "serbin", derive(Encode))]
#[cfg_attr(feature = "debin", derive(Decode))]
#[cfg_attr(feature = "serron", derive(SerRon))]
#[cfg_attr(feature = "deron", derive(DeRon))]
pub enum GodotValue {
    #[cfg_attr(feature = "minname", nserde(rename = "S"))]
    String(String),
    #[cfg_attr(feature = "minname", nserde(rename = "I"))]
    Integer(i64),
    #[cfg_attr(feature = "minname", nserde(rename = "F"))]
    Float(f64),
    #[cfg_attr(feature = "minname", nserde(rename = "B"))]
    Boolean(bool),
    #[cfg_attr(feature = "minname", nserde(rename = "PSA"))]
    PackedStringArray(Vec<String>),
    #[cfg_attr(feature = "minname", nserde(rename = "NP"))]
    NodePath(String),
    #[cfg_attr(feature = "minname", nserde(rename = "V2"))]
    Vector2((f64, f64)),
    #[cfg_attr(feature = "minname", nserde(rename = "R2"))]
    Rect2((f64, f64, f64, f64)),
    #[cfg_attr(feature = "minname", nserde(rename = "C"))]
    Color((f64, f64, f64, f64)),
    #[cfg_attr(feature = "minname", nserde(rename = "ERL"))]
    ExtResourceLink(String),
    #[cfg_attr(feature = "minname", nserde(rename = "SRL"))]
    SubResourceLink(String),
    #[cfg_attr(feature = "minname", nserde(rename = "A"))]
    Array(Vec<GodotValue>),
}