use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system nvpair of zfs
    // shared library.
    println!("cargo:rustc-link-lib=zfs_core");
    println!("cargo:rustc-link-lib=zfs");

    let default_enum_style = bindgen::EnumVariation::Rust {
        non_exhaustive: true,
    };
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .size_t_is_usize(true)
        .ctypes_prefix("libc")
        .allowlist_type("libzfs_handle_t")
        .allowlist_type("zfs_handle_t")
        .allowlist_type("zfs_prop_t")
        .allowlist_type("zfs_type_t")
        .bitfield_enum("zfs_type_t")
        .bitfield_enum("lzc_send_flags")
        .allowlist_function("zfs_close")
        .allowlist_function("zfs_open")
        .allowlist_function(r#"zfs_iter_\w*"#)
        .allowlist_function(r#"zfs_get_\w*"#)
        .allowlist_function(r#"zfs_set_\w*"#)
        .allowlist_function(r#"zfs_prop_\w*"#)
        .allowlist_function("zfs_version_kernel")
        .allowlist_function("zfs_version_userland")
        .allowlist_function(r#"lzc\w*"#)
        .allowlist_function(r#"libzfs\w*"#)
        .blocklist_item(r#"\w*nvlist\w*"#)
        .default_enum_style(default_enum_style)
        .allowlist_type("zfs_error")
        .constified_enum_module("zfs_error")
        .clang_arg("-D_GNU_SOURCE")
        .clang_args(["-I/usr/include/libzfs", "-I/usr/include/libspl"])
        .generate()
        .expect("Unable to generate bindings");

    let zfs_core = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR environment")
        .join("zfs_core.rs");

    bindings
        .write_to_file(zfs_core)
        .expect("Couldn't write bindings!");
}
