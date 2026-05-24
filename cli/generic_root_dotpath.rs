use std::str::FromStr;

use jangle::{
    DotPath, DotPathCreationError,
    dot_path::{self, GenericTarget, PositionedRoot, TargetTypeTrait, TrueRoot},
};

// TODO: Make this not suck.

/// The dot path structure does not have an option for a generic root.
/// For the perpouses of user interfacing I have created this hacky solution.
#[derive(Debug, Clone)]
pub enum GenericRootDotpath<TargetType: TargetTypeTrait = GenericTarget> {
    TrueRootDotpath(DotPath<TrueRoot, TargetType>),
    PositionedRootDotpath(DotPath<PositionedRoot, TargetType>),
}

impl<TargetType: TargetTypeTrait> FromStr for GenericRootDotpath<TargetType> {
    type Err = DotPathCreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(dot_path::TRUE_ROOT_SYMBOL) {
            // We have a true root.
            let result = DotPath::<TrueRoot, TargetType>::new(s)?;
            Ok(Self::TrueRootDotpath(result))
        } else {
            // We (prolly) have a positioned root.
            let result = DotPath::<PositionedRoot, TargetType>::new(s)?;
            Ok(Self::PositionedRootDotpath(result))
        }
    }
}
