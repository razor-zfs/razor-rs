use std::env;
use std::path::PathBuf;

fn main() {
    let lzc = pkg_config::Config::new()
        .atleast_version("2.1.0")
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

    println!("cargo:rustc-link-lib=nvpair");
    println!("cargo:rerun-if-changed=nvpair.h");

    let bindings = bindgen::Builder::default()
        .header("nvpair.h")
        .clang_args(cflags)
        .size_t_is_usize(true)
        .ctypes_prefix("libc")
        .allowlist_var(r#"(^NV_\w*)"#)
        .allowlist_type(r#"(\w*nvpair\w*)"#)
        .allowlist_type(r#"(\w*nvlist\w*)"#)
        .allowlist_function(r#"(\w*nvpair\w*)"#)
        .allowlist_function(r#"(\w*nvlist\w*)"#)
        .default_enum_style(default_enum_style)
        .generate()
        .expect("Unable to generate bindings");

    let nvpair = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR environment")
        .join("nvpair.rs");

    bindings
        .write_to_file(nvpair)
        .expect("Couldn't write bindings!");
}
