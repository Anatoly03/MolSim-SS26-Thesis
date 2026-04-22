use crate::{Force, Particle, simulation::Simulation};
use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind::InvalidInput},
    path::PathBuf,
    sync::Arc,
};

#[derive(Serialize, Deserialize)]
pub struct FileDefinition {
    pub name: String,

    #[serde(skip_serializing, default)]
    pub force: Box<dyn Force>,

    #[serde(skip_serializing, default)]
    pub algorithm: Box<dyn Simulation>,

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

impl From<FileDefinition> for Box<dyn Simulation> {
    fn from(value: FileDefinition) -> Self {
        let FileDefinition {
            name: _,
            force,
            mut algorithm,
            particles,
        } = value;

        let force_arc = Arc::from(force);
        algorithm.set_force(force_arc);
        algorithm.set_particles(particles);

        algorithm
    }
}
