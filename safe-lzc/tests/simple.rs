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

#[test]
fn destroy_non_existing() {
    let namespace = TestNamespace::unique();
    let name = dbg!(namespace.unique_name());
    let e = lzc::destroy_dataset(&name).unwrap_err();
    assert_eq!(e.code, libc::ENOENT);
}

#[test]
fn snapshot_without_properties() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let mut props = nvpair::NvList::new();
    props += ("razor-test:clean", "yes");
    lzc::create_filesystem(&name, props).unwrap();
    assert!(lzc::dataset_exists(&name));
    let snap = format!("{name}@snapshot_without_properties");
    lzc::create_snapshot(&snap, None).unwrap();
    assert!(lzc::dataset_exists(&snap));
    lzc::destroy_dataset(&snap).unwrap();
    assert!(!lzc::dataset_exists(&snap));
    lzc::destroy_dataset(&name).unwrap();
    assert!(!lzc::dataset_exists(&name));
}

#[test]
fn snapshot_with_properties() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let mut props = nvpair::NvList::new();
    props += ("razor-test:clean", "yes");
    lzc::create_filesystem(&name, props).expect("create filesystem");
    assert!(lzc::dataset_exists(&name));
    let snap = format!("{name}@snapshot_with_properties");
    let mut props = nvpair::NvList::new();
    props += ("razor-test:key", "value");
    lzc::create_snapshot(&snap, props).expect("create snapshot");
    assert!(lzc::dataset_exists(&snap));
    lzc::destroy_dataset(&snap).expect("destroy snapshot");
    assert!(!lzc::dataset_exists(&snap));
    lzc::destroy_dataset(&name).expect("destroy filesystem");
    assert!(!lzc::dataset_exists(&name));
}
