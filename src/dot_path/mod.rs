use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const TRUE_ROOT_SYMBOL: &str = "#";
pub const LOCAL_SYMBOL: &str = "$";
pub const DEFINED_ROOT_START: &str = "[[";
pub const DEFINED_ROOT_END: &str = "]]";
pub const IS_BRANCH_SYMBOL: &str = ".";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct DotPath<RootType: RootTypeTrait, TargetType: TargetTypeTrait> {
    pub path: Vec<String>,
    pub root: RootType,
    pub target: TargetType,
}

impl<RootType: RootTypeTrait, TargetType: TargetTypeTrait> DotPath<RootType, TargetType> {
    #[allow(unused_variables)]
    pub fn new(s: &str) -> Result<Self, DotPathCreationError> {
        // Check if this is data or a branch.
        let (better_s, target_type) = if let Some(beep) = s.strip_suffix(".") {
            (beep, TargetTypeEnum::Branch)
        } else {
            (s, TargetTypeEnum::Data)
        };
        if target_type != TargetType::ENUM {
            return Err(DotPathCreationError::WrongTargetIGuess);
        }

        let mut split_i_guess: Vec<&str> = better_s.split('.').collect();

        // If we got nothin then we have problems.
        if split_i_guess.is_empty() {
            return Err(DotPathCreationError::IsEmpty);
        }

        // If any of them are empty then we also have problems.
        if split_i_guess.iter().any(|v| v.is_empty()) {
            return Err(DotPathCreationError::EmptyBranchError(s.to_string()));
        }

        // Check the first thingy for the root.
        let first = split_i_guess.remove(0);
        let root: RootType = RootType::_get_root(first)?;

        // Figure out the ending part.
        let target = TargetType::_construct(&mut split_i_guess)?;

        Ok(DotPath {
            path: split_i_guess.iter().map(|v| v.to_string()).collect(),
            root,
            target,
        })
    }
}

// TODO: Make the errors not suck.
#[derive(Debug, Error)]
pub enum DotPathCreationError {
    #[error("Empty branch when parsing dot path \"{0}\".")]
    EmptyBranchError(String),

    #[error("There is no such thing as a root data value.")]
    PointingAtRootDataError,

    #[error(r#""{0}" does not match any of the root patterns avalible for this structure."#)]
    UndefinedRootError(String),

    #[error("Tried to construct a completly empty dotpath.")]
    IsEmpty,

    #[error(
        "You got the wrong target I guess.\n
        I am tired of writing these errors."
    )]
    WrongTargetIGuess,

    #[error("You did something or other wrong with the root.")]
    WrongRootIGuess,
}

impl<RootType: RootTypeTrait, TargetType: TargetTypeTrait> Display
    for DotPath<RootType, TargetType>
{
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub trait RootTypeTrait: Debug + Serialize + Clone + PartialEq + Hash {
    const ENUM: RootTypeEnum;

    fn positioned_root(&self) -> Option<&PositionedRootTypeStruct> {
        None
    }
    fn positioned_root_mut(&mut self) -> Option<&mut PositionedRootTypeStruct> {
        None
    }

    fn _get_root(root_string: &str) -> Result<Self, DotPathCreationError>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct TrueRootTypeStruct;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct PositionedRootTypeStruct {
    pub origin: PositionedRootOrigin,
}

impl RootTypeTrait for TrueRootTypeStruct {
    const ENUM: RootTypeEnum = RootTypeEnum::TrueRoot;

    fn _get_root(root_string: &str) -> Result<Self, DotPathCreationError> {
        if root_string == TRUE_ROOT_SYMBOL {
            Ok(Self)
        } else {
            Err(DotPathCreationError::WrongTargetIGuess)
        }
    }
}

impl RootTypeTrait for PositionedRootTypeStruct {
    const ENUM: RootTypeEnum = RootTypeEnum::PositionedRoot;

    fn positioned_root(&self) -> Option<&PositionedRootTypeStruct> {
        Some(self)
    }

    fn positioned_root_mut(&mut self) -> Option<&mut PositionedRootTypeStruct> {
        Some(self)
    }

    fn _get_root(root_string: &str) -> Result<Self, DotPathCreationError> {
        if root_string == LOCAL_SYMBOL {
            Ok(PositionedRootTypeStruct {
                origin: PositionedRootOrigin::Local,
            })
        } else {
            let dis = root_string
                .strip_prefix(DEFINED_ROOT_START)
                .ok_or(DotPathCreationError::WrongRootIGuess)?
                .strip_suffix(DEFINED_ROOT_END)
                .ok_or(DotPathCreationError::WrongRootIGuess)?
                .to_string();
            Ok(PositionedRootTypeStruct {
                origin: PositionedRootOrigin::Defined(dis),
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub enum RootTypeEnum {
    TrueRoot,
    PositionedRoot,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub enum PositionedRootOrigin {
    Local,
    Defined(String),
}

pub trait TargetTypeTrait: Debug + Serialize + Clone + PartialEq + Hash {
    const ENUM: TargetTypeEnum;

    fn data(&self) -> Option<&DataTargetTypeStruct> {
        None
    }
    fn data_mut(&mut self) -> Option<&mut DataTargetTypeStruct> {
        None
    }

    fn _construct(stuff: &mut Vec<&str>) -> Result<Self, DotPathCreationError>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub enum TargetTypeEnum {
    Data = 0x1111111111111111,
    Branch = 0x0000000000000000,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct DataTargetTypeStruct {
    pub data_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct BranchTargetTypeStruct;

impl TargetTypeTrait for DataTargetTypeStruct {
    const ENUM: TargetTypeEnum = TargetTypeEnum::Data;

    fn data(&self) -> Option<&DataTargetTypeStruct> {
        Some(self)
    }

    fn data_mut(&mut self) -> Option<&mut DataTargetTypeStruct> {
        Some(self)
    }

    fn _construct(stuff: &mut Vec<&str>) -> Result<Self, DotPathCreationError> {
        Ok(DataTargetTypeStruct {
            data_name: stuff
                .pop()
                .ok_or(DotPathCreationError::PointingAtRootDataError)?
                .to_string(),
        })
    }
}

impl TargetTypeTrait for BranchTargetTypeStruct {
    const ENUM: TargetTypeEnum = TargetTypeEnum::Branch;

    #[allow(unused_variables)]
    fn _construct(stuff: &mut Vec<&str>) -> Result<Self, DotPathCreationError> {
        Ok(BranchTargetTypeStruct)
    }
}
