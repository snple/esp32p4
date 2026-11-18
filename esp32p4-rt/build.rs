//! Build script for esp32p4-rt.
//!
//! Copies the linker script into `OUT_DIR` and puts that directory on the
//! linker search path, so applications can pass `-Tlink.x`.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("link.x"), include_bytes!("link.x")).unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=link.x");
    println!("cargo:rerun-if-changed=build.rs");
}
