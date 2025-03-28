//! [![github]](https://github.com/dtolnay/argv)&ensp;[![crates-io]](https://crates.io/crates/argv)&ensp;[![docs-rs]](https://docs.rs/argv)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
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

#![doc(html_root_url = "https://docs.rs/argv/0.1.12")]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(
    clippy::cast_sign_loss,
    clippy::extra_unused_type_parameters,
    clippy::let_underscore_untyped,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::similar_names
)]

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
    use std::mem;
    use std::os::raw::{c_char, c_int};
    use std::os::unix::ffi::OsStrExt;
    use std::ptr;

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
        unsafe {
            ARGC = argc;
            ARGV = argv;
        }
    }

    pub(crate) fn iter() -> Iter {
        // These are only mutated before main so they are safe to read once main
        // has begun.
        let argc = unsafe { ARGC };
        let argv = unsafe { ARGV };

        // We count on the OS to provide argv for which argv + argc does not
        // overflow.
        let end = unsafe { argv.offset(argc as isize) };

        Iter { next: argv, end }
    }

    pub(crate) struct Iter {
        next: *const *const c_char,
        end: *const *const c_char,
    }

    impl Iterator for Iter {
        type Item = &'static OsStr;

        fn next(&mut self) -> Option<Self::Item> {
            if ptr::eq(self.next, self.end) {
                None
            } else {
                let ptr = unsafe { *self.next };
                let c_str = unsafe { CStr::from_ptr(ptr) };
                self.next = unsafe { self.next.offset(1) };
                Some(OsStr::from_bytes(c_str.to_bytes()))
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.len();
            (len, Some(len))
        }
    }

    impl ExactSizeIterator for Iter {
        fn len(&self) -> usize {
            (self.end as usize - self.next as usize) / mem::size_of::<*const c_char>()
        }
    }

    // Thread safe despite the raw pointers.
    unsafe impl Send for Iter {}
    unsafe impl Sync for Iter {}
}

#[cfg(any(not(target_os = "linux"), target_env = "musl"))]
mod r#impl {
    use std::ffi::OsStr;
    use std::sync::Once;
    use std::{env, iter, ptr, slice};

    static ONCE: Once = Once::new();
    static mut ARGV: Vec<&'static OsStr> = Vec::new();

    pub(crate) fn iter() -> Iter {
        ONCE.call_once(|| {
            let argv = env::args_os()
                .map(|arg| -> &OsStr { Box::leak(arg.into_boxed_os_str()) })
                .collect();
            unsafe { ARGV = argv }
        });
        let argv = unsafe { &*ptr::addr_of!(ARGV) };
        argv.iter().copied()
    }

    pub(crate) type Iter = iter::Copied<slice::Iter<'static, &'static OsStr>>;
}

const _AUTO_TRAITS: () = {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    let _ = assert_send::<Iter>;
    let _ = assert_sync::<Iter>;
};
