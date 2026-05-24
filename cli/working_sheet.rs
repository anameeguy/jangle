use std::{
    fs::{self, File, canonicalize},
    io::Write,
    path::PathBuf,
};

use jangle::{
    DotPath,
    dot_path::{BranchTarget, TrueRoot},
};
use serde::{Deserialize, Serialize};

use crate::dir;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkingSheetCache {
    pub working_sheet_path: PathBuf, // TODO Make this an `Option`.
    pub working_branch_path: Option<DotPath<TrueRoot, BranchTarget>>,
}

impl WorkingSheetCache {
    pub fn get_cache_path() -> PathBuf {
        dir::get_session_dir().join("working_sheet_cache.ron")
    }

    pub fn get() -> Self {
        let path = WorkingSheetCache::get_cache_path();
        let contents = fs::read_to_string(path).expect("Unable to read working sheet cache file.");

        ron::from_str(&contents).expect("Unable to parse working sheet cache file.")
    }

    pub fn set(&self) {
        let path = WorkingSheetCache::get_cache_path();
        let mut file = File::create(path).expect("Unable to create cache file.");
        let s = ron::to_string(self).unwrap();
        file.write_all(s.as_bytes())
            .expect("Unable to write create cache file.");
    }

    pub fn exists() -> bool {
        Self::get_cache_path().is_file()
    }
}

pub fn set_working_sheet<P: Into<PathBuf>>(path: P) {
    reset_if_needed();
    let mut cache = WorkingSheetCache::get();
    cache.working_sheet_path = cannon(path.into());
    cache.set();
}

fn reset_if_needed() {
    let path = WorkingSheetCache::get_cache_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Unable to create cache directory.");
    }

    if !WorkingSheetCache::exists() {
        WorkingSheetCache::default().set();
    }
}

/// Just turns a path into an absoultue path.
fn cannon(path: PathBuf) -> PathBuf {
    canonicalize(&path).expect("Failed to do that cannon thing or something.")
}

/// This is for use in a single command.
pub fn get_working_sheet_location() -> Option<PathBuf> {
    if WorkingSheetCache::exists() {
        return Some(WorkingSheetCache::get().working_sheet_path);
    } else {
        return None;
    }
}
