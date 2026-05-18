use std::{collections::HashMap, io, path::Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::Branchlet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Branch {
    /// The stuff that is, ya know, in the branch.
    pub stuff: HashMap<String, Branchlet>,
    /// If this is active then it will be used to conider the root rather then the branch name.
    pub root_name: Option<String>,
}

impl Branch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        ron::to_string(self).unwrap().as_bytes().to_vec()
    }

    pub fn from_bytes(b: &[u8]) -> Result<Self, BranchToBytesError> {
        let string = String::from_utf8(b.to_vec())
            .map_err(|_| BranchToBytesError::ByteToStringConversionError)?;
        let output = ron::from_str(&string)
            .map_err(|_| BranchToBytesError::RonStringToBranchConversionError)?;
        Ok(output)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let save_bytes = self.to_bytes();
        std::fs::write(path, save_bytes)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, LoadBranchError> {
        let load_bytes = std::fs::read(path).map_err(|e| LoadBranchError::LoadFileError(e))?;
        let actually_thingy =
            Self::from_bytes(&load_bytes).map_err(|e| LoadBranchError::BranchToBytesError(e))?;
        Ok(actually_thingy)
    }
}

#[derive(Error, Debug)]
pub enum BranchToBytesError {
    #[error("Was unable to make the bytes a proper string")]
    ByteToStringConversionError,

    #[error("The RON string just didn't work the way that it should")]
    RonStringToBranchConversionError,
}

#[derive(Debug, Error)]
pub enum LoadBranchError {
    #[error("{0}")]
    LoadFileError(io::Error),

    #[error("{0}")]
    BranchToBytesError(BranchToBytesError),
}

impl Default for Branch {
    fn default() -> Self {
        Self {
            stuff: HashMap::new(),
            root_name: None,
        }
    }
}

impl std::fmt::Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn print_branch(branch: &Branch, level: usize) -> String {
            // Get keys from stuff and order them alphabetically
            let mut keys: Vec<_> = branch.stuff.keys().collect();
            keys.sort();
            let mut total = String::new();
            for key in keys {
                let value = branch.stuff.get(key).unwrap();
                let mut root_name_printed = String::new();
                let printed = match value {
                    crate::Branchlet::Value(value) => match value {
                        crate::data::Data::Int(i) => format!("{i}"),
                        crate::data::Data::Float(f) => format!("{f}"),
                        crate::data::Data::String(s) => format!("{s:?}"),
                    },
                    crate::Branchlet::Branch(b) => {
                        if let Some(root_name) = &b.root_name {
                            root_name_printed = format!("({}) ", root_name);
                        }
                        format!("󱞣\n{}", print_branch(b, level + 1))
                    }
                };
                total.push_str(&format!(
                    "{}{root_name_printed}{key}: {printed}\n",
                    " ".repeat(level * 4)
                ));
            }

            if let Some(new) = total.strip_suffix("\n") {
                total = new.to_string();
            }

            return total;
        }

        if let Some(root_name) = &self.root_name {
            writeln!(f, "({})", root_name)?;
        }

        writeln!(f, "{}", print_branch(&self, 0))?;

        Ok(())
    }
}
