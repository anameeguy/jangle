use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const TRUE_ROOT_SYMBOL: &str = "#";
pub const LOCAL_SYMBOL: &str = "$";
pub const DEFINED_ROOT_START: &str = "[[";
pub const DEFINED_ROOT_END: &str = "]]";
pub const IS_BRANCH_SYMBOL: &str = ".";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct DotPath {
    path: Vec<String>,
}

impl DotPath {
    pub fn new(s: &str) -> Result<Self, DotPathCreationError> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Copy)]
pub enum DestinationType {
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

    #[error(
        "You got the wrong ending I guess.\n
        I am tired of writing these errors."
    )]
    WrongEndingIGuess,
}

impl Display for DotPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
