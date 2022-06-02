// Make sure to run
// echo 3 | sudo tee /sys/module/zfs/parameters/zvol_volmode
// before running this test.

use std::process::Command;

use nanoid::nanoid;

use razor_libzfscore::error::CoreError;
use razor_libzfscore::zfs_type_t;
use razor_property as property;
use razor_zfs::*;

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
    dbg!("requesting to create filesystem");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    dbg!("filesystem created");
    assert!(
        Zfs::dataset_exists(filesystem.name()).is_ok(),
        "couldnt find filesystem"
    );
    dbg!("create_basic_filesystem finished");
}

#[test]
fn set_properties_filesystem() {
    dbg!("starting set_properties_filesystem");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "set_filesystem");
    dbg!("requesting to create filesystem");
    let filesystem = Zfs::filesystem()
        .canmount(property::OnOffNoAuto::Off)
        .checksum(property::CheckSum::Off)
        .readonly(property::OnOff::Off)
        .compression(property::Compression::Off)
        .vscan(property::OnOff::Off)
        .atime(property::OnOff::Off)
        .devices(property::OnOff::Off)
        .exec(property::OnOff::Off)
        .nbmand(property::OnOff::Off)
        .overlay(property::OnOff::Off)
        .relatime(property::OnOff::Off)
        .setuid(property::OnOff::Off)
        .zoned(property::OnOff::Off)
        .create(&name)
        .unwrap();
    dbg!("filesystem created");
    assert_eq!(property::OnOffNoAuto::Off, filesystem.canmount());
    assert_eq!(property::CheckSum::Off, filesystem.checksum());
    assert_eq!(property::OnOff::Off, filesystem.readonly());
    assert_eq!(property::Compression::Off, filesystem.compression());
    assert_eq!(property::OnOff::Off, filesystem.vscan());
    assert_eq!(property::OnOff::Off, filesystem.atime());
    assert_eq!(property::OnOff::Off, filesystem.devices());
    assert_eq!(property::OnOff::Off, filesystem.exec());
    assert_eq!(property::OnOff::Off, filesystem.nbmand());
    assert_eq!(property::OnOff::Off, filesystem.overlay());
    assert_eq!(property::OnOff::Off, filesystem.relatime());
    assert_eq!(property::OnOff::Off, filesystem.setuid());
    assert_eq!(property::OnOff::Off, filesystem.zoned());
    dbg!("passed creation test");
    filesystem
        .set()
        .canmount(property::OnOffNoAuto::On)
        .checksum(property::CheckSum::On)
        .readonly(property::OnOff::On)
        .compression(property::Compression::On)
        .vscan(property::OnOff::On)
        .atime(property::OnOff::On)
        .devices(property::OnOff::On)
        .exec(property::OnOff::On)
        .nbmand(property::OnOff::On)
        .overlay(property::OnOff::On)
        .relatime(property::OnOff::On)
        .setuid(property::OnOff::On)
        .zoned(property::OnOff::On)
        .add()
        .unwrap();
    assert_eq!(property::OnOffNoAuto::On, filesystem.canmount());
    assert_eq!(property::CheckSum::On, filesystem.checksum());
    assert_eq!(property::OnOff::On, filesystem.readonly());
    assert_eq!(property::Compression::On, filesystem.compression());
    assert_eq!(property::OnOff::On, filesystem.vscan());
    assert_eq!(property::OnOff::On, filesystem.atime());
    assert_eq!(property::OnOff::On, filesystem.devices());
    assert_eq!(property::OnOff::On, filesystem.exec());
    assert_eq!(property::OnOff::On, filesystem.nbmand());
    assert_eq!(property::OnOff::On, filesystem.overlay());
    assert_eq!(property::OnOff::On, filesystem.relatime());
    assert_eq!(property::OnOff::On, filesystem.setuid());
    assert_eq!(property::OnOff::On, filesystem.zoned());
}

#[test]
fn set_properties_volume() {
    dbg!("starting set_properties_volume");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "set_volume");
    dbg!("requesting to create volume");
    let filesystem = Zfs::volume()
        .checksum(property::CheckSum::Off)
        .compression(property::Compression::Off)
        .volmode(property::VolMode::None)
        .create(&name, 128 * 1024)
        .unwrap();
    dbg!("filesystem created");
    assert_eq!(property::CheckSum::Off, filesystem.checksum());
    assert_eq!(property::Compression::Off, filesystem.compression());
    dbg!("passed creation test");
    filesystem
        .set()
        .checksum(property::CheckSum::On)
        .compression(property::Compression::On)
        .add()
        .unwrap();
    assert_eq!(property::CheckSum::On, filesystem.checksum());
    assert_eq!(property::Compression::On, filesystem.compression());
}

