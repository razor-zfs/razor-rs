use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system nvpair of zfs
    // shared library.
    println!("cargo:rustc-link-lib=zfs_core");
    println!("cargo:rustc-link-lib=zfs");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .size_t_is_usize(true)
        .ctypes_prefix("libc")
        .allowlist_type(r#"zfs_type_dataset"#)
        .allowlist_type(r#"zfs_handle_t"#)
        .allowlist_type(r#"zfs_error_t"#)
        .allowlist_type(r#"libzfs_handle_t"#)
        .allowlist_function(r#"hasmntopt"#)
        .allowlist_type(r#"zfs_prop_t"#)
        .allowlist_type("zfs_type_t")
        .bitfield_enum("zfs_type_t")
        .allowlist_function(r#"zfs_strdup"#)
        .allowlist_function(r#"zfs_iter_\w*"#)
        .allowlist_function(r#"zfs_prop_default_numeric"#)
        .allowlist_function(r#"zfs_prop_default_string"#)
        .allowlist_function(r#"libzfs_mnttab_find"#)
        .allowlist_function(r#"libzfs_init"#)
        .allowlist_function(r#"zfs_close"#)
        .allowlist_function(r#"zfs_open"#)
        .allowlist_function(r#"zfs_get_\w*"#)
        .allowlist_function(r#"zfs_set_\w*"#)
        .allowlist_function(r#"zfs_prop_\w*"#)
        .allowlist_function(r#"make_dataset_handle"#)
        .allowlist_function(r#"lzc\w*"#)
        .allowlist_function(r#"libzfs\w*"#)
        .blocklist_item(r#"\w*nvlist\w*"#)
        .blocklist_type(r#"\w*nvlist\w*"#)
        .blocklist_function(r#"\w*nvlist\w*"#)
        .clang_args(vec!["-I/usr/include/libzfs", "-I/usr/include/libspl"])
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
        //.write_to_file(out_path)
        .write_to_file(out_path.join("zfs_core.rs"))
        .expect("Couldn't write bindings!");
}
