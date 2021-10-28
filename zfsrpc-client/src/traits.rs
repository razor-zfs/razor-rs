use super::property;
use super::proto;

impl From<property::CheckSum> for proto::VolumeProperty {
    fn from(p: property::CheckSum) -> Self {
        match p {
            property::CheckSum::On => Self {
                property: Some(proto::volume_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::On(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Off => Self {
                property: Some(proto::volume_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Off(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Fletcher2 => Self {
                property: Some(proto::volume_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Fletcher2(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Fletcher4 => Self {
                property: Some(proto::volume_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Fletcher4(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Sha256 => Self {
                property: Some(proto::volume_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Sha256(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::NoParity => Self {
                property: Some(proto::volume_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::NoParity(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Sha512 => Self {
                property: Some(proto::volume_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Sha512(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Skein => Self {
                property: Some(proto::volume_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Skein(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Edonr => Self {
                property: Some(proto::volume_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Edonr(
                            proto::Variant {},
                        )),
                    },
                )),
            },
        }
    }
}

impl From<property::Compression> for proto::VolumeProperty {
    fn from(p: property::Compression) -> Self {
        match p {
            property::Compression::On => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::On(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Off => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Off(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Lzjb => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Lzjb(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip1 => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip1(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip2 => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip2(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip3 => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip3(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip4 => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip4(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip5 => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip5(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip6 => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip6(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip7 => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip7(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip8 => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip8(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip9 => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip9(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Zle => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Zle(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Lz4 => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Lz4(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Zstd => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Zstd(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::ZstdFast => Self {
                property: Some(proto::volume_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::ZstdFast(
                            proto::Variant {},
                        )),
                    },
                )),
            },
        }
    }
}

impl From<property::VolMode> for proto::VolumeProperty {
    fn from(p: property::VolMode) -> Self {
        match p {
            property::VolMode::Default => Self {
                property: Some(proto::volume_property::Property::VolMode(
                    proto::dataset_properties::VolMode {
                        value: Some(proto::dataset_properties::vol_mode::Value::Default(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::VolMode::Full => Self {
                property: Some(proto::volume_property::Property::VolMode(
                    proto::dataset_properties::VolMode {
                        value: Some(proto::dataset_properties::vol_mode::Value::Full(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::VolMode::Geom => Self {
                property: Some(proto::volume_property::Property::VolMode(
                    proto::dataset_properties::VolMode {
                        value: Some(proto::dataset_properties::vol_mode::Value::Geom(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::VolMode::Dev => Self {
                property: Some(proto::volume_property::Property::VolMode(
                    proto::dataset_properties::VolMode {
                        value: Some(proto::dataset_properties::vol_mode::Value::Dev(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::VolMode::None => Self {
                property: Some(proto::volume_property::Property::VolMode(
                    proto::dataset_properties::VolMode {
                        value: Some(proto::dataset_properties::vol_mode::Value::None(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::VolMode::Unknown => Self {
                property: Some(proto::volume_property::Property::VolMode(
                    proto::dataset_properties::VolMode {
                        value: Some(proto::dataset_properties::vol_mode::Value::Unknown(
                            proto::Variant {},
                        )),
                    },
                )),
            },
        }
    }
}

impl From<property::CheckSum> for proto::FilesystemProperty {
    fn from(p: property::CheckSum) -> Self {
        match p {
            property::CheckSum::On => Self {
                property: Some(proto::filesystem_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::On(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Off => Self {
                property: Some(proto::filesystem_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Off(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Fletcher2 => Self {
                property: Some(proto::filesystem_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Fletcher2(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Fletcher4 => Self {
                property: Some(proto::filesystem_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Fletcher4(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Sha256 => Self {
                property: Some(proto::filesystem_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Sha256(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::NoParity => Self {
                property: Some(proto::filesystem_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::NoParity(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Sha512 => Self {
                property: Some(proto::filesystem_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Sha512(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Skein => Self {
                property: Some(proto::filesystem_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Skein(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::CheckSum::Edonr => Self {
                property: Some(proto::filesystem_property::Property::Checksum(
                    proto::dataset_properties::Checksum {
                        value: Some(proto::dataset_properties::checksum::Value::Edonr(
                            proto::Variant {},
                        )),
                    },
                )),
            },
        }
    }
}

impl From<property::Compression> for proto::FilesystemProperty {
    fn from(p: property::Compression) -> Self {
        match p {
            property::Compression::On => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::On(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Off => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Off(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Lzjb => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Lzjb(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip1 => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip1(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip2 => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip2(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip3 => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip3(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip4 => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip4(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip5 => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip5(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip6 => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip6(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip7 => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip7(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip8 => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip8(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Gzip9 => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Gzip9(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Zle => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Zle(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Lz4 => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Lz4(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::Zstd => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::Zstd(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::Compression::ZstdFast => Self {
                property: Some(proto::filesystem_property::Property::Compression(
                    proto::dataset_properties::Compression {
                        value: Some(proto::dataset_properties::compression::Value::ZstdFast(
                            proto::Variant {},
                        )),
                    },
                )),
            },
        }
    }
}

impl From<property::OnOffNoAuto> for proto::FilesystemProperty {
    fn from(p: property::OnOffNoAuto) -> Self {
        match p {
            property::OnOffNoAuto::On => Self {
                property: Some(proto::filesystem_property::Property::OnOffNoAuto(
                    proto::dataset_properties::OnOffNoAuto {
                        value: Some(proto::dataset_properties::on_off_no_auto::Value::On(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::OnOffNoAuto::Off => Self {
                property: Some(proto::filesystem_property::Property::OnOffNoAuto(
                    proto::dataset_properties::OnOffNoAuto {
                        value: Some(proto::dataset_properties::on_off_no_auto::Value::Off(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::OnOffNoAuto::NoAuto => Self {
                property: Some(proto::filesystem_property::Property::OnOffNoAuto(
                    proto::dataset_properties::OnOffNoAuto {
                        value: Some(proto::dataset_properties::on_off_no_auto::Value::NoAuto(
                            proto::Variant {},
                        )),
                    },
                )),
            },
        }
    }
}

impl From<property::OnOff> for proto::FilesystemProperty {
    fn from(p: property::OnOff) -> Self {
        match p {
            property::OnOff::On => Self {
                property: Some(proto::filesystem_property::Property::OnOff(
                    proto::dataset_properties::OnOff {
                        value: Some(proto::dataset_properties::on_off::Value::On(
                            proto::Variant {},
                        )),
                    },
                )),
            },
            property::OnOff::Off => Self {
                property: Some(proto::filesystem_property::Property::OnOff(
                    proto::dataset_properties::OnOff {
                        value: Some(proto::dataset_properties::on_off::Value::Off(
                            proto::Variant {},
                        )),
                    },
                )),
            },
        }
    }
}
