mod truly_root_dotpath;

use crate::{
    Branch, Branchlet, DotPath,
    dot_path::{BranchTarget, PositionedRoot, TargetTypeTrait, TrueRoot},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TrueRootSheet {
    pub root: Branch,
}

impl std::fmt::Display for TrueRootSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn print_branch(branch: &Branch, level: usize) -> String {
            // Get keys from stuff and order them alphabetically
            let mut keys: Vec<_> = branch.stuff.keys().collect();
            keys.sort();
            let mut total = String::new();
            for key in keys {
                let value = branch.stuff.get(key).unwrap();
                let mut root_name_printed = String::new();
                let printed = match value {
                    crate::Branchlet::Value(value) => match value {
                        crate::data::Data::Int(i) => format!("{i}"),
                        crate::data::Data::Float(f) => format!("{f}"),
                        crate::data::Data::String(s) => format!("{s:?}"),
                    },
                    crate::Branchlet::Branch(b) => {
                        if let Some(root_name) = &b.root_name {
                            root_name_printed = format!("({}) ", root_name);
                        }
                        format!("⤸\n{}", print_branch(b, level + 1))
                    }
                };
                total.push_str(&format!(
                    "{}{root_name_printed}{key}: {printed}\n",
                    " ".repeat(level * 4)
                ));
            }

            if let Some(new) = total.strip_suffix("\n") {
                total = new.to_string();
            }

            return total;
        }

        if let Some(root_name) = &self.root.root_name {
            writeln!(f, "({})", root_name)?;
        }

        writeln!(f, "{}", print_branch(&self.root, 0))?;

        Ok(())
    }
}

impl TrueRootSheet {
    pub fn is_true_dot_path_valid<TargetType: TargetTypeTrait>(
        &self,
        true_dot_path: &DotPath<TrueRoot, TargetType>,
    ) -> bool {
        let mut current = &self.root;

        let mut iter = <Vec<std::string::String> as Clone>::clone(&true_dot_path.path).into_iter();
        while let Some(b) = iter.next() {
            if let Some(Branchlet::Branch(beep)) = current.stuff.get(&b) {
                current = beep;
            } else {
                return false;
            }
        }

        if let Some(data_target) = true_dot_path.target.data() {
            let data_name = &data_target.data_name;
            if let Some(Branchlet::Value(_)) = current.stuff.get(data_name) {
                return true;
            } else {
                return false;
            }
        }

        true
    }

    pub fn get_branch(&self, path: &DotPath<TrueRoot, BranchTarget>) -> Result<&Branch, ()> {
        if !self.is_true_dot_path_valid(path) {
            return Err(());
        }

        let mut current = &self.root;

        let mut iter = (&path.path).into_iter();
        while let Some(b) = iter.next() {
            if let Some(Branchlet::Branch(beep)) = current.stuff.get(b) {
                current = beep;
            } else {
                return Err(());
            }
        }

        Ok(current)
    }

    pub fn get_branch_mut(
        &mut self,
        path: &DotPath<TrueRoot, BranchTarget>,
    ) -> Result<&mut Branch, ()> {
        if !self.is_true_dot_path_valid(path) {
            return Err(());
        }

        let mut current = &mut self.root;

        let mut iter = (&path.path).into_iter();
        while let Some(b) = iter.next() {
            if let Some(Branchlet::Branch(beep)) = current.stuff.get_mut(b) {
                current = beep;
            } else {
                return Err(());
            }
        }

        Ok(current)
    }

    #[allow(unused_variables)]
    pub fn truly_root_dotpath<TargetType: TargetTypeTrait>(
        &self,
        positioned_dotpath: &DotPath<PositionedRoot, TargetType>,
        position: &DotPath<TrueRoot, BranchTarget>,
    ) -> Result<DotPath<TrueRoot, TargetType>, TrulyRootDotpathError> {
        if !self.is_true_dot_path_valid(position) {
            return Err(TrulyRootDotpathError::InvalidTrueRootError);
        }

        let origin_path = match &positioned_dotpath.root.origin {
            crate::dot_path::PositionedRootOrigin::Local => position.clone(),
            crate::dot_path::PositionedRootOrigin::Defined(root_name) => {
                let mut working_origin_path = position.clone();
                loop {
                    let working_branch = self
                        .get_branch(&working_origin_path)
                        .map_err(|_| TrulyRootDotpathError::ProblematicProblemHasHappend)?;

                    // The root name is either the defined root name or just the branch name.
                    let working_root_name = if let Some(beep) = &working_branch.root_name {
                        beep
                    } else {
                        working_origin_path
                            .path
                            .last()
                            .ok_or(TrulyRootDotpathError::NoDefinedRoot)?
                    };

                    if working_root_name == root_name {
                        break;
                    }

                    let _ = working_origin_path.path.pop();
                }

                working_origin_path
            }
        };

        let mut new_path = Vec::new();
        new_path.extend(origin_path.path.clone());
        new_path.extend(positioned_dotpath.path.clone());

        let beep = DotPath::<TrueRoot, TargetType> {
            path: new_path,
            root: TrueRoot,
            target: positioned_dotpath.target.clone(),
        };

        if !self.is_true_dot_path_valid(&beep) {
            return Err(TrulyRootDotpathError::InvalidTrueRootError);
        }

        Ok(beep)
    }
}

#[derive(Debug, Error)]
pub enum TrulyRootDotpathError {
    #[error("That true root can not be found.")]
    InvalidTrueRootError,

    #[error(
        "Even though a check has happened a pretty bad error has happened. Something is very wrong."
    )]
    ProblematicProblemHasHappend,

    #[error("Wasn't able to find the defined root.")]
    NoDefinedRoot,
}
