use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    DotPath,
    dot_path::{DotPathCreationError, Ending, IS_BRANCH_SYMBOL, TRUE_ROOT_SYMBOL},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct TrueDotPath {
    pub parts: Vec<String>,
    pub ending: Ending,
}

impl TrueDotPath {
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
        if first != TRUE_ROOT_SYMBOL.to_string() {
            return Err(DotPathCreationError::UndefinedRootError(first));
        }

        if branches.is_empty() && ending == Ending::Data {
            return Err(DotPathCreationError::PointingAtRootDataError(s.to_string()));
        }

        #[allow(unreachable_code)]
        Ok(Self {
            parts: branches,
            ending,
        })
    }

    pub const TRUE_ROOT: Self = Self {
        parts: Vec::new(),
        ending: Ending::Branch,
    };
}

impl Display for TrueDotPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{TRUE_ROOT_SYMBOL}")?;

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

impl Into<DotPath> for TrueDotPath {
    fn into(self) -> DotPath {
        DotPath::True(self)
    }
}
