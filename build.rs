use std::env;
use std::error::Error;
use std::path::PathBuf;

/// builds hunspell in the `vendor` git submodule with the
/// `cc` crate: ignore any hunspell's build-scripts and
/// just compile the source code to a static lib.
///
/// This is a workaround on targets without autotools (i.e. Windows).
/// Otherwise use the `build-autotools` feature to build
/// hunspell using its build scripts.
#[cfg(feature = "build-cc")]
fn build_hunspell(builder: bindgen::Builder) -> bindgen::Builder {
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

    builder.clang_arg(format!("-I{}", "vendor/src"))
}

/// builds hunspell in the `vendor` git submodule with the
/// `autotools` crate: relays hunspell's build-scripts and
/// just compile the source code to a static lib.
///
/// This is the default, but only works when autotools are
/// installed.
/// Without autotools prefere the `build-cc` feature.
#[cfg(feature = "build-autotools")]
fn build_hunspell(builder: bindgen::Builder) -> bindgen::Builder {
    let dst = autotools::Config::new("vendor")
        .reconf("-ivf")
        .cxxflag("-fPIC")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    builder.clang_arg(format!("-I{}", dst.join("include").display()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = bindgen::Builder::default();

    let libcpp = if cfg!(target_os = "macos") {
        Some("dylib=c++")
    } else if cfg!(target_os = "linux") {
        Some("dylib=stdc++")
    } else {
        None
    };


    if pkg_config::probe_library("hunspell").is_err() {

        println!("cargo:rustc-link-lib=static=hunspell-1.7");

        if let Some(link) = libcpp {
            println!("cargo:rustc-link-lib={}", link);
        }

        builder = build_hunspell(builder);
    }

    let bindings = builder
        .header("wrapper.h")
        .generate()
        .expect("could not generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
