// use thiserror::Error;
//
// impl TrueRoot {
//     #[allow(unused_variables)]
//     pub fn truly_root_dotpath(
//         &self,
//         positioned_dotpath: &PositionedDotPath,
//         working_place: &TrueDotPath,
//     ) -> Result<TrueDotPath, TrulyRootDotpathError> {
//         if !self.is_true_dot_path_valid(working_place) {
//             return Err(TrulyRootDotpathError::InvalidTrueRootError);
//         }

//         let original_working_branch = match working_place.ending {
//             Data => {
//                 let mut thingy = working_place.clone();
//                 let _ = thingy.parts.pop();
//                 thingy.ending = Branch;
//                 thingy
//             }
//             Branch => working_place.clone(),
//         };

//         let starting_point = match &positioned_dotpath.root {
//             Local => positioned_dotpath,
//             Root(root_name) => todo!(),
//         };

//         todo!()
//     }
// }

// #[derive(Debug, Error)]
// pub enum TrulyRootDotpathError {
//     #[error("That true root can not be found.")]
//     InvalidTrueRootError,
// }
