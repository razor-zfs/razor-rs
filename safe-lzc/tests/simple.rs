use std::env;

use nanoid::nanoid;
use nanoid_dictionary::ALPHANUMERIC;

use razor_nvpair as nvpair;
use razor_safe_lzc as lzc;

struct LzcTestNamespace {
    parent: String,
}

impl LzcTestNamespace {
    fn new() -> Self {
        let parent = env::var("RAZOR_TEST")
            .ok()
            .unwrap_or_else(|| "rpool/razor-test".to_string());
        Self { parent }
    }

    fn name(&self) -> String {
        format!("{}/{}", self.parent, nanoid!(8, ALPHANUMERIC))
    }
}

#[test]
fn filesystem_create_destroy() {
    let namespace = LzcTestNamespace::new();
    let name = dbg!(namespace.name());
    let mut props = nvpair::NvList::new();
    props += ("razor-test:mountpoint", "/xxx");
    lzc::create_filesystem(&name, props).unwrap();
    assert!(lzc::dataset_exists(&name));
    lzc::destroy_dataset(&name).unwrap();
    assert!(!lzc::dataset_exists(&name));
}

#[test]
fn zvol_create_destroy() {
    let namespace = LzcTestNamespace::new();
    let name = dbg!(namespace.name());
    let mut props = nvpair::NvList::new();
    props += ("volsize", 1024_u64 * 1024 * 1024);
    props += ("volblocksize", 4096_u64);
    lzc::create_volume(&name, props).unwrap();
    assert!(lzc::dataset_exists(&name));
    lzc::destroy_dataset(&name).unwrap();
    assert!(!lzc::dataset_exists(&name));
}
