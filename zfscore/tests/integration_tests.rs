use zfscore::dataset::Dataset;
use zfscore::zfs_property;

#[test]
fn create_filesystem_dataset() {
    let filesystem = Dataset::new("filesystem")
        .unwrap()
        .atime(zfs_property::OnOff::On)
        .unwrap()
        .canmount(true)
        .unwrap()
        .create_filesystem()
        .unwrap();
}

#[test]
fn create_volume_dataset() {
    let volume = Dataset::new("volume")
        .unwrap()
        .atime(true)
        .unwrap()
        .canmount(false)
        .unwrap()
        .create_volume(256)
        .unwrap();
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
}
