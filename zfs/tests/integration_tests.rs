use std::thread::sleep;
use std::time::Duration;

use razor_zfs::zfs::*;

#[test]
fn create_basic_filesystem() {
    dbg!("starting create filesystem test");
    let filesystem = Zfs::filesystem("dpool/filesystem").create().unwrap();

    dbg!(&filesystem);
    assert_eq!(filesystem.name(), "dpool/filesystem");
    assert_eq!(filesystem.mounted(), property::YesNo::No);

    filesystem.destroy().unwrap();
}

#[test]
fn create_volume_dataset() {
    let volume = Zfs::volume("dpool/volume").create(128 * 1024).unwrap();
    dbg!(&volume);

    sleep(Duration::from_millis(3000));
    volume.destroy().unwrap();
}

#[test]
fn get_volume() {
    let volume = Zfs::get_volume("dpool/vol").unwrap();
    dbg!(&volume);
}

#[test]
fn get_filesystem() {
    let volume = Zfs::get_filesystem("dpool/test").unwrap();
    dbg!(&volume);
}

#[test]
fn create_delete_volume() {
    let volume = Zfs::volume("dpool/vol_to_delete")
        .create(128 * 1024)
        .unwrap();
    dbg!(&volume);
    sleep(Duration::from_millis(3000));
    Zfs::destroy_dataset("dpool/vol_to_delete").unwrap();
}

#[test]
fn create_delete_filesystem() {
    let filesystem = Zfs::filesystem("dpool/fs_to_delete").create().unwrap();

    dbg!(&filesystem);
    Zfs::destroy_dataset("dpool/fs_to_delete").unwrap();
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
