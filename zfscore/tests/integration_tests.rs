use zfscore::dataset::Dataset;
use zfscore::zfs_property;

#[test]
fn create_filesystem_dataset() {
    let filesystem = Dataset::builder()
        .unwrap()
        .atime(zfs_property::OnOff::On)
        .unwrap()
        .canmount(zfs_property::OnOffNoAuto::NoAuto)
        .unwrap()
        .copies(3)
        .unwrap()
        .create_filesystem()
        .unwrap();
}

#[test]
fn create_volume_dataset() {
    let volume = Dataset::builder()
        .unwrap()
        .atime(zfs_property::OnOff::On)
        .unwrap()
        .canmount(zfs_property::OnOffNoAuto::NoAuto)
        .unwrap()
        .copies(3)
        .unwrap()
        .create_volume(256)
        .unwrap();
}

#[test]
fn create_snapshot_dataset() {
    let snapshot = Dataset::builder()
        .unwrap()
        .atime(zfs_property::OnOff::On)
        .unwrap()
        .canmount(zfs_property::OnOffNoAuto::NoAuto)
        .unwrap()
        .copies(3)
        .unwrap()
        .create_snapshot()
        .unwrap();
}

#[test]
fn create_bookmark_dataset() {
    let bookmark = Dataset::builder()
        .unwrap()
        .atime(zfs_property::OnOff::On)
        .unwrap()
        .canmount(zfs_property::OnOffNoAuto::NoAuto)
        .unwrap()
        .copies(3)
        .unwrap()
        .create_bookmark()
        .unwrap();
}
