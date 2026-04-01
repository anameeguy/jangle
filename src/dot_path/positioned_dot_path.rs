use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    DotPath,
    dot_path::{
        DEFINED_ROOT_END, DEFINED_ROOT_START, DotPathCreationError, Ending, IS_BRANCH_SYMBOL,
        LOCAL_SYMBOL,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct PositionedDotPath {
    pub parts: Vec<String>,
    pub root: RootType,
    pub ending: Ending,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum RootType {
    Local,
    Root(String),
}

impl PositionedDotPath {
    pub fn new(s: &str) -> Result<Self, DotPathCreationError> {
        let mut new_s = s;

        // Check if this is a branch.
        let ending = if let Some(stripped) = new_s.strip_suffix(IS_BRANCH_SYMBOL) {
            new_s = stripped;
            Ending::Branch
        } else {
            Ending::Data
        };

        if new_s.is_empty() {
            return Err(DotPathCreationError::IsEmpty);
        }

        // Split branches.
        let mut branches: Vec<String> = Vec::new();
        for (i, branch) in new_s.split('.').enumerate() {
            if branch.is_empty() {
                return Err(DotPathCreationError::EmptyBranchError(new_s.to_string(), i)); // Unclosed root branch definition.
            }
            branches.push(branch.to_string());
        }

        let first = branches.remove(0);
        let root = if first == LOCAL_SYMBOL.to_string() {
            RootType::Local
        } else if first.starts_with(DEFINED_ROOT_START) && first.ends_with(DEFINED_ROOT_END) {
            RootType::Root(
                first
                    .strip_prefix(DEFINED_ROOT_START)
                    .unwrap()
                    .strip_suffix(DEFINED_ROOT_END)
                    .unwrap()
                    .to_string(),
            )
        } else {
            return Err(DotPathCreationError::UndefinedRootError(first));
        };

        if branches.is_empty() && ending == Ending::Data {
            return Err(DotPathCreationError::PointingAtRootDataError(s.to_string()));
        }

        #[allow(unreachable_code)]
        Ok(Self {
            parts: branches,
            ending,
            root,
        })
    }
}

impl Display for PositionedDotPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.root {
            RootType::Root(root) => {
                write!(f, "{DEFINED_ROOT_START}{root}{DEFINED_ROOT_END}")?;
            }
            RootType::Local => {
                write!(f, "{LOCAL_SYMBOL}")?;
            }
        }

        if !self.parts.is_empty() {
            write!(f, ".")?;
        }

        let da_rest = self.parts.join(".");
        write!(f, "{da_rest}")?;

        if self.ending == Ending::Branch {
            write!(f, ".")?;
        }

        Ok(())
    }
}

impl Into<DotPath> for PositionedDotPath {
    fn into(self) -> DotPath {
        DotPath::Positioned(self)
    }
}
