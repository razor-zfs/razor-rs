use razor_nvpair as nvpair;
use razor_safe_lzc as lzc;
use razor_test::TestNamespace;

#[test]
fn filesystem_create_destroy() {
    let namespace = TestNamespace::unique();
    let name = dbg!(namespace.unique_name());
    let mut props = nvpair::NvList::new();
    props += ("razor-test:clean", "yes");
    lzc::create_filesystem(&name, props).unwrap();
    assert!(lzc::dataset_exists(&name));
    lzc::destroy_dataset(&name).unwrap();
    assert!(!lzc::dataset_exists(&name));
}

#[test]
fn zvol_create_destroy() {
    let namespace = TestNamespace::unique();
    let name = dbg!(namespace.unique_name());
    let mut props = nvpair::NvList::new();
    props += ("razor-test:clean", "yes");
    props += ("volsize", 1024_u64 * 1024 * 1024);
    props += ("volblocksize", 4096_u64);
    lzc::create_volume(&name, props).unwrap();
    assert!(lzc::dataset_exists(&name));
    lzc::destroy_dataset(&name).unwrap();
    assert!(!lzc::dataset_exists(&name));
}

#[test]
fn filesystem_create_existing() {
    let namespace = TestNamespace::unique();
    let name = dbg!(namespace.unique_name());
    let mut props = nvpair::NvList::new();
    props += ("razor-test:clean", "yes");
    lzc::create_filesystem(&name, &props).unwrap();
    assert!(lzc::dataset_exists(&name));
    let e = lzc::create_filesystem(&name, props).unwrap_err();
    assert_eq!(e.code, libc::EEXIST);
    lzc::destroy_dataset(&name).unwrap();
    assert!(!lzc::dataset_exists(&name));
}

#[test]
fn zvol_create_existing() {
    let namespace = TestNamespace::unique();
    let name = dbg!(namespace.unique_name());
    let mut props = nvpair::NvList::new();
    props += ("razor-test:clean", "yes");
    props += ("volsize", 1024_u64 * 1024 * 1024);
    props += ("volblocksize", 4096_u64);
    lzc::create_volume(&name, &props).unwrap();
    assert!(lzc::dataset_exists(&name));
    let e = lzc::create_volume(&name, props).unwrap_err();
    assert_eq!(e.code, libc::EEXIST);
    lzc::destroy_dataset(&name).unwrap();
    assert!(!lzc::dataset_exists(&name));
}
