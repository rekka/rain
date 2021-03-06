use std::path::{Path, PathBuf};

use common::id::SubworkerId;

pub struct LogDir {
    path: PathBuf,
}

impl LogDir {
    pub fn new(path: PathBuf) -> Self {
        let sw = path.join("subworkers");
        if !sw.exists() {
            ::std::fs::create_dir(&sw).unwrap();
        }
        LogDir { path }
    }

    /// Get path to logs for subworker
    pub fn subworker_log_paths(&self, id: SubworkerId) -> (PathBuf, PathBuf) {
        let out = self.path
            .join(Path::new(&format!("subworkers/subworker-{}.out", id)));
        let err = self.path
            .join(Path::new(&format!("subworkers/subworker-{}.err", id)));
        (out, err)
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
