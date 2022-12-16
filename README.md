Command line arguments by reference
===================================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/argv-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/argv)
[<img alt="crates.io" src="https://img.shields.io/crates/v/argv.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/argv)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-argv-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/argv)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/argv/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/argv/actions?query=branch%3Amaster)

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
