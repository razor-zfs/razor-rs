use std::env;
use std::path::PathBuf;

fn main() {
    let lzc = pkg_config::Config::new()
        .atleast_version("0.8")
        .cargo_metadata(false)
        // Yes, this is on purpose 'libzfs_core' rather than 'libzfs'
        .probe("libzfs_core")
        .expect("ZFS development environment is not installed");

    let cflags = lzc
        .include_paths
        .iter()
        .map(|path| format!("-I{}", path.display()));

    let default_enum_style = bindgen::EnumVariation::Rust {
        non_exhaustive: true,
    };

    println!("cargo:rustc-link-lib=zfs");
    println!("cargo:rerun-if-changed=zfs.h");

    let bindings = bindgen::Builder::default()
        .header("zfs.h")
        .clang_arg("-D_GNU_SOURCE")
        .clang_args(cflags)
        .size_t_is_usize(true)
        .ctypes_prefix("libc")
        .allowlist_type("zfs_prop_t")
        .allowlist_type("zfs_userquota_prop_t")
        .allowlist_type("zpool_prop_t")
        .allowlist_type("zfs_type_t")
        .bitfield_enum("zfs_type_t")
        .allowlist_type("zfs_error_t")
        .constified_enum_module("zfs_error")
        .allowlist_type("zfs_handle_t")
        .allowlist_type("zpool_handle_t")
        .allowlist_type("libzfs_handle_t")
        .allowlist_function(r#"libzfs_\w*"#)
        .allowlist_function(r#"zpool_\w*"#)
        .allowlist_function(r#"zfs_\w*"#)
        .allowlist_var("ZFS_MAXPROPLEN")
        .allowlist_var("ZPOOL_MAXPROPLEN")
        .blocklist_item("boolean_t")
        .blocklist_item(r#"\w*nvlist\w*"#)
        .blocklist_item(r#"\w*nvpair\w*"#)
        .default_enum_style(default_enum_style)
        .generate()
        .expect("Unable to generate bindings");

    let zfs = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR environment")
        .join("zfs.rs");

    bindings
        .write_to_file(zfs)
        .expect("Couldn't write bindings!");
}
