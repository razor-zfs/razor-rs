use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system nvpair of zfs
    // shared library.
    println!("cargo:rustc-link-lib=nvpair");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args(vec!["-I/usr/include/libzfs", "-I/usr/include/libspl"])
        .allowlist_type(r#"(\w*nvpair\w*)"#)
        .allowlist_type(r#"(\w*nvpair\w*)"#)
        .allowlist_function(r#"(\w*nvpair\w*)"#)
        .allowlist_function(r#"(\w*nvpair\w*)"#)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: (true),
        })
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    //let out_path = env::current_dir().unwrap().join("src").join("bindings.rs");
    bindings
        .write_to_file(out_path.join("nvpair.rs"))
        //.write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
