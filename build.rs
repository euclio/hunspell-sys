use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process;

fn run() -> Result<(), Box<Error>> {
    pkg_config::probe_library("hunspell")?;

    let bindings = bindgen::Builder::default()
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
