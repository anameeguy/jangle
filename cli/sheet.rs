use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use jangle::TrueRootSheet;
use ron::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Sheet {
    pub is_sheet: IsSheet,
    pub true_root: TrueRootSheet,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct IsSheet;

impl Sheet {
    /// Checks if there is a proper sheet at the path.
    pub fn is_sheet_at_path(path: &PathBuf) -> bool {
        if !path.is_file() {
            println!("1");
            return false;
        }
        let path = path;
        let contents = fs::read_to_string(path).expect("Unable to read working sheet file.");
        if let Ok(value) = ron::from_str::<Value>(&contents) {
            if let Value::Map(map) = value {
                for (k, _) in map.iter() {
                    if let Value::String(tk) = k {
                        if *tk == "is_sheet".to_string() {
                            return true;
                        }
                    }
                }

                false
            } else {
                false
            }
        } else {
            println!("3");
            false
        }
    }

    pub fn save(&self, path: &PathBuf) {
        let mut file = File::create(path).expect("Unable to create sheet file.");
        let s = ron::to_string(self).unwrap();
        file.write_all(s.as_bytes())
            .expect("Unable to write to sheet file.");
    }

    pub fn load<P: Into<PathBuf>>(path: P) -> Self {
        let contents = fs::read_to_string(path.into()).expect("Unable to read sheet file.");
        ron::from_str(&contents).unwrap()
    }
}
