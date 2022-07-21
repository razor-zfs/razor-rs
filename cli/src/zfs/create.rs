use super::*;

#[derive(Debug, clap::Args)]
pub struct Create {
    /// Volume size; if specified ZVOL will be created
    #[clap(short = 'V')]
    volsize: Option<u64>,

    /// Volume block size (equivalent to -o volblocksize=<value>)
    #[clap(short = 'b', requires = "volsize")]
    volblocksize: Option<u64>,

    /// Create a sparse volume with no reservation
    #[clap(short, requires = "volsize")]
    sparse: bool,

    /// Dataset property in property=value format
    #[clap(
        short = 'o',
        // number_of_values = 2,
        // value_names = &["property", "value"],
        // value_delimiter = '=',
        // require_value_delimiter = true,
        value_parser = key_value::parse::<String, String>,
    )]
    properties: Vec<(String, String)>,

    /// Create all the non-existing parent datasets
    #[clap(short)]
    parents: bool,

    /// Do a dry-run creation
    #[clap(short = 'n')]
    dry_run: bool,

    /// Print verbose information about created dataset
    #[clap(short)]
    verbose: bool,

    /// Name of the dataset to create
    dataset: String,
}

impl Create {
    pub fn exec(self) -> anyhow::Result<String> {
        // println!("{self:?}");
        let text = if let Some(size) = self.volsize {
            self.properties
                .iter()
                .fold(zfs::Zfs::volume(), |zvol, (k, v)| zvol.property(k, v))
                .create(&self.dataset, size)?;
            format!("Creating volume {} with size {}", self.dataset, size)
        } else {
            self.properties
                .iter()
                .fold(zfs::Zfs::filesystem(), |zfs, (k, v)| zfs.property(k, v))
                .create(&self.dataset)?;
            format!("Creating filesystem {}", self.dataset)
        };
        Ok(text)
    }
}

// fn volume_properties(
//     builder: zfs::VolumeBuilder,
//     (property, value): (&str, &str),
// ) -> anyhow::Result<zfs::VolumeBuilder> {
//     let builder = match property {
//         "compression" | "compress" => builder.compression(value.parse::<property::Compression>()?),
//         // "volblocksize" => builder.volblocksize(value.parse::<u64>().unwrap()),
//         // "sparse" => builder.sparse(value.parse::<bool>().unwrap()),
//         "checksum" => builder.checksum(value.parse()?),
//         _ => anyhow::bail!("Unknown property: {}", property),
//     };
//     Ok(builder)
// }
