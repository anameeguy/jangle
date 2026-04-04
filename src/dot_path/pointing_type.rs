use serde::{Deserialize, Serialize};

use crate::PointingType;

pub(crate) trait PointingTypeTrait {
    const ENUM: PointingType;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct DataPointingType;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct BranchPointingType;

impl PointingTypeTrait for DataPointingType {
    const ENUM: PointingType = PointingType::Data;
}
impl PointingTypeTrait for BranchPointingType {
    const ENUM: PointingType = PointingType::Branch;
}

pub fn get_pointing_type<T: PointingTypeTrait>() -> PointingType {
    T::ENUM
}
