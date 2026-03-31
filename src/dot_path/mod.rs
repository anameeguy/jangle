use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

const TRUE_ROOT_SYMBOL: &str = "#";
const LOCAL_SYMBOL: &str = "$";
const DEFINED_ROOT_START: &str = "[[";
const DEFINED_ROOT_END: &str = "]]";
const IS_ROOT_SYMBOL: &str = ".";

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
        let ending = if let Some(stripped) = new_s.strip_suffix(IS_ROOT_SYMBOL) {
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
        let root = if first == TRUE_ROOT_SYMBOL.to_string() {
            RootType::TrueRoot
        } else if first == LOCAL_SYMBOL.to_string() {
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

        if (root == RootType::TrueRoot || root == RootType::Local)
            && branches.is_empty()
            && ending == Ending::Data
        {
            return Err(DotPathCreationError::PointingAtRootDataError(s.to_string()));
        }

        #[allow(unreachable_code)]
        Ok(Self {
            branches,
            ending,
            root,
        })
    }

    pub const TRUE_ROOT: Self = Self {
        branches: Vec::new(),
        ending: Ending::Branch,
        root: RootType::TrueRoot,
    };
}

impl Display for DotPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.root {
            RootType::Root(root) => {
                write!(f, "{DEFINED_ROOT_START}{root}{DEFINED_ROOT_END}")?;
            }
            RootType::TrueRoot => {
                write!(f, "{TRUE_ROOT_SYMBOL}")?;
            }
            RootType::Local => {
                write!(f, "{LOCAL_SYMBOL}")?;
            }
        }

        if !self.branches.is_empty() {
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
    #[error("Empty branch.\nWhen parsing dot path \"{0}\" the {1}nth root was found empty")]
    EmptyBranchError(String, usize),

    #[error("There is no such thing as a root data value, like \"{0}\" seems to be trying to do.")]
    PointingAtRootDataError(String),

    #[error(r#""{0}" does not match any of the root patterns."#)]
    UndefinedRootError(String),

    #[error("Tried to construct a completly empty dotpath.")]
    IsEmpty,
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- ✅ VALID CASES ---

    #[test]
    fn parse_true_root_branch() {
        let dp = DotPath::new("#.").unwrap();
        assert_eq!(dp.root, RootType::TrueRoot);
        assert_eq!(dp.branches.len(), 0);
        assert_eq!(dp.ending, Ending::Branch);
    }

    #[test]
    fn parse_local_with_branch() {
        let dp = DotPath::new("$.a").unwrap();
        assert_eq!(dp.root, RootType::Local);
        assert_eq!(dp.branches, vec!["a"]);
        assert_eq!(dp.ending, Ending::Data);
    }

    #[test]
    fn parse_defined_root() {
        let dp = DotPath::new("[[abc]].x.y").unwrap();
        assert_eq!(dp.root, RootType::Root("abc".to_string()));
        assert_eq!(dp.branches, vec!["x", "y"]);
        assert_eq!(dp.ending, Ending::Data);
    }

    #[test]
    fn parse_defined_root_branch() {
        let dp = DotPath::new("[[root]].a.").unwrap();
        assert_eq!(dp.root, RootType::Root("root".to_string()));
        assert_eq!(dp.branches, vec!["a"]);
        assert_eq!(dp.ending, Ending::Branch);
    }

    #[test]
    fn parse_multiple_branches() {
        let dp = DotPath::new("$.a.b.c").unwrap();
        assert_eq!(dp.branches, vec!["a", "b", "c"]);
    }

    // --- ❌ ERROR CASES ---

    #[test]
    fn error_empty_input() {
        let err = DotPath::new("").unwrap_err();
        matches!(err, DotPathCreationError::IsEmpty);
    }

    #[test]
    fn error_empty_branch_middle() {
        let err = DotPath::new("$.a..b").unwrap_err();
        matches!(err, DotPathCreationError::EmptyBranchError(_, 2));
    }

    #[test]
    fn error_undefined_root() {
        let err = DotPath::new("abc.def").unwrap_err();
        matches!(err, DotPathCreationError::UndefinedRootError(_));
    }

    #[test]
    fn error_root_as_data_true_root() {
        let err = DotPath::new("#").unwrap_err();
        matches!(err, DotPathCreationError::PointingAtRootDataError(_));
    }

    #[test]
    fn error_root_as_data_local() {
        let err = DotPath::new("$").unwrap_err();
        matches!(err, DotPathCreationError::PointingAtRootDataError(_));
    }

    // --- 🔁 ROUND TRIP TESTS ---

    fn round_trip_case(input: &str) {
        let parsed = DotPath::new(input).unwrap();
        let printed = parsed.to_string();
        assert_eq!(printed, input);

        let reparsed = DotPath::new(&printed).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn round_trip_true_root() {
        round_trip_case("#.");
    }

    #[test]
    fn round_trip_local() {
        round_trip_case("$.a");
    }

    #[test]
    fn round_trip_defined_root() {
        round_trip_case("[[abc]].x.y");
    }

    #[test]
    fn round_trip_branch() {
        round_trip_case("[[abc]].x.");
    }

    // --- 🧠 EDGE CASES ---

    #[test]
    fn single_branch_defined_root() {
        let dp = DotPath::new("[[r]].a").unwrap();
        assert_eq!(dp.branches, vec!["a"]);
    }

    #[test]
    fn branch_index_correct_on_error() {
        let err = DotPath::new("$.a..").unwrap_err();
        if let DotPathCreationError::EmptyBranchError(_, idx) = err {
            assert_eq!(idx, 2);
        } else {
            panic!("Expected EmptyBranchError");
        }
    }
}
