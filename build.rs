use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join(format!("{}.h", package_name))
        .display()
        .to_string();

    // Générer le header C/C++
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&output_file);

    // Copier dans include/
    let include_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("include");
    std::fs::create_dir_all(&include_dir).ok();
    let dest_file = include_dir.join("st3215.h");
    std::fs::copy(&output_file, &dest_file).ok();

    println!("cargo:rerun-if-changed=src/ffi.rs");
}

fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
