use crate::{Force, Particle};
use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind::InvalidInput},
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
pub struct FileDefinition {
    pub name: String,

    #[serde(skip_serializing, default)]
    pub force: Box<dyn Force>,

    #[serde(default)]
    pub particles: Vec<Particle>,
}

impl TryFrom<PathBuf> for FileDefinition {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        // determines the file format
        let file_extension = value
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        match file_extension.as_str() {
            "yaml" | "yml" => {
                let file = std::fs::File::open(&value)?;

                let a = serde_yaml::from_reader(file)
                    .map_err(|e| Error::new(InvalidInput, format!("parse error: {e}")))?;

                Ok(a)
            }
            _ => Err(Error::new(
                InvalidInput,
                format!("unsupported file extension: {file_extension}"),
            )),
        }
    }
}
