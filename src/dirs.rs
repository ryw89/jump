use std::path::PathBuf;

use directories::BaseDirs;

pub fn get_data_dir() -> PathBuf {
    BaseDirs::new().unwrap().data_dir().join("jump")
}
