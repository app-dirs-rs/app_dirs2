# app_dirs2

*Put your app's data in the right place on every platform*

[![crates.io:app_dirs2](https://img.shields.io/crates/v/app_dirs2.svg?label=crates.io%3A%20app_dirs2)](https://lib.rs/crates/app_dirs2)

## This is the up-to-date version of `app_dirs`

The original [app_dirs](https://lib.rs/crates/app_dirs) crate is deprecated and unmaintained. This is a drop-in-replacement fork that keeps the crate working and up-to-date.

This is a *community-maintained project*, so if you find a bug or the crate is missing support for your platform, please help out.

There are no major changes planned. If you're looking for a crate with more features, check out the [directories](https://lib.rs/crates/directories) crate.

## Documentation & examples

https://docs.rs/app_dirs2

## Installation

Add the following to your `Cargo.toml` under `[dependencies]`:

```toml
app_dirs = { package = "app_dirs2", version = "2.5" }
```

The syntax with `package` allows you to keep the old name in the code (`use app_dirs::*`), so you only need to change one line in `Cargo.toml`.
