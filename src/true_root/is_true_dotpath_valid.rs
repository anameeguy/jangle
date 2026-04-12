// use crate::Data::{Branch, Value};
//
// impl TrueRoot {
//     pub fn is_true_dot_path_valid(&self, true_dot_path: &TrueDotPath) -> bool {
//         if true_dot_path.parts.is_empty() {
//             return true_dot_path.ending == Ending::Branch;
//         }

//         let mut current = &self.root;

//         for part in &true_dot_path.parts[..true_dot_path.parts.len().saturating_sub(1)] {
//             match current.stuff.get(part) {
//                 Some(Branch(branch)) => current = branch,
//                 _ => return false,
//             }
//         }

//         match (true_dot_path.parts.last(), true_dot_path.ending) {
//             (Some(last), Ending::Data) => matches!(current.stuff.get(last), Some(Value(_))),
//             (Some(last), Ending::Branch) => matches!(current.stuff.get(last), Some(Branch(_))),
//             _ => false,
//         }
//     }
// }
