use std::env;
use std::path::PathBuf;

fn main() {
    // system library is libdlt
    println!("cargo:rustc-link-lib=dlt");

    let bindings = bindgen::Builder::default()
        .header("src/libdlt_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // list of methods we use
        .allowlist_item("dlt_register_app")
        .allowlist_item("dlt_register_context")
        .allowlist_item("dlt_log_string")
        // the return value shall always be checked
        .must_use_type("DltReturnValue")
        // default for initialization of DltContext
        .derive_default(true)
        // avoid unncessary prepending, there are no conflicts
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libdlt_bindings.rs"))
        .expect("Couldn't write bindings!");
}
