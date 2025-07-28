use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::function::merge::merge_two;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn init() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn from_partial(partial: PartialPosition) -> Self {
        let init_json = json!({
            "x": 0.0,
            "y": 0.0
        });
        
        let partial_json = serde_json::to_value(partial).unwrap_or(Value::Null);
        let merged = merge_two(init_json, partial_json);
        
        serde_json::from_value(merged).unwrap_or_else(|_| Self::init())
    }

    pub fn from_number(value: f64) -> Self {
        Self { x: value, y: value }
    }

    pub fn from(input: PositionInput) -> Self {
        match input {
            PositionInput::Partial(partial) => Self::from_partial(partial),
            PositionInput::Number(value) => Self::from_number(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialPosition {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum PositionInput {
    Partial(PartialPosition),
    Number(f64),
}

impl From<PartialPosition> for PositionInput {
    fn from(partial: PartialPosition) -> Self {
        PositionInput::Partial(partial)
    }
}

impl From<f64> for PositionInput {
    fn from(value: f64) -> Self {
        PositionInput::Number(value)
    }
}
