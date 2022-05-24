use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=nvpair");
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args(vec!["-I/usr/include/libzfs", "-I/usr/include/libspl"])
        .size_t_is_usize(true)
        .ctypes_prefix("libc")
        .allowlist_var(r#"(^NV_\w*)"#)
        .allowlist_type(r#"(\w*nvpair\w*)"#)
        .allowlist_type(r#"(\w*nvlist\w*)"#)
        .allowlist_function(r#"(\w*nvpair\w*)"#)
        .allowlist_function(r#"(\w*nvlist\w*)"#)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: (true),
        })
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let nvpair = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR environment")
        .join("nvpair.rs");

    bindings
        .write_to_file(nvpair)
        .expect("Couldn't write bindings!");
}
