extern crate bindgen;

use std::env;
use std::path::PathBuf;

use glob::glob;

fn main() {
    // Tell cargo to tell rustc to link the objectbox shared library.
    println!("cargo:rustc-link-lib=objectbox");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("src/objectbox.h")
        // Some settings
        .whitelist_function("obx_.*")
        .whitelist_type("OBX_.*")
        .whitelist_var("OBX_.*")
        .prepend_enum_name(false)
        .derive_copy(false)
        .derive_debug(false)
        .derive_default(false)
        .rustfmt_bindings(true)
        .generate_comments(false) // generated comments breaks rust
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("objectbox-c-bindings.rs"))
        .expect("Couldn't write bindings!");

    // Read <entity>.objectbox.info and consolidate into
    // objectbox-model.json & objectbox-generated.rs

    let glob_path = format!("{}.objectbox.info", out_path.to_str().unwrap_or_default());
    for entry in glob(&glob_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file = format!("{:?}", path.display());
                
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
}
