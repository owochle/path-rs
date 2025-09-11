use core::cmp::Ordering;
use crate::components::{Component, Components};

#[derive(Debug)]
#[repr(transparent)]
pub struct Path(str);

impl Path {
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> &Self {
        unsafe { &*(s.as_ref() as *const str as *const Self) }
    }

    pub fn components(&self) -> Components<'_> {
        Components::from_str(&self.0)
    }

    pub fn file_stem(&self) -> Option<&str> {
        self.file_name().map(rsplit_file_at_dot).and_then(|(before, after)| before.or(after))
    }

    pub fn file_prefix(&self) -> Option<&str> {
        self.file_name().map(split_file_at_dot).and_then(|(before, _after)| Some(before))
    }

    pub fn file_name(&self) -> Option<&str> {
        self.components().next_back().map(|x| {
            if let Component::Name(s) = x {
                Some(s)
            } else {
                None
            }
        }).flatten()
    }

    #[must_use]
    pub fn extension(&self) -> Option<&str> {
        self.file_name().map(rsplit_file_at_dot).and_then(|(before, after)| before.and(after))
    }

    pub fn is_absolute(&self) -> bool {
        self.0.starts_with('/')
    }

    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }
}

fn split_file_at_dot(file: &str) -> (&str, Option<&str>) {
    let slice = file.as_bytes();
    if slice == b".." {
        return (file, None);
    }

    // The unsafety here stems from converting between &OsStr and &[u8]
    // and back. This is safe to do because (1) we only look at ASCII
    // contents of the encoding and (2) new &OsStr values are produced
    // only from ASCII-bounded slices of existing &OsStr values.
    let i = match slice[1..].iter().position(|b| *b == b'.') {
        Some(i) => i + 1,
        None => return (file, None),
    };
    let before = &slice[..i];
    let after = &slice[i + 1..];
    unsafe {
        (
            str::from_utf8_unchecked(before),
            Some(str::from_utf8_unchecked(after)),
        )
    }
}

fn rsplit_file_at_dot(file: &str) -> (Option<&str>, Option<&str>) {
    if file.as_bytes() == b".." {
        return (Some(file), None);
    }

    let mut iter = file.as_bytes().rsplitn(2, |b| *b == b'.');
    let after = iter.next();
    let before = iter.next();
    if before == Some(b"") {
        (Some(file), None)
    } else {
        unsafe {
            (
                before.map(|s| str::from_utf8_unchecked(s)),
                after.map(|s| str::from_utf8_unchecked(s)),
            )
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.components(), &other.components())
    }
}


impl PartialEq<str> for Path {
    fn eq(&self, other: &str) -> bool {
        self.eq(Path::new(other))
    }
}

impl PartialEq<Path> for str {
    fn eq(&self, other: &Path) -> bool {
        other.eq(self)
    }
}

impl AsRef<Path> for Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

#[cfg(feature = "alloc")]
mod allocated_path {
    use alloc::borrow::ToOwned;
    use alloc::string::String;
    use core::ops::Deref;
    use crate::Path;
    use crate::path_buf::PathBuf;

    impl Path {
        pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
            let r = path.as_ref();

            if r.is_absolute() {
                r.to_owned()
            } else {
                let mut p = self.to_owned();
                for comp in path.as_ref().components() {
                    p.push_component(comp);
                }

                p
            }
        }

        pub fn resolve<P: AsRef<Path>>(&self, other: P) -> PathBuf {
            todo!()
        }
    }

    impl PartialEq<String> for Path {
        fn eq(&self, other: &String) -> bool {
            self.eq(Path::new(other))
        }
    }

    impl PartialEq<Path> for String {
        fn eq(&self, other: &Path) -> bool {
            other.eq(self)
        }
    }

    impl PartialEq<PathBuf> for Path {
        fn eq(&self, other: &PathBuf) -> bool {
            self.eq(other.deref())
        }
    }

    impl PartialEq<Path> for PathBuf {
        fn eq(&self, other: &Path) -> bool {
            other.eq(self)
        }
    }

    impl ToOwned for Path {
        type Owned = PathBuf;

        fn to_owned(&self) -> Self::Owned {
            PathBuf(String::from(self.0.to_owned()))
        }
    }

    impl From<&Path> for String {
        fn from(value: &Path) -> Self {
            value.0.to_owned()
        }
    }
}