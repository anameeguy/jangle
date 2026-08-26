use std::str::FromStr;

use anyhow::{Context, Result};
use jangle::{
    DotPath, DotPathCreationError, TrueRootSheet,
    dot_path::{self, GenericTarget, PositionedRoot, TargetTypeTrait, TrueRoot},
};

use crate::working_sheet::WorkingSheetCache;

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

impl<TargetType: TargetTypeTrait> GenericRootDotpath<TargetType> {
    pub fn get_a_true_root_based_on_working_branch<'a>(
        &'a self,
        working_sheet_cache: &WorkingSheetCache,
        sheet: &TrueRootSheet,
    ) -> Result<&'a DotPath<TrueRoot, TargetType>> {
        match &self {
            GenericRootDotpath::TrueRootDotpath(dot_path) => {
                // If it is already a true root then not much to do now is there.
                Ok(dot_path)
            }
            GenericRootDotpath::PositionedRootDotpath(dot_path) => {
                let working_branch = &working_sheet_cache.working_branch_path;

                let thingy = working_sheet_cache
                    .working_branch_path
                    .as_ref()
                    .context("")?;
                let thingy3 = sheet.truly_root_dotpath::<TargetType>(dot_path, &thingy);

                // .context("No working branch path in config")
                todo!()
            }
        }
    }
}
