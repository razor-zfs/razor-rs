// Make sure to run
// echo 3 | sudo tee /sys/module/zfs/parameters/zvol_volmode
// before running this test.

use razor_test::TestNamespace;
use razor_zfs as zfs;

use zfs::zfs::property;
// use zfs::Filesystem;
use zfs::Zfs;

#[test]
fn create_basic_filesystem() -> anyhow::Result<()> {
    dbg!("starting create basic filesystem");
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let filesystem = Zfs::filesystem().create(&name)?;
    assert_eq!(filesystem.name(), name);
    // namespace.sync_delay();
    assert!(Zfs::dataset_exists(name));
    Ok(())
}

#[test]
fn set_properties_filesystem() -> anyhow::Result<()> {
    dbg!("starting set_properties_filesystem");
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let mut filesystem = Zfs::filesystem()
        .canmount(property::CanMount::Off)
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
        .create(&name)?;
    assert_eq!(property::CanMount::Off, filesystem.canmount());
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
    filesystem
        .set()
        .canmount(property::CanMount::On)
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
        .commit()?;
    assert_eq!(property::CanMount::On, filesystem.canmount());
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
    Ok(())
}

#[test]
fn set_properties_volume() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let mut volume = Zfs::volume()
        .checksum(property::CheckSum::Off)
        .compression(property::Compression::Off)
        .volmode(property::VolMode::None)
        .create(&name, 128 * 1024)
        .unwrap();
    assert_eq!(property::CheckSum::Off, volume.checksum());
    assert_eq!(property::Compression::Off, volume.compression());
    volume
        .set()
        .checksum(property::CheckSum::On)
        .compression(property::Compression::On)
        .commit()
        .unwrap();
    assert_eq!(property::CheckSum::On, volume.checksum());
    assert_eq!(property::Compression::On, volume.compression());
}

#[test]
fn create_dup_filesystem() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    assert!(
        Zfs::dataset_exists(filesystem.name()),
        "couldnt find filesystem"
    );
    let _res = Zfs::filesystem().create(&name).unwrap_err();
    // let expected = DatasetError::CoreErr(CoreError::LibcError(
    //     libc::EEXIST,
    //     "file exists".to_string(),
    // ));
    // assert_eq!(expected, res);
}

#[test]
fn create_basic_volume() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let volume = Zfs::volume()
        .volmode(property::VolMode::None)
        .create(name, 128 * 1024)
        .unwrap();
    namespace.sync_delay();
    assert!(Zfs::dataset_exists(volume.name()), "couldnt find volume");
}

#[test]
fn create_dup_volume() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let volume = Zfs::volume()
        .volmode(property::VolMode::None)
        .create(&name, 128 * 1024)
        .unwrap();
    assert!(
        Zfs::dataset_exists(volume.name()),
        "couldnt find filesystem"
    );
    let _res = Zfs::volume()
        .volmode(property::VolMode::None)
        .create(&name, 128 * 1024)
        .unwrap_err();
    // let expected = DatasetError::CoreErr(CoreError::LibcError(
    //     libc::EEXIST,
    //     "file exists".to_string(),
    // ));
    // assert_eq!(expected, res);
}

#[test]
fn get_volume() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let volume = Zfs::volume()
        .volmode(property::VolMode::None)
        .create(name, 128 * 1024)
        .unwrap();
    let res_vol = Zfs::get_volume(volume.name());
    assert!(res_vol.is_ok(), "couldnt get volume");
}

#[test]
fn get_filesystem() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let filesystem = Zfs::filesystem().create(&name).unwrap();
    let res_filesystem = Zfs::get_filesystem(filesystem.name());
    assert!(res_filesystem.is_ok(), "couldnt get filesystem");
}

#[test]
fn filesystem_snapshot() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let filesystem = Zfs::filesystem()
        .canmount(property::CanMount::On)
        .create(&name)
        .unwrap();
    let name = format!("{}/{}", filesystem.name(), "another_fs_snapshot");
    let _another_filesystem = Zfs::filesystem()
        .canmount(property::CanMount::On)
        .create(&name)
        .unwrap();
    filesystem.snapshot("snap1").unwrap();
    filesystem.snapshot("snap2").unwrap();
    filesystem.snapshot("snap3").unwrap();
    filesystem.snapshot("snap4").unwrap();

    let snapshots = Zfs::list_from(filesystem.name())
        .snapshots()
        .recursive(true)
        .get_collection();

    for snapshot in snapshots {
        dbg!(snapshot.name());
        assert!(snapshot.r#type().is_snapshot());
    }
}

#[test]
fn get_non_existent_volume() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let _res_vol = Zfs::get_volume(name).unwrap_err();
    // let expected = DatasetError::CoreErr(CoreError::DatasetNotExist);
    // assert_eq!(expected, res_vol);
}

