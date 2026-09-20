use std::env;
use std::path::PathBuf;

use userspace_build::info;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let linker_script = PathBuf::from(&manifest_dir).join("linker.ld");

    info!("cargo:rerun-if-changed={}\n", linker_script.display());
    info!("cargo:rerun-if-changed=./build.rs\n");
    info!("cargo:rerun-if-changed=./src/\n");
    info!("cargo:rerun-if-changed=./crates/\n");

    info!("cargo:rustc-link-arg=-static\n");
    info!("cargo:rustc-link-arg=--no-dynamic-linker\n");
    info!("cargo:rustc-link-arg=-n\n");
    info!("cargo:rustc-link-arg=--no-pie\n");
    info!("cargo:rustc-link-arg=-T{}\n", linker_script.display());

    // Compile assembly startup code
    cc::Build::new()
        .file("src/start.s")
        .flag("-fno-pic")
        .flag("-fno-pie")
        .compile("start");
}
