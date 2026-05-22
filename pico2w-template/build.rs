//! Set up the RP2350 linker script.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    let mut f = File::create(out.join("memory.x")).unwrap();
    f.write_all(include_bytes!("rp2350.x")).unwrap();
    println!("cargo:rerun-if-changed=rp2350.x");
    println!("cargo:rerun-if-changed=build.rs");
}
