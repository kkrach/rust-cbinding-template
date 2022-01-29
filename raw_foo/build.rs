extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Compile libfoo with make
    Command::new("make")
        .arg("-C")
        .arg("libfoo")
        .arg("CROSS_COMPILE=")
        .output()
        .expect("failed to execute process");

    // Add dependency to libfoo
    println!("cargo:rustc-link-lib=static=foo");
    println!("cargo:rustc-link-search=raw_foo/libfoo");

    println!("cargo:rerun-if-changed=bindings.h");

    let bindings = bindgen::Builder::default()
        .header("bindings.h")
        .raw_line("use cty;")
        .ctypes_prefix("cty")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
