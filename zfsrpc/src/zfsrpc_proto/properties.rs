use crate::zfsrpc_proto::tonic_zfsrpc::{
    dataset_properties, filesystem_property, volume_property, FilesystemProperty, Variant,
    VolumeProperty,
};
use prop_macro::{classcase, classcase_path_end, snakecase_fn};
use razor_property as property;

/// Macros used by client code:

/// Defining functions for creating property variant
/// example:
/// // Creating CanMount property with variant "off":
/// impl_functions(can_mount => on,off)
/// // using this created function:
/// let _canmount = dataset_properties::CanMount::off();
/// let _canmount = dataset_properties::CanMount::on();
/// Note: This macro is mainly used by impl_property! macro below
macro_rules! impl_functions {

    ($p:ident => $($fn:ident),*) => {
        $(
            snakecase_fn!(pub fn $fn() -> Self {
                use dataset_properties::$p::Value::*;
                Self {
                    value: Some($fn(Variant {})),
                }
            });
        )*
    };

}

/// Implementing 'From' trait.
/// Converting from dataset generic property to specific property (Filesystem/Volume/etc.)
/// example:
/// // Implementing 'From' trait for filesystem_property:
/// impl_property_for_type!(can_mount, filesystem_property)
/// // Using the created trait when creating filesystem properties vector:
/// let canmount = dataset_properties::CanMount::off().into();
/// //                                                 ^^^^^^
/// let properties = vec![canmount];
/// let request = CreateFilesystemRequest {
///     //...
///     properties,
/// }
/// Note: This macro is used by impl_property! macro
macro_rules! impl_property_for_type {

    ($prop:ident, $($type:ident),+) => {
        $(
            impl From<classcase_path_end!(dataset_properties::$prop)> for classcase!($type) {
                fn from(var: classcase_path_end!(dataset_properties::$prop)) -> Self {
                    Self {
                        property: Some(classcase_path_end!($type::Property::$prop)(var)),
                    }
                }
            }
        )+
    };
}

/// Macros used by Server code

/// Implementing 'From' trait.
/// Converting from protobuf generated dataset property variants to razor_zfs property variants
/// example:
/// // Implementing 'From' trait for canmount with variants: on/off/noauto
/// // when razor_zfs variants of canmount are specified at enum OnOffNoAuto {On, Off, NoAuto}
/// impl_property_for_zfs!(can_mount, OnOffNoAuto, On,Off,NoAuto)
/// // Using this created trait by invoking razor_zfs FilesystemBuilder property methods:
/// // Client side:
/// // ...
/// let canmount = dataset_properties::CanMount::off().into();
/// // ...
/// // Server side:
/// // ...
/// let canmount = request.properties.first().property;
/// let fs_builder = Zfs::filesystem("pool/fs1".into());
/// fs_builder.canmount(canmount.value.ok_or_else(|| DatasetError::InvalidArgument)?);
///                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// // ...
/// Note: This macro is used by impl_property! macro
macro_rules! impl_property_for_zfs {

    ($prop:ident, $zfs_enum:ident, $($var:ident),+) => {
        impl From<dataset_properties::$prop::Value> for property::$zfs_enum {
            fn from(v: dataset_properties::$prop::Value) -> Self {
                match v {
                    $(dataset_properties::$prop::Value::$var(_) => property::$zfs_enum::$var),+
                }
            }
        }
    };
}

/// Vice-Versa
macro_rules! impl_zfs_for_property {
    ($prop:ident, $zfs_enum:ident, $($var:ident),+) => {

        impl From<property::$zfs_enum> for classcase_path_end!(dataset_properties::$prop) {
            fn from(prop: property::$zfs_enum) -> Self {
                let value = match prop {
                    $(
                        property::$zfs_enum::$var => dataset_properties::$prop::Value::$var(Variant {})
                    ),+
                };

                Self {value: Some(value)}
            }
        }
    }
}

// Invoke of macros
macro_rules! impl_property {

    ($prop:ident for $($ds_type:tt),+ and $zfs_enum:ident => $($variant:tt),+) => {
        impl classcase_path_end!(dataset_properties::$prop) {

            impl_functions!($prop => $($variant),+);
        }

        impl_property_for_type!($prop, $($ds_type),+);

        impl_property_for_zfs!($prop, $zfs_enum, $($variant),+);

        impl_zfs_for_property!($prop, $zfs_enum, $($variant),+);
    };
}

macro_rules! impl_from_u64_for_dataset_properties {
    ($($prop:ident),+) => {
        $(
            impl From<u64> for dataset_properties::$prop {
                fn from(u: u64) -> Self {
                    Self{value: u}
                }
            }
        )+
    };
}

macro_rules! impl_from_string_for_dataset_properties {
    ($($prop:ident),+) => {
        $(
            impl From<String> for dataset_properties::$prop {
                fn from(s: String) -> Self {
                    Self{value: s}
                }
            }
        )+
    };
}

// Generating code

impl_from_string_for_dataset_properties!(Name);

