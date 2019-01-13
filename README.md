Command line arguments by reference
===================================

[![Build Status](https://api.travis-ci.com/dtolnay/argv.svg?branch=master)](https://travis-ci.com/dtolnay/argv)
[![Latest Version](https://img.shields.io/crates/v/argv.svg)](https://crates.io/crates/argv)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/argv)

The standard library's [`std::env::args_os`] iterator produces an owned string
(`OsString`) for each argument. In some use cases it can be more convenient for
the arguments to be produced by static reference (`&'static OsStr`).

[`std::env::args_os`]: https://doc.rust-lang.org/std/env/fn.args_os.html

```toml
[dependencies]
argv = "0.1"
```

## Example

```rust
fn main() {
    for arg in argv::iter() {
        // arg is a &'static OsStr.
        println!("{}", arg.to_string_lossy());
    }
}
```

## Portability

This crate is intended to be used on Linux and macOS, on which command line
arguments naturally live for the duration of the program. This crate implements
the same API on other platforms as well, such as Windows, but leaks memory on
platforms other than Linux and macOS.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
