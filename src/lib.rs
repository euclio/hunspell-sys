//! Bindings for the [hunspell] C API.
//!
//! [hunspell]: https://hunspell.github.io/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
