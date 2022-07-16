use std::env;
use std::path::PathBuf;

fn main() {
    let lzc = pkg_config::Config::new()
        .atleast_version("0.8")
        .cargo_metadata(false)
        // Yes, this is on purpose 'libzfs_core' rather than 'libzpool'
        .probe("libzfs_core")
        .expect("ZFS development environment is not installed");

    let cflags = lzc
        .include_paths
        .iter()
        .map(|path| format!("-I{}", path.display()));

    let rust_non_exhaustive_enum_style = bindgen::EnumVariation::Rust {
        non_exhaustive: true,
    };

    println!("cargo:rustc-link-lib=zpool");
    println!("cargo:rerun-if-changed=zpool.h");

    let bindings = bindgen::Builder::default()
        .header("zpool.h")
        // .clang_arg("-D_GNU_SOURCE")
        .clang_args(cflags)
        .size_t_is_usize(true)
        .ctypes_prefix("libc")
        .constified_enum_module("spa_mode")
        .allowlist_type("spa_mode_t")
        .allowlist_function(r#"kernel_\w*"#)
        .allowlist_function(r#"spa_\w*"#)
        .allowlist_function(r#"umem_\w*"#)
        .allowlist_var("zfs_arc_min")
        .allowlist_var("zfs_arc_max")
        .allowlist_var("zfs_vdev_async_read_max_active")
        .allowlist_var(r#"TXG_\w"#)
        // .allowlist_type("zfs_handle_t")
        // .allowlist_type("zpool_handle_t")
        // .allowlist_type("libzfs_handle_t")
        // .allowlist_function(r#"zpool_\w*"#)
        // .allowlist_function(r#"zfs_\w*"#)
        // .allowlist_var("ZPOOL_MAXPROPLEN")
        .blocklist_item("boolean_t")
        .blocklist_item(r#"\w*nvlist\w*"#)
        .blocklist_item(r#"\w*nvpair\w*"#)
        // .blocklist_item("zfs_type_t")
        .default_enum_style(rust_non_exhaustive_enum_style)
        .generate()
        .expect("Unable to generate bindings");

    let zpool = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR environment")
        .join("zpool.rs");

    bindings
        .write_to_file(zpool)
        .expect("Couldn't write bindings!");
}
