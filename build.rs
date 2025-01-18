use std::env;
use std::path::PathBuf;

// see https://rust-lang.github.io/rust-bindgen/tutorial-3.html
fn main() {
    println!("cargo:rustc-link-lib=dlt");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_item("DltContext")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