impl_from_u64_for_dataset_properties!(
    Available,
    LogicalUsed,
    Guid,
    Creation,
    CreateTxg,
    CompressRatio,
    Used,
    Referenced,
    LogicalReferenced,
    ObjSetId,
    VolSize,
    BlockSize
);

impl_property!(can_mount for filesystem_property and OnOffNoAuto => On,Off,NoAuto);
impl_property!(a_time for filesystem_property and OnOff => On,Off);
impl_property!(exec for filesystem_property and OnOff => On,Off);
impl_property!(nbmand for filesystem_property and OnOff => On,Off);
impl_property!(overlay for filesystem_property and OnOff => On,Off);
impl_property!(readonly for filesystem_property and OnOff => On,Off);
impl_property!(relatime for filesystem_property and OnOff => On,Off);
impl_property!(setuid for filesystem_property and OnOff => On,Off);
impl_property!(vscan for filesystem_property and OnOff => On,Off);
impl_property!(zoned for filesystem_property and OnOff => On,Off);
impl_property!(checksum for filesystem_property,volume_property and CheckSum =>
    On,
    Off,
    Fletcher2,
    Fletcher4,
    Sha256,
    NoParity,
    Sha512,
    Skein,
    Edonr
);
impl_property!(devices for filesystem_property and OnOff => On,Off);
impl_property!(compression for filesystem_property,volume_property and Compression =>
    On,
    Off,
    Lzjb,
    Gzip,
    Gzip1,
    Gzip2,
    Gzip3,
    Gzip4,
    Gzip5,
    Gzip6,
    Gzip7,
    Gzip8,
    Gzip9,
    Zle,
    Lz4,
    Zstd,
    ZstdFast
);
impl_property!(vol_mode for volume_property and VolMode =>
    Default,
    Full,
    Geom,
    Dev,
    None,
    Unknown
);

// Only Getter property, thus doesn't exists at filesystem_property mod
impl_zfs_for_property!(mounted, YesNo, Yes, No);

impl From<property::CheckSum> for VolumeProperty {
    fn from(p: property::CheckSum) -> Self {
        match p {
            property::CheckSum::On => Self {
                property: Some(volume_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::On(Variant {})),
                    },
                )),
            },
            property::CheckSum::Off => Self {
                property: Some(volume_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Off(Variant {})),
                    },
                )),
            },
            property::CheckSum::Fletcher2 => Self {
                property: Some(volume_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Fletcher2(Variant {})),
                    },
                )),
            },
            property::CheckSum::Fletcher4 => Self {
                property: Some(volume_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Fletcher4(Variant {})),
                    },
                )),
            },
            property::CheckSum::Sha256 => Self {
                property: Some(volume_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Sha256(Variant {})),
                    },
                )),
            },
            property::CheckSum::NoParity => Self {
                property: Some(volume_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::NoParity(Variant {})),
                    },
                )),
            },
            property::CheckSum::Sha512 => Self {
                property: Some(volume_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Sha512(Variant {})),
                    },
                )),
            },
            property::CheckSum::Skein => Self {
                property: Some(volume_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Skein(Variant {})),
                    },
                )),
            },
            property::CheckSum::Edonr => Self {
                property: Some(volume_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Edonr(Variant {})),
                    },
                )),
            },
        }
    }
}

impl From<property::Compression> for VolumeProperty {
    fn from(p: property::Compression) -> Self {
        match p {
            property::Compression::On => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::On(Variant {})),
                    },
                )),
            },
            property::Compression::Off => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Off(Variant {})),
                    },
                )),
            },
            property::Compression::Lzjb => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Lzjb(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip1 => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip1(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip2 => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip2(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip3 => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip3(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip4 => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip4(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip5 => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip5(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip6 => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip6(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip7 => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip7(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip8 => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip8(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip9 => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip9(Variant {})),
                    },
                )),
            },
            property::Compression::Zle => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Zle(Variant {})),
                    },
                )),
            },
            property::Compression::Lz4 => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Lz4(Variant {})),
                    },
                )),
            },
            property::Compression::Zstd => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Zstd(Variant {})),
                    },
                )),
            },
            property::Compression::ZstdFast => Self {
                property: Some(volume_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::ZstdFast(Variant {})),
                    },
                )),
            },
        }
    }
}

impl From<property::VolMode> for VolumeProperty {
    fn from(p: property::VolMode) -> Self {
        match p {
            property::VolMode::Default => Self {
                property: Some(volume_property::Property::VolMode(
                    dataset_properties::VolMode {
                        value: Some(dataset_properties::vol_mode::Value::Default(Variant {})),
                    },
                )),
            },
            property::VolMode::Full => Self {
                property: Some(volume_property::Property::VolMode(
                    dataset_properties::VolMode {
                        value: Some(dataset_properties::vol_mode::Value::Full(Variant {})),
                    },
                )),
            },
            property::VolMode::Geom => Self {
                property: Some(volume_property::Property::VolMode(
                    dataset_properties::VolMode {
                        value: Some(dataset_properties::vol_mode::Value::Geom(Variant {})),
                    },
                )),
            },
            property::VolMode::Dev => Self {
                property: Some(volume_property::Property::VolMode(
                    dataset_properties::VolMode {
                        value: Some(dataset_properties::vol_mode::Value::Dev(Variant {})),
                    },
                )),
            },
            property::VolMode::None => Self {
                property: Some(volume_property::Property::VolMode(
                    dataset_properties::VolMode {
                        value: Some(dataset_properties::vol_mode::Value::None(Variant {})),
                    },
                )),
            },
            property::VolMode::Unknown => Self {
                property: Some(volume_property::Property::VolMode(
                    dataset_properties::VolMode {
                        value: Some(dataset_properties::vol_mode::Value::Unknown(Variant {})),
                    },
                )),
            },
        }
    }
}

