use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct DotPath {
    pub branches: Vec<String>,
    /// If none then the root is the local branch.
    pub root: RootType,
    pub ending: Ending,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Ending {
    Data,
    Branch,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum RootType {
    Local,
    Root(String),
    /// This is used internally and should never actually be created RAW.
    TrueRoot,
}

impl DotPath {
    pub fn new(s: &str) -> Result<Self, DotPathCreationError> {
        let mut new_s = s;

        // Check if this is a branch.
        let ending = if let Some(stripped) = new_s.strip_suffix(".") {
            new_s = stripped;
            Ending::Branch
        } else {
            Ending::Data
        };

        // If it starts with `[[` we assume that we are defining a root.
        let root = if let Some(stripped) = new_s.strip_prefix("[[") {
            if !stripped.contains("]]") {
                return Err(DotPathCreationError::UnclosedRootBranchDefinitionError(
                    s.into(),
                ));
            }

            let end_root = stripped
                .find("]]")
                .expect("Unable to find pattern that has already been confirmed to exist.");

            // If the end is `]]` then make `new_s` empty
            if end_root != stripped.len() - 2 {
                new_s = &stripped[end_root + 3..];
            } else {
                if ending == Ending::Data {
                    return Err(DotPathCreationError::UnclosedRootBranchDefinitionError(
                        s.into(),
                    ));
                }
                new_s = "";
            }

            RootType::Root(stripped[..end_root].to_string())
        } else {
            RootType::Local
        };

        // Split branches.
        let mut branches: Vec<String> = Vec::new();
        if new_s != "" {
            for (i, branch) in new_s.split('.').enumerate() {
                if branch.is_empty() {
                    return Err(DotPathCreationError::EmptyBranchError(new_s.to_string(), i)); // Unclosed root branch definition.
                }
                branches.push(branch.to_string());
            }
        }

        #[allow(unreachable_code)]
        Ok(Self {
            branches,
            ending,
            root,
        })
    }
}

#[derive(Debug, Error)]
pub enum DotPathCreationError {
    #[error("Unable to root branch for: {0}")]
    UnclosedRootBranchDefinitionError(String),

    #[error("Empty branch.\nWhen parsing dot path \"{0}\" the {1}nth root was found empty")]
    EmptyBranchError(String, usize),

    #[error("There is no such thing as a root data value, like \"{0}\" seems to be trying to do.")]
    PointingAtRootDataError(String),
}
