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

/// A struct representing the settings and input parameters to a simulation. This
/// is the entry point for the particle data deserialization and initialization of
/// the [Simulation] instance.
#[derive(Serialize, Deserialize)]
pub struct FileDefinition {
    /// TODO document
    pub name: Option<String>,

    /// TODO document
    #[serde(default)]
    pub force: Box<dyn Force>,

    /// TODO document
    #[serde(default)]
    pub algorithm: Box<dyn Simulation>,

    /// TODO document
    #[serde(default)]
    pub args: SimulationArgs,

    /// TODO document
    #[serde(default)]
    pub particles: Vec<ParticleLike>,
}

impl TryFrom<PathBuf> for FileDefinition {
    type Error = Error;

    /// Performs the conversion from [PathBuf] to [FileDefinition] by reading the
    /// file at the specified path and deserializing it based on the file extension.
    /// 
    /// # Supported File Formats
    /// 
    /// | File Format | Extensions | URL |
    /// | --- | --- | --- |
    #[cfg_attr(feature = "yaml", doc = " | YAML      | `.yml`, `.yaml` | https://yaml.org/")]
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        // determines the file format
        let ext = value
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

        #[cfg(feature = "yaml")]
        if matches!(ext.as_ref(), "yml" | "yaml") {
            let file = std::fs::File::open(&value)?;

            let a = serde_yaml::from_reader(file)
                .map_err(|e| Error::new(InvalidInput, format!("Parse error: {e}")))?;

            return Ok(a);
        }

        Err(Error::new(
            InvalidInput,
            format!("Unsupported file extension: `{ext}`"),
        ))
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
