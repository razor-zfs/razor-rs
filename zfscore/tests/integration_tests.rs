use zfscore::ZFS;

#[test]
fn create_basic_filesystem() {
    dbg!("starting create filesystem test");
    let filesystem = ZFS
        .lock()
        .unwrap()
        .new_filesystem("dpool/filesystem")
        .create()
        .unwrap();

    dbg!(&filesystem);

    filesystem.destroy().unwrap();
}

#[test]
fn create_volume_dataset() {
    let volume = ZFS
        .lock()
        .unwrap()
        .new_volume("dpool/volume")
        .create(128 * 1024)
        .unwrap();
    dbg!(&volume);

    volume.destroy().unwrap();
}
/*
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
} */
