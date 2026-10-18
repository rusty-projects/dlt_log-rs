use std::env;
use std::path::PathBuf;

fn main() {
    let mut builder = bindgen::Builder::default();

    // find libdlt with pkg-config, which also emits the link flags
    match pkg_config::probe_library("automotive-dlt") {
        Ok(library) => {
            for path in &library.include_paths {
                builder = builder.clang_arg(format!("-I{}", path.display()));
            }
            for (name, value) in &library.defines {
                builder = builder.clang_arg(match value {
                    Some(value) => format!("-D{name}={value}"),
                    None => format!("-D{name}"),
                });
            }
        }
        Err(error) => {
            // fall back to default include and library paths
            eprintln!("pkg-config failed, using default paths: {error}");
            println!("cargo:rustc-link-lib=dlt");
        }
    }

    let bindings = builder
        .header("src/libdlt_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // list of methods we use
        .allowlist_item("dlt_register_app")
        .allowlist_item("dlt_register_context")
        .allowlist_item("dlt_unregister_app")
        .allowlist_item("dlt_unregister_context")
        .allowlist_item("dlt_log_string")
        // create rust-friendly enums
        .rustified_enum("DltReturnValue")
        .rustified_enum("DltLogLevelType")
        // the return value shall always be checked
        .must_use_type("DltReturnValue")
        // default for initialization of DltContext
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libdlt_bindings.rs"))
        .expect("Couldn't write bindings!");
}
