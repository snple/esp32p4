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

    // rustc does not report the F extension in `target_feature` for the
    // `riscv32imafc` triple, so the trap handler cannot ask `cfg!`. Derive the
    // answer from the target name instead: the `f` in `imafc`/`imafdc` is the
    // only reliable signal, and the ABI follows it (`ilp32f`/`ilp32d`).
    let target = env::var("TARGET").unwrap_or_default();
    let has_fpu = target.starts_with("riscv32imaf") || target.starts_with("riscv32imafd");
    if has_fpu {
        println!("cargo:rustc-cfg=riscv_fpu");
    }
    println!("cargo:rerun-if-env-changed=TARGET");
}