#[test]
fn create_dup_filesystem() {
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "dup_filesystem");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    assert!(
        Zfs::dataset_exists(filesystem.name()).is_ok(),
        "couldnt find filesystem"
    );
    let res = Zfs::filesystem().create(&name).unwrap_err();
    let expected = DatasetError::CoreErr(CoreError::LibcError(
        libc::EEXIST,
        "file exists".to_string(),
    ));
    assert_eq!(expected, res);
}

#[test]
fn create_basic_volume() {
    dbg!("starting create basic volume");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "volume");
    dbg!("requesting to create volume");
    let volume = Zfs::volume()
        .volmode(property::VolMode::None)
        .create(name, 128 * 1024)
        .unwrap();
    dbg!("volume created");
    assert!(
        Zfs::dataset_exists(volume.name()).is_ok(),
        "couldnt find volume"
    );
    dbg!("create_basic_volume finished");
}

#[test]
fn create_dup_volume() {
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "dup_volume");
    let volume = Zfs::volume()
        .volmode(property::VolMode::None)
        .create(&name, 128 * 1024)
        .unwrap();
    assert!(
        Zfs::dataset_exists(volume.name()).is_ok(),
        "couldnt find filesystem"
    );
    let res = Zfs::volume()
        .volmode(property::VolMode::None)
        .create(&name, 128 * 1024)
        .unwrap_err();
    let expected = DatasetError::CoreErr(CoreError::LibcError(
        libc::EEXIST,
        "file exists".to_string(),
    ));
    assert_eq!(expected, res);
}

