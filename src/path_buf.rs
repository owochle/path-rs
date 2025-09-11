use alloc::borrow::ToOwned;
use alloc::string::String;
use core::borrow::Borrow;
use core::ops::Deref;
use core::str::FromStr;
use crate::components::Component;
use crate::path::Path;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PathBuf(pub(crate) String);

impl PathBuf {
    fn push(&mut self, s: &str) {
        if self.0.ends_with('/') {
            self.0.push_str(s)
        } else {
            self.0.push('/');
            self.0.push_str(s)
        }
    }
    fn pop(&mut self) {
        let Some(parent) = self.0.trim_end_matches('/').rfind('/') else {
            return
        };
        self.0.replace_range(parent..self.0.len(), "");
    }

    pub(crate) fn push_component(&mut self, component: Component) {
        match component {
            Component::Root => {
                self.0 = String::from("/");
            }
            Component::Current => {}
            Component::Parent if self.0 == "/" => {
                return
            }
            Component::Parent => {
                self.pop()
            }
            Component::Name(n) => {
                self.push(n)
            }
        }
    }
}

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

impl FromStr for PathBuf {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PathBuf::from(s))
    }
}

impl PartialEq for PathBuf {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other)
    }
}