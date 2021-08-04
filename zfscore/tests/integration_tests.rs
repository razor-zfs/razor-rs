use zfscore::dataset::Dataset;
use zfscore::zfs_property;

#[test]
fn create_basic_filesystem() {
    let filesystem = Dataset::new("dpool/filesystem")
        .unwrap()
        .create_filesystem()
        .unwrap();
    dbg!(filesystem);
}

#[test]
fn create_volume_dataset() {
    let volume = Dataset::new("dpool/volume")
        .unwrap()
        .create_volume(128 * 1024)
        .unwrap();
    dbg!(volume);
}

#[test]
fn create_snapshot_dataset() {
    let snapshot = Dataset::new("snapshot")
        .unwrap()
        .atime(zfs_property::OnOff::On)
        .unwrap()
        .canmount(zfs_property::OnOffNoAuto::NoAuto)
        .unwrap()
        .create_snapshot()
        .unwrap();
    dbg!(snapshot);
}

#[test]
fn create_bookmark_dataset() {
    let bookmark = Dataset::new("bookmark")
        .unwrap()
        .atime(zfs_property::OnOff::On)
        .unwrap()
        .canmount(zfs_property::OnOffNoAuto::NoAuto)
        .unwrap()
        .create_bookmark()
        .unwrap();
    dbg!(bookmark);
}
