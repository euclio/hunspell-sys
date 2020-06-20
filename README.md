# hunspell-sys

[![Crate version](https://img.shields.io/crates/v/hunspell-sys.svg)](https://crates.io/crates/hunspell-sys)
[![Documentation](https://docs.rs/hunspell-sys/badge.svg)](https://docs.rs/hunspell-sys)
[![Build Status](https://travis-ci.com/euclio/hunspell-sys.svg?branch=master)](https://travis-ci.com/euclio/hunspell-sys)

Rust bindings for the [hunspell] C API.

[hunspell]: https://hunspell.github.io/

## Building

By default `hunspell-sys` searches while building for hunspell library with `pkg-config`.
If not found, then the hunspell library form the `vendor` git submodule will be build with autotools.

If autotools are unavailable (e.g. on Windows), then one can configure `hunspell-sys` to build the hunspell library without autotools with the `build-cc` feature:

```toml
[dependencies]
hunspell-sys = { version = 0.1.3, default-features = false, features = ["build-cc"] }
```

Alternatively, it is possible to specify the features target dependend. For instance, for the `msvc` target environment, where autotools are not available:

```toml
[target.'cfg(target_env = "msvc")'.dependencies]
hunspell-sys = { version = "0.1.3", default-features = false, features = ["build-cc"] }

[target.'cfg(not(target_env = "msvc"))'.dependencies]
hunspell-sys = "0.1.3"
```
