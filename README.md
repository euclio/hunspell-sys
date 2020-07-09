# hunspell-sys

[![Crate version](https://img.shields.io/crates/v/hunspell-sys.svg)](https://crates.io/crates/hunspell-sys)
[![Documentation](https://docs.rs/hunspell-sys/badge.svg)](https://docs.rs/hunspell-sys)
[![Build Status](https://travis-ci.com/euclio/hunspell-sys.svg?branch=master)](https://travis-ci.com/euclio/hunspell-sys)

Rust bindings for the [hunspell] C API.

[hunspell]: https://hunspell.github.io/

## Building

By default `hunspell-sys` searches while building for hunspell library with `pkg-config`.

Optionally, the bundled code of hunspell can be compiled with the `cc` crate.
This behavior needs to activated by the `bundled` feature.

```toml
[dependencies]
hunspell-sys = { version = 0.1.3, features = ["bundled"] }
```
