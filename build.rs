use std::env;
use std::path::PathBuf;

// see https://rust-lang.github.io/rust-bindgen/tutorial-3.html
fn main() {
    println!("cargo:rustc-link-lib=dlt");

    let bindings = bindgen::Builder::default()
        .header("src/libdlt_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_item("dlt_register_app")
        .must_use_type("DltReturnValue")
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libdlt_bindings.rs"))
        .expect("Couldn't write bindings!");
}
