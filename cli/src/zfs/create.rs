use super::*;

#[derive(Debug, Parser)]
pub struct Create {
    #[clap(short = 'V', help = "Volume size; if specified ZVOL will be created")]
    volsize: Option<u64>,
    #[clap(short = 'o')]
    properties: Vec<String>,
    #[clap(help = "Name of the dataset to create")]
    dataset: String,
}

impl Create {
    pub fn exec(self) -> anyhow::Result<String> {
        let text = if let Some(volsize) = self.volsize {
            format!("Creating volume {} with size {}", self.dataset, volsize)
        } else {
            format!("Creating filesystem {}", self.dataset)
        };
        Ok(text)
    }
}
