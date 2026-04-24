//! TODO document

mod args;
mod entry;

use crate::{Force, simulation::Simulation};
pub use args::SimulationArgs;
pub use entry::ParticleLike;
use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind::InvalidInput},
    path::PathBuf,
    sync::Arc,
};

/// TODO document
#[derive(Serialize, Deserialize)]
pub struct FileDefinition {
    /// TODO document
    pub name: Option<String>,

    /// TODO document
    #[serde(skip_serializing, default)]
    pub force: Box<dyn Force>,

    /// TODO document
    #[serde(skip_serializing, default)]
    pub algorithm: Box<dyn Simulation>,

    /// TODO document
    pub args: SimulationArgs,

    /// TODO document
    #[serde(default)]
    pub particles: Vec<ParticleLike>,
}

impl TryFrom<PathBuf> for FileDefinition {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        // determines the file format
        let file_extension = value
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or(Error::new(
                InvalidInput,
                format!(
                    "No file extension provided, could not select parser for `{}`",
                    value.display()
                ),
            ))?
            .to_ascii_lowercase();

        match file_extension.as_str() {
            "yaml" | "yml" => {
                let file = std::fs::File::open(&value)?;

                let a = serde_yaml::from_reader(file)
                    .map_err(|e| Error::new(InvalidInput, format!("Parse error: {e}")))?;

                Ok(a)
            }
            _ => Err(Error::new(
                InvalidInput,
                format!("Unsupported file extension: `{file_extension}`"),
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
            args,
            particles,
        } = value;

        algorithm.set_force(Arc::from(force));
        algorithm.set_args(args);

        for p in particles.into_iter() {
            algorithm.add_particles(p.into());
        }

        algorithm
    }
}