#[test]
fn get_non_existent_filesystem() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let _res_filesystem = Zfs::get_filesystem(name).unwrap_err();
    // let expected = DatasetError::CoreErr(CoreError::DatasetNotExist);
    // assert_eq!(expected, res_filesystem);
}

#[test]
fn list_filesystems() {
    let datasets = Zfs::list().filesystems().recursive(true).get_collection();

    for dataset in datasets {
        dbg!(dataset.name());
        assert!(dataset.r#type().is_filesystem());
    }
}

// macro_rules! list_filesystems_from_dup {
//     ($($name:ident: $num_of_parents:expr, $vec_of_childrens:expr,)*) => {
//     $(
//         #[test]
//         fn $name() {
//             dbg!("starting list filesystems from");
//     let namespace = TestNamespace::unique();
//     let num_of_parents: u64 = $num_of_parents;
//     let vec_of_childrens: Vec<u64> = $vec_of_childrens;
//     let mut names = Vec::new();

//     let base = namespace.unique_name();
//     for i in 1..num_of_parents {
//         let name = format!("{base}{i}");
//         Zfs::filesystem().create(&name).unwrap();
//         names.push(name);
//     }

//     let mut accamulator = Vec::new();
//     let mut childs_iter = vec_of_childrens.into_iter();

//     for name in names.iter() {
//         let mut children_names = Vec::new();
//         let num_of_childrens = childs_iter.next();
//         //let rnd = rand::thread_rng().gen_range(1..10);
//         if let Some(childrens) = num_of_childrens {
//             for _i in 0..childrens {
//             children_names.push(format!("{}/{}", name, nanoid!()));
//             Zfs::filesystem()
//                 .create(children_names.last().unwrap())
//                 .unwrap();
//         }

//         accamulator.append(&mut children_names);
//         }
//     }

//     names.append(&mut accamulator);

//     let datasets = Zfs::list_from(test.namespace.name())
//         .filesystems()
//         .recursive(true)
//         .get_collection();

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
//         assert!(dataset.r#type().is_filesystem());
//     }

//     dbg!("finished asserting: all good");
//         }
//     )*
//     }
// }

// list_filesystems_from_dup! {
//     fs_from_test_1: 5, vec![2,4,6,8,10],
//     fs_from_test_2: 6, vec![1,3,5,7,9,11],
//     fs_from_test_3: 7, vec![1,2,3,4,5,6,7],
//     fs_from_test_4: 8, vec![6,7,8,9,10,11,12,13],
//     fs_from_test_5: 2, vec![1,4],
//     fs_from_test_6: 1, vec![1],
// }

#[test]
fn list_volumes() {
    let datasets = Zfs::list().volumes().recursive(true).get_collection();

    for dataset in datasets {
        dbg!(dataset.name());
        assert!(dataset.r#type().is_volume());
    }
}

#[test]
fn list_all() {
    let datasets = Zfs::list()
        .filesystems()
        .volumes()
        .recursive(true)
        .get_collection();

    for dataset in datasets {
        dbg!(dataset.name());
    }
}

#[test]
fn list_all_non_recursive() {
    let datasets = Zfs::list().filesystems().volumes().get_collection();

    for dataset in datasets {
        dbg!(dataset.name());
    }
}

#[test]
fn create_delete_volume() -> anyhow::Result<()> {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let volume = Zfs::volume()
        .volmode(property::VolMode::None)
        .create(&name, 128 * 1024)?;
    volume.destroy()?;
    namespace.sync_delay();
    assert!(!Zfs::dataset_exists(name));
    Ok(())
}

#[test]
fn create_delete_filesystem() -> anyhow::Result<()> {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let filesystem = Zfs::filesystem().create(&name)?;
    filesystem.destroy()?;
    namespace.sync_delay();
    assert!(!Zfs::dataset_exists(name));
    Ok(())
}

#[test]
fn delete_non_existent_filesystem() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let _res = Zfs::destroy_dataset(name).unwrap_err();
    // let expected = DatasetError::CoreErr(CoreError::LibcError(
    //     libc::ENOENT,
    //     "no such file or directory".to_string(),
    // ));
    // assert_eq!(expected, res);
}

#[test]
fn delete_non_existent_volume() {
    let namespace = TestNamespace::unique();
    let name = namespace.unique_name();
    let _res = Zfs::destroy_dataset(name).unwrap_err();
    // let expected = DatasetError::CoreErr(CoreError::LibcError(
    //     libc::ENOENT,
    //     "no such file or directory".to_string(),
    // ));
    // assert_eq!(expected, res);
}
