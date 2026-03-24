use regex::Regex;
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
        // Define the regex
        let re = Regex::new(r"^(?:\[\[(.*?)\]\]\.)?(.+)$").unwrap();

        //  Do the regex thing.
        let caps = re
            .captures(&s)
            .ok_or(DotPathCreationError::RegexMatchError(s.to_string()))?;

        // Get values.
        let fake_root = caps.get(1).map(|m| m.as_str().to_string());
        let mut new_s = caps.get(2).unwrap().as_str();

        let ending = if let Some(stripped) = new_s.strip_suffix(".") {
            new_s = stripped;
            Ending::Branch
        } else {
            Ending::Data
        };

        // Split branches.
        let mut branches: Vec<String> = Vec::new();
        for (i, branch) in new_s.split('.').enumerate() {
            if branch.is_empty() {
                return Err(DotPathCreationError::EmptyBranchError(s.to_string(), i));
            }
            branches.push(branch.to_string());
        }

        // Get the actual root type.
        let root = if let Some(root_name) = fake_root {
            RootType::Root(root_name)
        } else {
            RootType::Local
        };

        Ok(Self {
            branches,
            root,
            ending,
        })
    }
}

#[derive(Debug, Error)]
pub enum DotPathCreationError {
    #[error("Unable to match regex: {0}")]
    RegexMatchError(String),

    #[error("Empty branch.\nWhen parsing dot path \"{0}\" the {1}nth root was found empty")]
    EmptyBranchError(String, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_root_data() {
        let dp = DotPath::new("a.b.c").unwrap();
        assert_eq!(dp.root, RootType::Local);
        assert_eq!(dp.branches, vec!["a", "b", "c"]);
        assert_eq!(dp.ending, Ending::Data);
    }

    #[test]
    fn test_local_root_branch() {
        let dp = DotPath::new("a.b.c.").unwrap();
        assert_eq!(dp.root, RootType::Local);
        assert_eq!(dp.branches, vec!["a", "b", "c"]);
        assert_eq!(dp.ending, Ending::Branch);
    }

    #[test]
    fn test_named_root_data() {
        let dp = DotPath::new("[[root]].a.b").unwrap();
        assert_eq!(dp.root, RootType::Root("root".to_string()));
        assert_eq!(dp.branches, vec!["a", "b"]);
        assert_eq!(dp.ending, Ending::Data);
    }

    #[test]
    fn test_named_root_branch() {
        let dp = DotPath::new("[[root]].a.b.").unwrap();
        assert_eq!(dp.root, RootType::Root("root".to_string()));
        assert_eq!(dp.branches, vec!["a", "b"]);
        assert_eq!(dp.ending, Ending::Branch);
    }

    #[test]
    fn test_single_branch() {
        let dp = DotPath::new("a").unwrap();
        assert_eq!(dp.root, RootType::Local);
        assert_eq!(dp.branches, vec!["a"]);
        assert_eq!(dp.ending, Ending::Data);
    }

    #[test]
    fn test_empty_branch_middle() {
        let err = DotPath::new("a..b").unwrap_err();
        match err {
            DotPathCreationError::EmptyBranchError(s, idx) => {
                assert_eq!(s, "a..b".to_string());
                assert_eq!(idx, 1);
            }
            _ => panic!("Expected EmptyBranchError"),
        }
    }

    #[test]
    fn test_empty_branch_start() {
        let err = DotPath::new(".a").unwrap_err();
        match err {
            DotPathCreationError::EmptyBranchError(_, idx) => {
                assert_eq!(idx, 0);
            }
            _ => panic!("Expected EmptyBranchError"),
        }
    }

    #[test]
    fn test_empty_branch_end() {
        let err = DotPath::new("a.b..").unwrap_err();
        match err {
            DotPathCreationError::EmptyBranchError(_, idx) => {
                assert_eq!(idx, 2);
            }
            _ => panic!("Expected EmptyBranchError"),
        }
    }

    #[test]
    fn test_root_only_invalid() {
        let err = DotPath::new("[[root]].").unwrap_err();
        match err {
            DotPathCreationError::EmptyBranchError(_, idx) => {
                assert_eq!(idx, 0);
            }
            _ => panic!("Expected EmptyBranchError"),
        }
    }

    #[test]
    fn test_root_with_empty_branch() {
        let err = DotPath::new("[[r]].a..b").unwrap_err();
        match err {
            DotPathCreationError::EmptyBranchError(_, idx) => {
                assert_eq!(idx, 1);
            }
            _ => panic!("Expected EmptyBranchError"),
        }
    }

    #[test]
    fn test_malformed_root_current_behavior() {
        let dp = DotPath::new("[[root.a.b").unwrap();
        assert_eq!(dp.root, RootType::Local);
        assert_eq!(dp.branches, vec!["[[root", "a", "b"]);
    }
}
