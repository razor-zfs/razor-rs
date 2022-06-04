use std::env;
use std::path::PathBuf;

fn main() {
    let lzc = pkg_config::Config::new()
        .atleast_version("0.8")
        .cargo_metadata(false)
        .probe("libzfs_core")
        .expect("ZFS development environment is not installed");

    let cflags = lzc
        .include_paths
        .iter()
        .map(|path| format!("-I{}", path.display()));

    let default_enum_style = bindgen::EnumVariation::Rust {
        non_exhaustive: true,
    };

    println!("cargo:rustc-link-lib=zfs_core");
    println!("cargo:rustc-link-lib=zfs");
    println!("cargo:rerun-if-changed=zfscore.h");

    let bindings = bindgen::Builder::default()
        .header("zfscore.h")
        .clang_arg("-D_GNU_SOURCE")
        .clang_args(cflags)
        .size_t_is_usize(true)
        .ctypes_prefix("libc")
        .allowlist_type("libzfs_handle_t")
        .allowlist_type("zfs_handle_t")
        .allowlist_type("zfs_prop_t")
        .allowlist_type("zfs_type_t")
        .allowlist_type("zfs_error_t")
        .bitfield_enum("zfs_type_t")
        .bitfield_enum("lzc_send_flags")
        .allowlist_function("zfs_close")
        .allowlist_function("zfs_open")
        .allowlist_function("zfs_version_kernel")
        .allowlist_function("zfs_version_userland")
        .allowlist_function(r#"zfs_iter_\w*"#)
        .allowlist_function(r#"zfs_get_\w*"#)
        .allowlist_function(r#"zfs_set_\w*"#)
        .allowlist_function(r#"zfs_prop_\w*"#)
        .allowlist_function(r#"lzc\w*"#)
        .allowlist_function(r#"libzfs\w*"#)
        .allowlist_var("ZFS_MAXPROPLEN")
        .allowlist_var("ZPOOL_MAXPROPLEN")
        .blocklist_item(r#"\w*nvlist\w*"#)
        .default_enum_style(default_enum_style)
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
