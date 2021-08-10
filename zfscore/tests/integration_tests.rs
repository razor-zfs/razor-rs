use std::fs::File;
use std::io::prelude::*;
use zfscore::dataset::Dataset;

#[test]
fn create_basic_filesystem() {
    dbg!("BEFORE THE TEST");
    println!("asdasdas");
    let filesystem = Dataset::new("dpool/filesystem")
        .unwrap()
        .create_filesystem()
        .unwrap();
    dbg!("AFTER THE TEST");
    dbg!(&filesystem);
    let mut file = File::create("create_basic_filesystem.txt").unwrap();
    file.write_all(
        serde_json::to_string_pretty(&filesystem)
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
    file.sync_all().unwrap();
    filesystem.destroy().unwrap();
    /*let expected = Filesystem {
        available: filesystem.available.clone(),
        atime: zfs_property::Atime::new(zfs_property::OnOff::On),
        logicalused: zfs_property::LogicalUsed::new(43008),
        canmount: zfs_property::CanMount::new(zfs_property::OnOffNoAuto::On),
        mounted: zfs_property::Mounted::new(zfs_property::YesNo::No),
        checksum: zfs_property::CheckSum::new(zfs_property::CheckSumAlgo::On),
        compression: zfs_property::Compression::new(zfs_property::CompressionAlgo::Off),
    };

    assert_eq!(expected, filesystem);*/
}

/*#[test]
fn create_volume_dataset() {
    let volume = Dataset::new("dpool/volume")
        .unwrap()
        .create_volume(128 * 1024)
        .unwrap();
    dbg!("AFTER THE TEST");
    dbg!(volume);
}

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
