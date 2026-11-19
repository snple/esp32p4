use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Each transport defines defmt's global logger symbols, so enabling both only
    // shows up as a wall of "duplicate symbol: _defmt_acquire" link errors.
    if env::var_os("CARGO_FEATURE_DEFMT_RTT").is_some()
        && env::var_os("CARGO_FEATURE_DEFMT_USB").is_some()
    {
        panic!(
            "features `defmt-rtt` (RTT logger) and `defmt-usb` (USB Serial/JTAG logger) \
             are mutually exclusive: pick one, e.g. \
             `--no-default-features --features defmt-usb`"
        );
    }

    // `defmt` alone leaves the logging macros without a logger, which surfaces
    // as an "undefined reference to `_defmt_acquire`" deep in the link.
    if env::var_os("CARGO_FEATURE_DEFMT").is_some()
        && env::var_os("CARGO_FEATURE_DEFMT_RTT").is_none()
        && env::var_os("CARGO_FEATURE_DEFMT_USB").is_none()
    {
        println!(
            "cargo:warning=feature `defmt` is enabled without a logger transport; \
             add `defmt-rtt` or `defmt-usb`"
        );
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());

    // Link the `defmt` linker script, which collects the interned format
    // strings into a single `.defmt` section that the host can decode.
    // `defmt.x` itself is provided by the `defmt` crate's build script, which
    // adds its output directory to the linker search path.
    if env::var_os("CARGO_FEATURE_DEFMT").is_some() {
        println!("cargo:rustc-link-arg=-Tdefmt.x");
    }
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
