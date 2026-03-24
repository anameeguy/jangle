use serde::{Deserialize, Serialize};

use crate::branch::Branch;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Data {
    Value(Value),
    Branch(Branch),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
}
