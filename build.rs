use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process;

use autotools;

fn run() -> Result<(), Box<Error>> {
    let mut builder = bindgen::Builder::default();

    let libcpp = if cfg!(target_os = "macos") {
        Some("dylib=c++")
    } else if cfg!(target_os = "linux") {
        Some("dylib=stdc++")
    } else {
        None
    };

    if pkg_config::probe_library("hunspell").is_err() {
        let dst = autotools::Config::new("vendor")
            .reconf("-ivf")
            .cxxflag("-fPIC")
            .build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
        println!("cargo:rustc-link-lib=static=hunspell-1.7");

        if let Some(link) = libcpp {
            println!("cargo:rustc-link-lib={}", link);
        }

        builder = builder.clang_arg(format!("-I{}", dst.join("include").display()));
    }

    let bindings = builder
        .header("wrapper.h")
        .generate()
        .expect("could not generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
