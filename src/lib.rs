//! Bindings for the [hunspell] C API.
//!
//! [hunspell]: https://hunspell.github.io/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use std::ffi::{CStr, CString};
    use tempfile::tempdir;

    #[test]
    fn simple_sanity_test() {
        let dir = tempdir().unwrap();

        let aff = dir.path().join("foo.aff");

        std::fs::write(
            &aff,
            "SET UTF-8

SFX S Y 1
SFX S   0     s          [^sxzhy]
        ",
        )
        .unwrap();

        let dic = dir.path().join("foo.dic");

        std::fs::write(
            &dic,
            "2
cat/S
program/S
        ",
        )
        .unwrap();

        unsafe {
            let h = Hunspell_create(
                CString::new(aff.to_str().unwrap()).unwrap().as_ptr(),
                CString::new(dic.to_str().unwrap()).unwrap().as_ptr(),
            );

            assert!(h != std::ptr::null_mut());

            let cats = CString::new("cats").unwrap();

            let mut result = Vec::new();
            let mut list = std::ptr::null_mut();

            let n = Hunspell_stem(h, &mut list, cats.as_ptr());

            assert_ne!(n, 0);

            for i in 0..n {
                let item = CStr::from_ptr(*list.offset(i.try_into().unwrap()));
                result.push(item.to_str().unwrap());
            }

            assert_eq!(result, vec!["cat"]);

            Hunspell_free_list(h, &mut list, n);

            Hunspell_destroy(h);
        }
    }
}
