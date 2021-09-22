// Make sure to run
// echo 3 | sudo tee /sys/module/zfs/parameters/zvol_volmode
// before running this test.

use std::process::Command;

use nanoid::nanoid;
use rand::prelude::*;

use razor_zfs::zfs::*;
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
        let namespace = String::from(format!("dpool/{}", nanoid!()));
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
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "filesystem");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    let res = Zfs::dataset_exists(filesystem.name());
    assert_eq!((), res.unwrap());
}

#[test]
fn create_basic_volume() {
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "volume");
    let volume = Zfs::volume()
        .volmode(property::VolModeId::None)
        .create(name, 128 * 1024)
        .unwrap();
    let res = Zfs::dataset_exists(volume.name());
    assert_eq!((), res.unwrap());
}

#[test]
fn get_volume() {
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "get_vol");
    let volume = Zfs::volume()
        .volmode(property::VolModeId::None)
        .create(name, 128 * 1024)
        .unwrap();
    let res_vol = Zfs::get_volume(volume.name());
    assert_eq!(true, res_vol.is_ok());
}

#[test]
fn get_filesystem() {
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "get_fs");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    let res_filesystem = Zfs::get_filesystem(filesystem.name());
    assert_eq!(true, res_filesystem.is_ok());
}

#[test]
fn get_invalid_volume() {
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), nanoid!());
    let res_vol = Zfs::get_volume(name);
    assert_eq!(true, res_vol.is_err());
}

#[test]
fn get_invalid_filesystem() {
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "get_fs");
    let res_filesystem = Zfs::get_filesystem(name);
    assert_eq!(true, res_filesystem.is_err());
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
        assert_eq!(true, names.contains(&dataset.name().to_string()));
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
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "volume_to_delete");
    let volume = Zfs::volume()
        .volmode(property::VolModeId::None)
        .create(name, 128 * 1024)
        .unwrap();
    Zfs::destroy_dataset(volume.name()).unwrap();
    let res = Zfs::dataset_exists(volume.name());
    assert_eq!(true, res.is_err());
}

#[test]
fn create_delete_filesystem() {
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "filesystem_to_delete");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    Zfs::destroy_dataset(filesystem.name()).unwrap();
    let res = Zfs::dataset_exists(filesystem.name());

    assert_eq!(true, res.is_err());
}
