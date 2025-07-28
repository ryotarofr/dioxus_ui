use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::function::merge::merge_two;
use super::position::Position;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialSize {
    pub width: Option<f64>,
    pub height: Option<f64>,
}

impl Size {
    pub fn init() -> Self {
        Self { width: 1.0, height: 1.0 }
    }

    pub fn from_partial(partial: PartialSize) -> Self {
        let init_json = json!({
            "width": 1.0,
            "height": 1.0
        });
        
        let partial_json = serde_json::to_value(partial).unwrap_or(Value::Null);
        let merged = merge_two(init_json, partial_json);
        
        serde_json::from_value(merged).unwrap_or_else(|_| Self::init())
    }

    pub fn from_number(value: f64) -> Self {
        Self { width: value, height: value }
    }

    pub fn from(input: SizeInput) -> Self {
        match input {
            SizeInput::Partial(partial) => Self::from_partial(partial),
            SizeInput::Number(value) => Self::from_number(value),
        }
    }

    pub fn to_position(self) -> Position {
        Position { x: self.width, y: self.height }
    }

    pub fn from_position(position: Position) -> Self {
        Self { width: position.x, height: position.y }
    }
}

#[derive(Debug, Clone)]
pub enum SizeInput {
    Partial(PartialSize),
    Number(f64),
}

impl From<PartialSize> for SizeInput {
    fn from(partial: PartialSize) -> Self {
        SizeInput::Partial(partial)
    }
}

impl From<f64> for SizeInput {
    fn from(value: f64) -> Self {
        SizeInput::Number(value)
    }
}
