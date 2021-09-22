// Make sure to run
// echo 3 | sudo tee /sys/module/zfs/parameters/zvol_volmode
// before running this test.

use std::process::Command;

use nanoid::nanoid;
use once_cell::sync::Lazy;

use razor_zfs::zfs::*;

static TEST: Lazy<TestNamespace> = Lazy::new(|| {
    let test_namespace = TestNamespace::new();
    test_namespace
});

struct TestNamespace {
    namespace: String,
}

impl TestNamespace {
    fn new() -> Self {
        Command::new("echo")
            .args([
                "3",
                "|",
                "sudo",
                "tee",
                "/sys/module/zfs/parameters/zvol_volmode",
            ])
            .output()
            .expect("failed to execute process");
        let namespace = String::from(format!("dpool/{}", nanoid!()));
        Zfs::filesystem().create(&namespace).unwrap();
        Self { namespace }
    }
}

impl Drop for TestNamespace {
    fn drop(&mut self) {
        Zfs::destroy_dataset(&self.namespace).unwrap();
    }
}

#[test]
fn create_basic_filesystem() {
    let name = format!("{}/{}", TEST.namespace, "filesystem");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    let res = Zfs::dataset_exists(filesystem.name());
    assert_eq!((), res.unwrap());
    // dbg!(serde_json::to_string(&filesystem).unwrap());
    // filesystem
    //     .set()
    //     .overlay(property::OnOff::On)
    //     .readonly(property::OnOff::On)
    //     .add()
    //     .unwrap();
    // dbg!(serde_json::to_string(&filesystem).unwrap());
    // assert_eq!(filesystem.name(), filesystem_name);
    // assert_eq!(filesystem.mounted(), property::YesNo::No);

    filesystem.destroy().unwrap();
}

#[test]
fn create_basic_volume() {
    let name = format!("{}/{}", TEST.namespace, "volume");
    let volume = Zfs::volume()
        .volmode(property::VolModeId::None)
        .create(name, 128 * 1024)
        .unwrap();
    let res = Zfs::dataset_exists(volume.name());
    assert_eq!((), res.unwrap());

    volume.destroy().unwrap();
}

#[test]
fn get_volume() {
    let name = format!("{}/{}", TEST.namespace, "get_vol");
    let volume = Zfs::volume()
        .volmode(property::VolModeId::None)
        .create(name, 128 * 1024)
        .unwrap();
    let res_vol = Zfs::get_volume(volume.name());
    assert_eq!(true, res_vol.is_ok());
}

#[test]
fn get_filesystem() {
    let name = format!("{}/{}", TEST.namespace, "get_fs");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    let res_filesystem = Zfs::get_filesystem(filesystem.name());
    assert_eq!(true, res_filesystem.is_ok());
}

#[test]
fn list_filesystems() {
    dbg!("starting list_filesystem test");
    let datasets = Zfs::list()
        .filesystems()
        .recursive()
        .get_collection()
        .unwrap();

    for dataset in datasets {
        dbg!(dataset.name());
    }
}

#[test]
fn list_filesystems_from() {
    dbg!("starting list_filesystem_from test");
    let datasets = Zfs::list_from("dpool/export")
        .filesystems()
        .recursive()
        .get_collection()
        .unwrap();

    for dataset in datasets {
        dbg!(dataset.name());
    }
}

#[test]
fn list_volumes() {
    dbg!("starting list_volumes test");
    let datasets = Zfs::list().volumes().recursive().get_collection().unwrap();

    for dataset in datasets {
        dbg!(dataset.name());
    }
}

#[test]
fn list_all() {
    dbg!("starting list_all test");
    let datasets = Zfs::list()
        .filesystems()
        .volumes()
        .recursive()
        .get_collection()
        .unwrap();

    for dataset in datasets {
        dbg!(dataset.name());
    }
}

#[test]
fn list_all_non_recursive() {
    dbg!("starting list_all_non_recursive test");
    let datasets = Zfs::list()
        .filesystems()
        .volumes()
        .get_collection()
        .unwrap();

    for dataset in datasets {
        dbg!(dataset.name());
    }
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
