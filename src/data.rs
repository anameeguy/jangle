use serde::{Deserialize, Serialize};

use crate::branch::Branch;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Branchlet {
    Value(Data),
    Branch(Branch),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Data {
    Int(i64),
    Float(f64),
    String(String),
}
