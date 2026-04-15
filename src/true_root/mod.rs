mod truly_root_dotpath;

use crate::{
    Branch, DotPath,
    dot_path::{TargetTypeTrait, TrueRoot},
};
use serde::{Deserialize, Serialize};

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
                    crate::Data::Value(value) => match value {
                        crate::data::Value::Int(i) => format!("{i}"),
                        crate::data::Value::Float(f) => format!("{f}"),
                        crate::data::Value::String(s) => format!("{s:?}"),
                    },
                    crate::Data::Branch(b) => {
                        if let Some(root_name) = &b.root_name {
                            root_name_printed = format!("({}) ", root_name);
                        }
                        format!("⤸\n{}", print_branch(b, level + 1))
                    }
                };
                total.push_str(&format!(
                    "{}{root_name_printed}{key}: {printed}\n",
                    "\t".repeat(level)
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
        // if true_dot_path.parts.is_empty() {
        //     return true_dot_path.ending == Ending::Branch;
        // }

        // let mut current = &self.root;

        // for part in &true_dot_path.parts[..true_dot_path.parts.len().saturating_sub(1)] {
        //     match current.stuff.get(part) {
        //         Some(Branch(branch)) => current = branch,
        //         _ => return false,
        //     }
        // }

        // match (true_dot_path.parts.last(), true_dot_path.ending) {
        //     (Some(last), Ending::Data) => matches!(current.stuff.get(last), Some(Value(_))),
        //     (Some(last), Ending::Branch) => matches!(current.stuff.get(last), Some(Branch(_))),
        //     _ => false,
        // }
        todo!()
    }
}
