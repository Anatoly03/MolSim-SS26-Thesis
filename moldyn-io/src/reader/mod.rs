//! TODO document

mod entry;

use moldyn_core::{Force, Simulation, SimulationArgs};
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
    /// The name of the simulation. This is an optional field that can be used to
    /// sign a simulation input file with human-understandable semantics.
    /// 
    /// # Example
    /// 
    /// ```yaml
    /// # This is a simple named simulation without any particles.
    /// name: My Simulation
    /// particles: []
    /// ```
    pub name: Option<String>,

    /// The force model to use in the simulation. This affects the computation of
    /// the [potential energy](https://en.wikipedia.org/wiki/Potential_energy) and
    /// [forces](https://en.wikipedia.org/wiki/Force) between particle and yields
    /// different results.
    /// 
    /// # Example
    /// 
    /// ```yaml
    /// name: My Lennard-Jones Simulation
    /// force: lennard-jones
    /// ```
    /// 
    /// Or alternatively, you can set the force model to `newton` to simulate
    /// gravitational interactions:
    /// 
    /// ```yaml
    /// name: My Gravitational Simulation
    /// force: newton
    /// ```
    /// 
    /// # Note on Mass Scale
    /// 
    /// Note that different force models require a different scale for the
    /// particle mass. For example, for Lennard-Jones simulations, the mass is
    /// typically around `1.0` for every body, while for Newtonian simulations the
    /// "mass scale" is around `1.0` for heavy bodies like the sun and a fraction
    /// of that for lighter bodies.
    /// 
    /// In the simulation of Halleys Comet orbiting the solar system, each body has
    /// the following mass:
    /// 
    /// - Sun: `1.0`
    /// - Earth: `3.0 e-6`
    /// - Jupiter: `9.5 e-4`
    /// - Halleys Comet: `1.0 e-14`
    /// 
    /// Therefore representing in kilograms, you can assume that the mass of `1.0`
    /// corresponds to `1.988 e30 kg`.
    #[serde(default)]
    pub force: Box<dyn Force>,

    /// The simulation algorithm to use in the simulation. This affects performance
    /// and the memory structure of the simulation when managing the particle data.
    /// A change in the algorithm should, as a rule of thumbs, not yield different
    /// results for the same input.
    /// 
    /// # Example
    /// 
    /// ```yaml
    /// name: My Simulation
    /// algorithm: direct-sum
    /// ```
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
    #[cfg_attr(feature = "json", doc = " | JSON      | `.json`         | https://www.json.org/json-en.html")]
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

        #[cfg(feature = "json")]
        if matches!(ext.as_ref(), "json") {
            let file = std::fs::File::open(&value)?;

            let a = serde_json::from_reader(file)
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
