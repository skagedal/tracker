use std::path::{Path, PathBuf};

use directories::ProjectDirs;

pub struct TrackerDirs {
    config_dir: PathBuf,
    data_dir: PathBuf,
}

impl TrackerDirs {
    pub fn real() -> TrackerDirs {
        let proj_dirs = ProjectDirs::from("tech", "skagedal", "tracker").unwrap();
        TrackerDirs {
            config_dir: proj_dirs.config_dir().to_path_buf(),
            data_dir: proj_dirs.data_dir().to_path_buf(),
        }
    }

    pub fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }

    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }
}
