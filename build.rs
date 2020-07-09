use std::env;
use std::error::Error;
use std::path::PathBuf;

/// builds hunspell in the `vendor` git submodule with the
/// `cc` crate: ignore any hunspell's build-scripts and
/// just compile the source code to a static lib.
/// 
/// Note: list of *.cxx files might need to be updated,
/// if `vendor` git submodule is updated
#[cfg(feature = "bundled")]
fn build_or_find_hunspell() -> Result<bindgen::Builder, Box<dyn Error>> {
    let libcpp = if cfg!(target_os = "macos") {
        Some("dylib=c++")
    } else if cfg!(target_os = "linux") {
        Some("dylib=stdc++")
    } else {
        None
    };

    if let Some(link) = libcpp {
        println!("cargo:rustc-link-lib={}", link);
    }

    println!("cargo:rustc-link-lib=static=hunspell-1.7");

    cc::Build::new()
        .file("vendor/src/hunspell/affentry.cxx")
        .file("vendor/src/hunspell/affixmgr.cxx")
        .file("vendor/src/hunspell/csutil.cxx")
        .file("vendor/src/hunspell/filemgr.cxx")
        .file("vendor/src/hunspell/hashmgr.cxx")
        .file("vendor/src/hunspell/hunspell.cxx")
        .file("vendor/src/hunspell/hunzip.cxx")
        .file("vendor/src/hunspell/phonet.cxx")
        .file("vendor/src/hunspell/replist.cxx")
        .file("vendor/src/hunspell/suggestmgr.cxx")
        .define("BUILDING_LIBHUNSPELL", "1")
        .cpp(true)
        .compile("hunspell-1.7");

    Ok(bindgen::Builder::default().clang_arg(format!("-I{}", "vendor/src")))
}

#[cfg(not(feature = "bundled"))]
fn build_or_find_hunspell() -> Result<bindgen::Builder, Box<dyn Error>> {
    pkg_config::probe_library("hunspell")?;

    Ok(bindgen::Builder::default())
}

fn main() -> Result<(), Box<dyn Error>> {
    
    let builder = build_or_find_hunspell()?;

    let bindings = builder
        .header("wrapper.h")
        .generate()
        .expect("could not generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
