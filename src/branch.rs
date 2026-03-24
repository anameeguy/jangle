use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Data;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Branch(pub HashMap<String, Data>);

impl Branch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pack(self) -> Vec<u8> {
        ron::to_string(&self).unwrap().as_bytes().to_vec()
    }

    pub fn unpack(b: &[u8]) -> Self {
        ron::from_str(&String::from_utf8(b.to_vec()).unwrap()).unwrap()
    }
}

impl Default for Branch {
    fn default() -> Self {
        Self(HashMap::new())
    }
}
