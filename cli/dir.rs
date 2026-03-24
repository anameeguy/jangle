use std::path::PathBuf;

use directories::ProjectDirs;

pub fn get_session_dir() -> PathBuf {
    let proj_dirs = ProjectDirs::from("net", "dabeep", "jangle")
        .expect("You really shouldn't be seeing this error.");
    let cache_dir = proj_dirs.cache_dir();
    cache_dir.to_path_buf()
}
