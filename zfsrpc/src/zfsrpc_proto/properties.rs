use crate::zfsrpc_proto::tonic_zfsrpc::{
    dataset_properties, filesystem_property, volume_property, FilesystemProperty, Variant,
    VolumeProperty,
};
use prop_macro::{classcase, classcase_path_end, snakecase_fn};
use razor_zfs::zfs::property;

impl dataset_properties::BlockSize {
    pub(crate) fn check(&self) -> std::io::Result<Self> {
        todo!("implement blocksize restrictions")
    }
}

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
macro_rules! impl_property_for_zfs{

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
