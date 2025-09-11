use core::iter::Filter;
use core::str::Split;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Component<'a> {
    Root,
    Current,
    Parent, // ..
    Name(&'a str)
}

impl<'a> Component<'a> {
    fn from_str(s: &'a str) -> Component<'a> {
        match s {
            "." => Component::Current,
            ".." => Component::Parent,
            "/" => unreachable!("Not supported"),
            s => Component::Name(s)
        }
    }
}

#[derive(Debug)]
pub struct Components<'a> {
    path: Filter<Split<'a, char>, for <'b> fn(&'b &'a str) -> bool>,
    has_root: bool
}

impl<'a> Components<'a> {
    pub(crate) fn from_str(s: &'a str) -> Components<'a> {
        Components{
            path: s.trim_end_matches('/').split('/').filter(|s| !s.is_empty()),
            has_root: s.starts_with('/')
        }
    }
}

impl PartialEq for Components<'_> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'a> Iterator for Components<'a> {
    type Item = Component<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_root {
            self.has_root = false;
            Some(Component::Root)
        } else {
            self.path.next().map(Component::from_str)
        }
    }
}

impl<'a> DoubleEndedIterator for Components<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let res = self.path.next_back().map(Component::from_str);

        if let Some(res) = res {
            Some(res)
        } else if self.has_root {
            self.has_root = false;
            Some(Component::Root)
        } else {
            None
        }
    }
}