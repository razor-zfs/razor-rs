// Make sure to run
// echo 3 | sudo tee /sys/module/zfs/parameters/zvol_volmode
// before running this test.

use std::process::Command;

use nanoid::nanoid;
use rand::prelude::*;

use razor_zfs::{error::DatasetError, zfs::*};
use razor_zfscore::error::CoreError;
use razor_zfscore_sys::zfs_type_t;

#[derive(Debug)]
struct TestNamespace {
    namespace: Filesystem,
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
        let namespace = format!("dpool/{}", nanoid!());
        let namespace = Zfs::filesystem().create(&namespace).unwrap();
        Self { namespace }
    }
}

impl Drop for TestNamespace {
    fn drop(&mut self) {
        self.namespace.destroy_recursive().unwrap()
    }
}

#[test]
fn create_basic_filesystem() {
    dbg!("starting create basic filesystem");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "filesystem");
    dbg!("requested to create filesystem");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    dbg!("filesystem created");
    assert!(
        Zfs::dataset_exists(filesystem.name()).is_ok(),
        "couldnt find filesystem"
    );
    dbg!("create_basic_filesystem finished");
}

#[test]
fn create_basic_volume() {
    dbg!("starting create basic volume");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "volume");
    let volume = Zfs::volume()
        .volmode(property::VolModeId::None)
        .create(name, 128 * 1024)
        .unwrap();
    assert!(
        Zfs::dataset_exists(volume.name()).is_ok(),
        "couldnt find volume"
    )
}

#[test]
fn get_volume() {
    dbg!("starting get volume");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "get_vol");
    let volume = Zfs::volume()
        .volmode(property::VolModeId::None)
        .create(name, 128 * 1024)
        .unwrap();
    let res_vol = Zfs::get_volume(volume.name());
    assert!(res_vol.is_ok(), "couldnt get volume");
}

#[test]
fn get_filesystem() {
    dbg!("starting get filesystem");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "get_fs");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    let res_filesystem = Zfs::get_filesystem(filesystem.name());
    assert!(res_filesystem.is_ok(), "couldnt get filesystem");
}

#[test]
fn get_invalid_volume() {
    dbg!("starting get invalid volume");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), nanoid!());
    let res_vol = Zfs::get_volume(name).unwrap_err();
    let expected = DatasetError::CoreErr(CoreError::DatasetNotExist);
    assert_eq!(expected, res_vol);
}

#[test]
fn get_invalid_filesystem() {
    dbg!("starting get invalid filesystem");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "get_fs");
    let res_filesystem = Zfs::get_filesystem(name).unwrap_err();
    let expected = DatasetError::CoreErr(CoreError::DatasetNotExist);
    assert_eq!(expected, res_filesystem);
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
        assert_eq!(zfs_type_t::ZFS_TYPE_FILESYSTEM, dataset.r#type());
    }
}

#[test]
fn list_filesystems_from() {
    dbg!("starting list filesystems from");
    let test = TestNamespace::new();
    let mut names = Vec::new();

    for i in 1..rand::thread_rng().gen_range(5..10) {
        names.push(format!(
            "{}/{}{}",
            test.namespace.name(),
            "from_filesystem",
            i.to_string()
        ));

        Zfs::filesystem().create(names.last().unwrap()).unwrap();
    }

    let mut accamulator = Vec::new();

    for name in names.iter() {
        let mut children_names = Vec::new();
        let rnd = rand::thread_rng().gen_range(1..10);

        for _i in 0..rnd {
            children_names.push(format!("{}/{}", name, nanoid!()));
            Zfs::filesystem()
                .create(children_names.last().unwrap())
                .unwrap();
        }

        accamulator.append(&mut children_names);
    }

    names.append(&mut accamulator);

    let datasets = Zfs::list_from(test.namespace.name())
        .filesystems()
        .recursive()
        .get_collection()
        .unwrap();

    dbg!("names i created: ", &names);

    dbg!("wanted lenght: ", names.len());
    dbg!("got lenght: ", datasets.len());
    assert_eq!(names.len(), datasets.len());

    for dataset in datasets.into_iter() {
        dbg!(dataset.name());
        assert!(
            names.contains(&dataset.name().to_string()),
            "received dataset dont exist in names vector"
        );
        assert_eq!(zfs_type_t::ZFS_TYPE_FILESYSTEM, dataset.r#type());
    }

    dbg!("finished asserting: all good");
}

#[test]
fn list_volumes() {
    dbg!("starting list_volumes test");
    let datasets = Zfs::list().volumes().recursive().get_collection().unwrap();

    for dataset in datasets {
        dbg!(dataset.name());
        assert_eq!(zfs_type_t::ZFS_TYPE_VOLUME, dataset.r#type());
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
    dbg!("starting delete volume");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "volume_to_delete");
    let volume = Zfs::volume()
        .volmode(property::VolModeId::None)
        .create(name, 128 * 1024)
        .unwrap();
    Zfs::destroy_dataset(volume.name()).unwrap();
    let res = Zfs::dataset_exists(volume.name());
    assert!(res.is_err(), "couldnt delete volume");
}

#[test]
fn create_delete_filesystem() {
    dbg!("starting delete filesystem");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "filesystem_to_delete");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    Zfs::destroy_dataset(filesystem.name()).unwrap();
    let res = Zfs::dataset_exists(filesystem.name());

    assert!(res.is_err(), "couldnt delete filesystem");
}
