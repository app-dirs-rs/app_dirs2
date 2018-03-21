# app_dirs
*Put your app's data in the right place on every platform*

[![crates.io:app_dirs2](https://img.shields.io/crates/v/app_dirs2.svg?label=crates.io%3A%20app_dirs2)](https://crates.io/crates/app_dirs2)

## This is a fork!

The original [app_dirs](https://crates.io/crates/app_dirs) crate appears essentially unmaintained
by now, so I have made this fork with the intent of making sure that it is at least maintained
and that bugs stay fixed so we can have something to rely on.  I don't intend to do any major
rearchitecting or updating, but bugs *will* get fixed and builds *will* succeed.

If you want to help maintain this, open an issue or such and we can work something out.

An alternative might be the [directories](https://crates.io/crates/directories) crate,
but when we have existing code that relies on this, forking `app_dirs` is easier than porting.

If you like it, feel free to drop a tip my the jar on [Patreon](https://www.patreon.com/icefox) or
[Liberapay](https://liberapay.com/icefox).

## Documentation & examples
https://docs.rs/app_dirs2

## Installation
Add the following to your `Cargo.toml` under `[dependencies]`:

```toml
app_dirs2 = "2"
```
