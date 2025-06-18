use std::path::{Path, PathBuf};

struct EphemeralDir {
    // TODO: should we use a Box<Path> here instead of PathBuf?
    path: PathBuf,
}

impl EphemeralDir {
    pub fn new(path: impl AsRef<Path>) -> Self {
        todo!()
    }
}

impl Drop for EphemeralDir {
    fn drop(&mut self) {
        todo!()
    }
}

#[test]
fn test_dir_exists_after_creation() {
    todo!()
}

#[test]
fn test_dir_does_not_exist_after_going_out_of_scope() {
    todo!()
}
