use std::thread::sleep;
use std::time::Duration;

use zfs::zfs::*;

#[test]
fn create_basic_filesystem() {
    dbg!("starting create filesystem test");
    let filesystem = Zfs::filesystem("dpool/filesystem").create().unwrap();

    dbg!(&filesystem);

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
