use std::{fs, path::PathBuf};
use uuid::Uuid;

/// Temporary files owned by a test. The directory is removed on drop.
pub struct Files(PathBuf);

impl Files {
    pub fn new() -> Self {
        let path = std::env::temp_dir().join(format!("sibs-test-{}", Uuid::new_v4()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    pub fn path(&self) -> &PathBuf {
        &self.0
    }

    pub fn write(&self, name: &str, content: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, content).unwrap();
        path
    }
}

impl Default for Files {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
