use std::fs;
use std::path::{Path, PathBuf};

pub fn ephemeral_dir(path: impl AsRef<Path>) -> eyre::Result<EphemeralDir> {
    EphemeralDir::new(path)
}

pub fn ephemeral_dir_forced(path: impl AsRef<Path>) -> eyre::Result<EphemeralDir> {
    EphemeralDir::new_forced(path)
}

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
        let _ = fs::remove_dir_all(self.path());
    }
}

impl AsRef<Path> for EphemeralDir {
    fn as_ref(&self) -> &Path {
        self.path()
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
}

#[test]
fn test_dir_does_not_exist_after_going_out_of_scope() {
    let path = "/tmp/ephemeral_dir_test";
    let res_dir = EphemeralDir::new_forced(path);
    assert!(res_dir.is_ok());
    {
        let Ok(dir) = res_dir else { unreachable!() };
        assert!(dir.path().exists());
        let dir_path_str = format!("{}", dir.path().display());
        assert_eq!(dir_path_str, path);
        let dir_path = Path::new(&dir_path_str);
        assert!(dir_path.exists());
    }
    let path_path = Path::new(path);
    assert!(!path_path.exists());
}

#[test]
fn test_wrapper_functions() {
    // ephemeral_dir::ephemeral_dir()
    let path = "/tmp/ephemeral_dir_test";
    let res_dir = ephemeral_dir(path);
    assert!(res_dir.is_ok());
    let Ok(dir) = res_dir else { unreachable!() };
    assert!(dir.path().exists());
    let dir_path_str = format!("{}", dir.path().display());
    assert_eq!(dir_path_str, path);

    // ephemeral_dir::ephemeral_dir_forced()
    let path = "/tmp/ephemeral_dir_test";
    let res_dir = ephemeral_dir_forced(path);
    assert!(res_dir.is_ok());
    {
        let Ok(dir) = res_dir else { unreachable!() };
        assert!(dir.path().exists());
        let dir_path_str = format!("{}", dir.path().display());
        assert_eq!(dir_path_str, path);
        let dir_path = Path::new(&dir_path_str);
        assert!(dir_path.exists());
    }
    let path_path = Path::new(path);
    assert!(!path_path.exists());
}

#[test]
fn test_asref_path() {
    let path = "/tmp/ephemeral_test_dir";
    let res_dir = ephemeral_dir(path).unwrap();

    fn foo(p: impl AsRef<Path>) -> bool {
        let p = p.as_ref();
        p.exists()
    }

    assert!(foo(&res_dir));
}
