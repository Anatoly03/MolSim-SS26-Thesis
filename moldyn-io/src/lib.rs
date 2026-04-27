//! The IO crate of the molecular dynamics simulation. This library defines
//! every program-understandable file format and exposes file system bindings,
//! such as reading and writing simulation data.
//!
//! # Features
//!
//! > **NOTE**: Currently, if the `json` and `yaml` features are disabled, this
//! program is unusable due to not having a default file format for file reading.
//!
//! ### `vtk` (default)
//! 
//! Integrates VTK-support for reading and writing `.vtu` files. This feature
//! is enabled by default.
//! 
//! ### `yaml` (default)
//! 
//! Integrates YAML-support for reading and writing `.yaml` files. This feature
//! is enabled by default.
//! 
//! ### `json`
//! 
//! Integrates JSON-support for reading `.json` files. Disabled by default.

pub mod reader;
pub mod writer;

pub use reader::*;
pub use writer::*;

// re-export of core library, not used in the project
pub use moldyn_core as core;
