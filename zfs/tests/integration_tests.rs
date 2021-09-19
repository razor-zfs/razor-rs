// Make sure to run
// echo 3 | sudo tee /sys/module/zfs/parameters/zvol_volmode
// before running this test.

use rand::Rng;

use razor_zfs::zfs::*;

use std::sync::Once;

static INIT: Once = Once::new();

pub fn initialize() -> String {
    let mut namespace = String::new();

    INIT.call_once(|| {
        let mut rng = rand::thread_rng();
        let identifier: u8 = rng.gen();
        namespace.push_str("dpool/test");
        namespace.push_str(identifier.to_string().as_str());
        dbg!("trying to create dataset: ", &namespace);
        Zfs::filesystem().create(&namespace).unwrap();
        dbg!("created test namespace");
    });

    namespace
}

#[test]
fn create_basic_filesystem() {
    let namespace = initialize();
    dbg!("starting create filesystem test");
    let filesystem_name = format!("{}/{}", namespace, "filesystem");
    dbg!("name: ", &filesystem_name);
    let filesystem = Zfs::filesystem()
        .atime(property::OnOff::Off)
        .canmount(property::OnOffNoAuto::Off)
        .create(&filesystem_name)
        .unwrap();
    dbg!(serde_json::to_string(&filesystem).unwrap());
    filesystem
        .set()
        .overlay(property::OnOff::On)
        .readonly(property::OnOff::On)
        .add()
        .unwrap();
    dbg!(serde_json::to_string(&filesystem).unwrap());
    assert_eq!(filesystem.name(), filesystem_name);
    assert_eq!(filesystem.mounted(), property::YesNo::No);

    filesystem.destroy().unwrap();
}

#[test]
fn create_volume_dataset() {
    let volume = Zfs::volume()
        .volmode(property::VolModeId::None)
        .create("dpool/volume", 128 * 1024)
        .unwrap();
    dbg!(&volume);

    volume.destroy().unwrap();
}

#[test]
fn list() {
    let datasets = Zfs::list().get_collection();

    for dataset in datasets {
        dbg!(dataset.name());
    }
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
    let volume = Zfs::volume()
        .create("dpool/vol_to_delete", 128 * 1024)
        .unwrap();
    dbg!(&volume);
    Zfs::destroy_dataset("dpool/vol_to_delete").unwrap();
}

#[test]
fn create_delete_filesystem() {
    let filesystem = Zfs::filesystem().create("dpool/fs_to_delete").unwrap();

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