impl From<property::CheckSum> for FilesystemProperty {
    fn from(p: property::CheckSum) -> Self {
        match p {
            property::CheckSum::On => Self {
                property: Some(filesystem_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::On(Variant {})),
                    },
                )),
            },
            property::CheckSum::Off => Self {
                property: Some(filesystem_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Off(Variant {})),
                    },
                )),
            },
            property::CheckSum::Fletcher2 => Self {
                property: Some(filesystem_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Fletcher2(Variant {})),
                    },
                )),
            },
            property::CheckSum::Fletcher4 => Self {
                property: Some(filesystem_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Fletcher4(Variant {})),
                    },
                )),
            },
            property::CheckSum::Sha256 => Self {
                property: Some(filesystem_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Sha256(Variant {})),
                    },
                )),
            },
            property::CheckSum::NoParity => Self {
                property: Some(filesystem_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::NoParity(Variant {})),
                    },
                )),
            },
            property::CheckSum::Sha512 => Self {
                property: Some(filesystem_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Sha512(Variant {})),
                    },
                )),
            },
            property::CheckSum::Skein => Self {
                property: Some(filesystem_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Skein(Variant {})),
                    },
                )),
            },
            property::CheckSum::Edonr => Self {
                property: Some(filesystem_property::Property::Checksum(
                    dataset_properties::Checksum {
                        value: Some(dataset_properties::checksum::Value::Edonr(Variant {})),
                    },
                )),
            },
        }
    }
}

impl From<property::Compression> for FilesystemProperty {
    fn from(p: property::Compression) -> Self {
        match p {
            property::Compression::On => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::On(Variant {})),
                    },
                )),
            },
            property::Compression::Off => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Off(Variant {})),
                    },
                )),
            },
            property::Compression::Lzjb => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Lzjb(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip1 => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip1(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip2 => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip2(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip3 => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip3(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip4 => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip4(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip5 => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip5(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip6 => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip6(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip7 => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip7(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip8 => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip8(Variant {})),
                    },
                )),
            },
            property::Compression::Gzip9 => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Gzip9(Variant {})),
                    },
                )),
            },
            property::Compression::Zle => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Zle(Variant {})),
                    },
                )),
            },
            property::Compression::Lz4 => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Lz4(Variant {})),
                    },
                )),
            },
            property::Compression::Zstd => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::Zstd(Variant {})),
                    },
                )),
            },
            property::Compression::ZstdFast => Self {
                property: Some(filesystem_property::Property::Compression(
                    dataset_properties::Compression {
                        value: Some(dataset_properties::compression::Value::ZstdFast(Variant {})),
                    },
                )),
            },
        }
    }
}

impl From<property::OnOffNoAuto> for FilesystemProperty {
    fn from(p: property::OnOffNoAuto) -> Self {
        match p {
            property::OnOffNoAuto::On => Self {
                property: Some(filesystem_property::Property::OnOffNoAuto(
                    dataset_properties::OnOffNoAuto {
                        value: Some(dataset_properties::on_off_no_auto::Value::On(Variant {})),
                    },
                )),
            },
            property::OnOffNoAuto::Off => Self {
                property: Some(filesystem_property::Property::OnOffNoAuto(
                    dataset_properties::OnOffNoAuto {
                        value: Some(dataset_properties::on_off_no_auto::Value::Off(Variant {})),
                    },
                )),
            },
            property::OnOffNoAuto::NoAuto => Self {
                property: Some(filesystem_property::Property::OnOffNoAuto(
                    dataset_properties::OnOffNoAuto {
                        value: Some(dataset_properties::on_off_no_auto::Value::NoAuto(
                            Variant {},
                        )),
                    },
                )),
            },
        }
    }
}

impl From<property::OnOff> for FilesystemProperty {
    fn from(p: property::OnOff) -> Self {
        match p {
            property::OnOff::On => Self {
                property: Some(filesystem_property::Property::OnOff(
                    dataset_properties::OnOff {
                        value: Some(dataset_properties::on_off::Value::On(Variant {})),
                    },
                )),
            },
            property::OnOff::Off => Self {
                property: Some(filesystem_property::Property::OnOff(
                    dataset_properties::OnOff {
                        value: Some(dataset_properties::on_off::Value::Off(Variant {})),
                    },
                )),
            },
        }
    }
}
