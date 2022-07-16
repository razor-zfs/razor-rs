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
    println!("cargo:rerun-if-changed=zfscore.h");

    let bindings = bindgen::Builder::default()
        .header("zfscore.h")
        .clang_arg("-D_GNU_SOURCE")
        .clang_args(cflags)
        .size_t_is_usize(true)
        .ctypes_prefix("libc")
        .allowlist_type("zfs_prop_t")
        .allowlist_type("zfs_userquota_prop_t")
        .allowlist_type("zpool_prop_t")
        .allowlist_type("zfs_type_t")
        .bitfield_enum("zfs_type_t")
        .bitfield_enum("lzc_send_flags")
        .allowlist_function(r#"lzc\w*"#)
        .allowlist_function(r#"libzfs_core_\w*"#)
        .allowlist_var(r#"ZPOOL_CONFIG_\w*"#)
        .allowlist_var(r#"ZPOOL_LOAD_\w*"#)
        .blocklist_item("boolean_t")
        .blocklist_item(r#"\w*nvlist\w*"#)
        .blocklist_item(r#"\w*nvpair\w*"#)
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
