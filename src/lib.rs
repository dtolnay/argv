//! [![github]](https://github.com/dtolnay/argv)&ensp;[![crates-io]](https://crates.io/crates/argv)&ensp;[![docs-rs]](https://docs.rs/argv)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Command line arguments by reference: `Iterator<Item = &'static OsStr>`
//!
//! The standard library's [`std::env::args_os`] iterator produces an owned
//! string (`OsString`) for each argument. In some use cases it can be more
//! convenient for the arguments to be produced by static reference (`&'static
//! OsStr`).
//!
//! [`std::env::args_os`]: https://doc.rust-lang.org/std/env/fn.args_os.html
//!
//! # Examples
//!
//! ```
//! fn main() {
//!     for arg in argv::iter() {
//!         // arg is a &'static OsStr.
//!         println!("{}", arg.to_string_lossy());
//!     }
//! }
//! ```
//!
//! # Portability
//!
//! This crate is intended to be used on Linux and macOS, on which command line
//! arguments naturally live for the duration of the program. This crate
//! implements the same API on other platforms as well, such as Windows, but
//! leaks memory on platforms other than Linux and macOS.

#![doc(html_root_url = "https://docs.rs/argv/0.1.3")]
#![allow(clippy::must_use_candidate, clippy::similar_names)]

use std::ffi::OsStr;

/// Returns an iterator over command line arguments.
pub fn iter() -> Iter {
    Iter {
        platform_specific: crate::r#impl::iter(),
    }
}

/// Iterator over command line arguments.
pub struct Iter {
    platform_specific: crate::r#impl::Iter,
}

impl Iterator for Iter {
    type Item = &'static OsStr;

    fn next(&mut self) -> Option<Self::Item> {
        self.platform_specific.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.platform_specific.size_hint()
    }
}

impl ExactSizeIterator for Iter {
    fn len(&self) -> usize {
        self.platform_specific.len()
    }
}

#[cfg(all(target_os = "linux", not(target_env = "musl")))]
mod r#impl {
    use std::ffi::{CStr, OsStr};
    use std::os::raw::{c_char, c_int};
    use std::os::unix::ffi::OsStrExt;
    use std::ptr;
    use std::slice;

    static mut ARGC: c_int = 0;
    static mut ARGV: *const *const c_char = ptr::null();

    #[cfg(target_os = "linux")]
    #[link_section = ".init_array"]
    #[used]
    static CAPTURE: unsafe extern "C" fn(c_int, *const *const c_char) = capture;

    // Disabled for now until we investigate https://github.com/dtolnay/argv/issues/1
    #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
    #[allow(dead_code)]
    unsafe extern "C" fn capture(argc: c_int, argv: *const *const c_char) {
        ARGC = argc;
        ARGV = argv;
    }

    pub fn iter() -> Iter {
        // These are only mutated before main so they are safe to read once main
        // has begun.
        let argc = unsafe { ARGC };
        let argv = unsafe { ARGV };

        // We count on the OS to provide argv for which argv + argc does not
        // overflow.
        let argv = unsafe { slice::from_raw_parts(argv, argc as usize) };

        Iter { inner: argv.iter() }
    }

    pub struct Iter {
        inner: slice::Iter<'static, *const c_char>,
    }

    impl Iterator for Iter {
        type Item = &'static OsStr;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|&ptr| {
                let c_str = unsafe { CStr::from_ptr(ptr) };
                OsStr::from_bytes(c_str.to_bytes())
            })
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.inner.size_hint()
        }
    }

    impl ExactSizeIterator for Iter {
        fn len(&self) -> usize {
            self.inner.len()
        }
    }
}

#[cfg(any(not(target_os = "linux"), target_env = "musl"))]
mod r#impl {
    use once_cell::sync::OnceCell;
    use std::ffi::OsStr;
    use std::{env, iter, slice};

    static ARGV: OnceCell<Vec<&'static OsStr>> = OnceCell::new();

    pub fn iter() -> Iter {
        let v = ARGV.get_or_init(|| {
            env::args_os()
                .map(|arg| -> &OsStr { Box::leak(arg.into_boxed_os_str()) })
                .collect()
        });
        v.iter().copied()
    }

    pub type Iter = iter::Copied<slice::Iter<'static, &'static OsStr>>;
}
