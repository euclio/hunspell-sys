# hunspell-sys

[![Crate version](https://img.shields.io/crates/v/hunspell-sys.svg)](https://crates.io/crates/hunspell-sys)
[![Documentation](https://docs.rs/hunspell-sys/badge.svg)](https://docs.rs/hunspell-sys)
[![Build Status](https://travis-ci.com/euclio/hunspell-sys.svg?branch=master)](https://travis-ci.com/euclio/hunspell-sys)

Rust bindings for the [hunspell] C API.

[hunspell]: https://hunspell.github.io/

## Building

By default `hunspell-sys` searches for a hunspell library installation with `pkg-config`. By default the linkage is `dynamic`, if `static` is required use `static`

Optionally, the bundled code of hunspell can be compiled with the `cc` crate and will be linked `static`ally when the `bundled` feature is present. The feature `static` is not required for this, the `bundled` feature will always link the produced hunspell artifact statically.

```toml
[dependencies]
hunspell-sys = { version = "0.3.1", features = ["bundled"] }
```

### musl targets

If compiling for/on `musl` systems, `libclang` as used by `bindgen-rs`
must be linked statically as well, which can be achieved with feature
`static_libclang`.

```toml
[dependencies]
hunspell-sys = { version = "0.3.1", features = ["bundled", "static_libclang"] }
```
