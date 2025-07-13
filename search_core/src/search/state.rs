use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Int(i32),
    Int64(i64),
    OrderedFloat64(ordered_float::OrderedFloat<f64>),
    Text(String),
    Bool(bool),
    Position(Position),
    Positions(BTreeMap<String, Position>),
    IntArray(Vec<i32>),
    Int64Array(Vec<i64>),
    BoolArray(Vec<bool>),
    StringArray(Vec<String>),
    MapToVecString(BTreeMap<String, Vec<String>>),
    MapToString(BTreeMap<String, String>),
    MapToInt(BTreeMap<String, i32>),
    MapToBool(BTreeMap<String, bool>),
    MapToValue(BTreeMap<String, Value>),
    MapToMapToInt(BTreeMap<String, BTreeMap<String, i32>>),
    MapToMapToString(BTreeMap<String, BTreeMap<String, String>>),
}

pub trait StateTrait: Debug + Clone + Serialize + Hash + for<'de> Deserialize<'de> + Eq {}
