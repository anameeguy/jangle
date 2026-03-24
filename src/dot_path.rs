use std::fmt::Display;

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
                // Make sure that period is directly after `]]`
                if stripped.chars().nth(end_root + 2).unwrap() != '.' {
                    return Err(DotPathCreationError::MalformedRootError(s.into()));
                }
                new_s = &stripped[end_root + 3..];
            } else {
                if ending == Ending::Data {
                    return Err(DotPathCreationError::UnclosedRootBranchDefinitionError(
                        s.into(),
                    ));
                }
                new_s = "";
            }

            let root = &stripped[..end_root];
            if root != "TRUEROOT" {
                RootType::Root(root.to_string())
            } else {
                RootType::TrueRoot
            }
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

impl Display for DotPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.root {
            RootType::Root(root) => {
                write!(f, "[[{root}]]")?;
            }
            RootType::TrueRoot => {
                write!(f, "[[TRUEROOT]]")?;
            }
            _ => {}
        }

        if self.root != RootType::Local && !self.branches.is_empty() {
            write!(f, ".")?;
        }

        let da_rest = self.branches.join(".");
        write!(f, "{da_rest}")?;

        if self.ending == Ending::Branch {
            write!(f, ".")?;
        }

        Ok(())
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

    #[error("Malformed root: {0}")]
    MalformedRootError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(s: &str) {
        let parsed = DotPath::new(s).unwrap();
        let printed = parsed.to_string();
        assert_eq!(printed, s, "Roundtrip failed: {s} -> {printed}");
    }

    #[test]
    fn basic_paths() {
        roundtrip("a");
        roundtrip("a.b");
        roundtrip("a.b.c");
    }

    #[test]
    fn branch_paths() {
        roundtrip("a.");
        roundtrip("a.b.");
    }

    #[test]
    fn root_paths() {
        roundtrip("[[root]].a");
        roundtrip("[[root]].a.b");
        roundtrip("[[TRUEROOT]].a");
    }

    #[test]
    fn root_branch_paths() {
        roundtrip("[[root]].");
        roundtrip("[[TRUEROOT]].");
    }

    #[test]
    fn empty_branches_fail() {
        assert!(DotPath::new("a..b").is_err());
        assert!(DotPath::new(".a").is_err());
        assert!(DotPath::new("a.").is_ok()); // valid branch ending
    }

    #[test]
    fn malformed_root_fails() {
        assert!(DotPath::new("[[root]").is_err());
        assert!(DotPath::new("[[root]a").is_err());
        assert!(DotPath::new("[[root]]a").is_err());
    }

    #[test]
    fn root_without_branch_is_invalid() {
        // According to your rule: root must be a branch
        assert!(DotPath::new("[[root]]").is_err());
    }

    #[test]
    fn display_never_produces_invalid_strings() {
        let paths = vec![
            DotPath {
                branches: vec!["a".into()],
                root: RootType::Local,
                ending: Ending::Data,
            },
            DotPath {
                branches: vec!["a".into()],
                root: RootType::Local,
                ending: Ending::Branch,
            },
            DotPath {
                branches: vec!["a".into()],
                root: RootType::Root("r".into()),
                ending: Ending::Data,
            },
            DotPath {
                branches: vec!["a".into()],
                root: RootType::Root("r".into()),
                ending: Ending::Branch,
            },
        ];

        for p in paths {
            let s = p.to_string();
            assert!(
                DotPath::new(&s).is_ok(),
                "Display produced invalid string: {s}"
            );
        }
    }
}
