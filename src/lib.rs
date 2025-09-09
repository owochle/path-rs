#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
mod path_buf;
#[cfg(feature = "alloc")]
pub use path_buf::PathBuf;

mod path;

mod components;

pub use path::Path;