#[test]
fn get_volume() {
    dbg!("starting get volume");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "get_vol");
    dbg!("inside get_volume starting to create volume");
    let volume = Zfs::volume()
        .volmode(property::VolMode::None)
        .create(name, 128 * 1024)
        .unwrap();
    dbg!("inside get_volume finished to create volume");
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
fn filesystem_snapshot() {
    dbg!("starting filesystem_snapshot");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "fs_snapshot");
    let filesystem = Zfs::filesystem()
        .canmount(property::OnOffNoAuto::On)
        .create(&name)
        .unwrap();
    let name = format!("{}/{}", filesystem.name(), "another_fs_snapshot");
    let _another_filesystem = Zfs::filesystem()
        .canmount(property::OnOffNoAuto::On)
        .create(&name)
        .unwrap();
    filesystem.snapshot("snap1").unwrap();
    filesystem.snapshot("snap2").unwrap();
    filesystem.snapshot("snap3").unwrap();
    filesystem.snapshot("snap4").unwrap();

    let snapshots = Zfs::list_from(filesystem.name())
        .snapshots()
        .recursive(true)
        .get_collection()
        .unwrap();

    for snapshot in snapshots {
        dbg!(snapshot.name());
        assert_eq!(zfs_type_t::ZFS_TYPE_SNAPSHOT, snapshot.r#type());
    }
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
        .recursive(true)
        .get_collection()
        .unwrap();

    for dataset in datasets {
        dbg!(dataset.name());
        assert_eq!(zfs_type_t::ZFS_TYPE_FILESYSTEM, dataset.r#type());
    }
}

// #[test]
// fn list_filesystems_from() {
//     dbg!("starting list filesystems from");
//     let test = TestNamespace::new();
//     let mut names = Vec::new();

//     for i in 1..rand::thread_rng().gen_range(5..10) {
//         names.push(format!(
//             "{}/{}{}",
//             test.namespace.name(),
//             "from_filesystem",
//             i.to_string()
//         ));

//         Zfs::filesystem().create(names.last().unwrap()).unwrap();
//     }

//     let mut accamulator = Vec::new();

//     for name in names.iter() {
//         let mut children_names = Vec::new();
//         let rnd = rand::thread_rng().gen_range(1..10);

//         for _i in 0..rnd {
//             children_names.push(format!("{}/{}", name, nanoid!()));
//             Zfs::filesystem()
//                 .create(children_names.last().unwrap())
//                 .unwrap();
//         }

//         accamulator.append(&mut children_names);
//     }

//     names.append(&mut accamulator);

//     let datasets = Zfs::list_from(test.namespace.name())
//         .filesystems()
//         .recursive()
//         .get_collection()
//         .unwrap();

//     dbg!("names i created: ", &names);

//     dbg!("wanted lenght: ", names.len());
//     dbg!("got lenght: ", datasets.len());
//     assert_eq!(names.len(), datasets.len());

//     for dataset in datasets.into_iter() {
//         dbg!(dataset.name());
//         assert!(
//             names.contains(&dataset.name().to_string()),
//             "received dataset dont exist in names vector"
//         );
//         assert_eq!(zfs_type_t::ZFS_TYPE_FILESYSTEM, dataset.r#type());
//     }

//     dbg!("finished asserting: all good");
// }

macro_rules! list_filesystems_from_dup {
    ($($name:ident: $num_of_parents:expr, $vec_of_childrens:expr,)*) => {
    $(
        #[test]
        fn $name() {
            dbg!("starting list filesystems from");
    let test = TestNamespace::new();
    let num_of_parents: u64 = $num_of_parents;
    let vec_of_childrens: Vec<u64> = $vec_of_childrens;
    let mut names = Vec::new();

    for i in 1..num_of_parents {
        names.push(format!(
            "{}/{}{}",
            test.namespace.name(),
            "from_filesystem",
            i.to_string()
        ));

        Zfs::filesystem().create(names.last().unwrap()).unwrap();
    }

    let mut accamulator = Vec::new();
    let mut childs_iter = vec_of_childrens.into_iter();

    for name in names.iter() {
        let mut children_names = Vec::new();
        let num_of_childrens = childs_iter.next();
        //let rnd = rand::thread_rng().gen_range(1..10);
        if let Some(childrens) = num_of_childrens {
            for _i in 0..childrens {
            children_names.push(format!("{}/{}", name, nanoid!()));
            Zfs::filesystem()
                .create(children_names.last().unwrap())
                .unwrap();
        }

        accamulator.append(&mut children_names);
        }
    }

    names.append(&mut accamulator);

    let datasets = Zfs::list_from(test.namespace.name())
        .filesystems()
        .recursive(true)
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
    )*
    }
}

list_filesystems_from_dup! {
    fs_from_test_1: 5, vec![2,4,6,8,10],
    fs_from_test_2: 6, vec![1,3,5,7,9,11],
    fs_from_test_3: 7, vec![1,2,3,4,5,6,7],
    fs_from_test_4: 8, vec![6,7,8,9,10,11,12,13],
    fs_from_test_5: 2, vec![1,4],
    fs_from_test_6: 1, vec![1],
}

#[test]
fn list_volumes() {
    dbg!("starting list_volumes test");
    let datasets = Zfs::list()
        .volumes()
        .recursive(true)
        .get_collection()
        .unwrap();

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
        .recursive(true)
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
    dbg!("requesting to create volume in create_delete_volume");
    let volume = Zfs::volume()
        .volmode(property::VolMode::None)
        .create(name, 128 * 1024)
        .unwrap();
    dbg!("volume created in create_delete_volume");
    dbg!("requesting to delete volume in create_delete_volume");
    Zfs::destroy_dataset(volume.name()).unwrap();
    dbg!("volume deleted in create_delete_volume");
    let res = Zfs::dataset_exists(volume.name()).unwrap_err();
    let expected = DatasetError::CoreErr(CoreError::DatasetNotExist);
    assert_eq!(expected, res);
    dbg!("create_delete_volume finished");
}

#[test]
fn create_delete_filesystem() {
    dbg!("starting delete filesystem");
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "filesystem_to_delete");
    dbg!("requesting to create filesystem in create_delete_filesystem");
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    dbg!("filesystem created in create_delete_filesystem");
    dbg!("requesting to delete filesystem in create_delete_filesystem");
    Zfs::destroy_dataset(filesystem.name()).unwrap();
    dbg!("filesystem deleted in create_delete_filesystem");
    let res = Zfs::dataset_exists(filesystem.name()).unwrap_err();
    let expected = DatasetError::CoreErr(CoreError::DatasetNotExist);
    assert_eq!(expected, res);
    dbg!("create_delete_filesystem finished");
}

#[test]
fn delete_invalid_filesystem() {
    let test = TestNamespace::new();
    let name = format!(
        "{}/{}",
        test.namespace.name(),
        "invalid_filesystem_to_delete"
    );
    let res = Zfs::destroy_dataset(name).unwrap_err();
    let expected = DatasetError::CoreErr(CoreError::LibcError(
        libc::ENOENT,
        "no such file or directory".to_string(),
    ));
    assert_eq!(expected, res);
}

#[test]
fn delete_invalid_volume() {
    let test = TestNamespace::new();
    let name = format!("{}/{}", test.namespace.name(), "invalid_volume_to_delete");
    let res = Zfs::destroy_dataset(name).unwrap_err();
    let expected = DatasetError::CoreErr(CoreError::LibcError(
        libc::ENOENT,
        "no such file or directory".to_string(),
    ));
    assert_eq!(expected, res);
}
