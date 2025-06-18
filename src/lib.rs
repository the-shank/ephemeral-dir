use std::fs;
use std::path::{Path, PathBuf};

pub struct EphemeralDir {
    // TODO: should we use a Box<Path> here instead of PathBuf?
    path: PathBuf,
}

impl EphemeralDir {
    pub fn new(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let path = path.as_ref();
        assert!(!path.exists());

        fs::create_dir_all(path)?;

        Ok(Self {
            path: PathBuf::from(path),
        })
    }

    pub fn new_forced(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            fs::remove_dir_all(path)?;
        }

        Self::new(path)
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

impl Drop for EphemeralDir {
    fn drop(&mut self) {
        todo!()
    }
}

#[test]
fn test_dir_exists_after_creation() {
    let path = "/tmp/ephemeral_dir_test";
    let res_dir = EphemeralDir::new_forced(path);
    assert!(res_dir.is_ok());
    let Ok(dir) = res_dir else { unreachable!() };
    assert!(dir.path().exists());
    let dir_path_str = format!("{}", dir.path().display());
    assert_eq!(dir_path_str, path);
    todo!()
}

#[test]
fn test_dir_does_not_exist_after_going_out_of_scope() {
    todo!()
}
