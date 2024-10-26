use nanoserde::{DeBin, DeJson, SerBin, SerJson};
use godot_macros::SerJsonEnumPretty;

#[cfg_attr(feature = "serjson", derive(SerJson))]
#[cfg_attr(feature = "serjsonpretty", derive(SerJsonEnumPretty))]
#[cfg_attr(feature = "dejson", derive(DeJson))]
#[cfg_attr(feature = "serbin", derive(SerBin))]
#[cfg_attr(feature = "debin", derive(DeBin))]
pub enum GodotValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    PackedStringArray(Vec<String>),
    NodePath(String),
    Vector2((f64, f64)),
    Rect2((f64, f64, f64, f64)),
    Color((f64, f64, f64, f64)),
    ExtResourceLink(String),
    SubResourceLink(String),
}