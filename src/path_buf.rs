use alloc::borrow::ToOwned;
use alloc::string::String;
use core::borrow::Borrow;
use core::ops::Deref;
use crate::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathBuf(pub(crate) String);

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl Deref for PathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        Path::new(&self.0)
    }
}

impl Borrow<Path> for PathBuf {
    fn borrow(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl From<String> for PathBuf {
    fn from(value: String) -> Self {
        PathBuf(value)
    }
}

impl From<&str> for PathBuf {
    fn from(value: &str) -> Self {
        PathBuf(value.to_owned())
    }
}

impl From<PathBuf> for String {
    fn from(value: PathBuf) -> Self {
        value.0
    }
}