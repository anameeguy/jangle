pub mod positioned_dot_path;
#[cfg(test)]
mod tests;
pub mod true_dot_path;

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dot_path::{positioned_dot_path::PositionedDotPath, true_dot_path::TrueDotPath};

pub const TRUE_ROOT_SYMBOL: &str = "#";
pub const LOCAL_SYMBOL: &str = "$";
pub const DEFINED_ROOT_START: &str = "[[";
pub const DEFINED_ROOT_END: &str = "]]";
pub const IS_BRANCH_SYMBOL: &str = ".";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub enum DotPath {
    Positioned(PositionedDotPath),
    True(TrueDotPath),
}

impl DotPath {
    pub fn new(s: &str) -> Result<Self, DotPathCreationError> {
        if s.starts_with(TRUE_ROOT_SYMBOL) {
            TrueDotPath::new(s).map(|tdp| DotPath::True(tdp))
        } else {
            PositionedDotPath::new(s).map(|pdp| DotPath::Positioned(pdp))
        }
    }

    pub fn parts(&self) -> &Vec<String> {
        match self {
            DotPath::Positioned(positioned_dot_path) => &positioned_dot_path.parts,
            DotPath::True(true_dot_path) => &true_dot_path.parts,
        }
    }

    pub fn parts_mut(&mut self) -> &mut Vec<String> {
        match self {
            DotPath::Positioned(positioned_dot_path) => &mut positioned_dot_path.parts,
            DotPath::True(true_dot_path) => &mut true_dot_path.parts,
        }
    }

    pub fn endings(&self) -> Ending {
        match self {
            DotPath::Positioned(positioned_dot_path) => positioned_dot_path.ending,
            DotPath::True(true_dot_path) => true_dot_path.ending,
        }
    }

    pub fn endings_mut(&mut self) -> &mut Ending {
        match self {
            DotPath::Positioned(positioned_dot_path) => &mut positioned_dot_path.ending,
            DotPath::True(true_dot_path) => &mut true_dot_path.ending,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Ending {
    Data,
    Branch,
}

#[derive(Debug, Error)]
pub enum DotPathCreationError {
    #[error("Empty branch.\nWhen parsing dot path \"{0}\" the {1}nth root was found empty")]
    EmptyBranchError(String, usize),

    #[error("There is no such thing as a root data value, like \"{0}\" seems to be trying to do.")]
    PointingAtRootDataError(String),

    #[error(r#""{0}" does not match any of the root patterns avalible for this structure."#)]
    UndefinedRootError(String),

    #[error("Tried to construct a completly empty dotpath.")]
    IsEmpty,
}

impl Display for DotPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DotPath::Positioned(positioned_dot_path) => format!("{positioned_dot_path}"),
                DotPath::True(true_dot_path) => format!("{true_dot_path}"),
            }
        )
    }
}
