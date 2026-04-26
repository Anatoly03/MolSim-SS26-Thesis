//! The IO crate of the molecular dynamics simulation. This library defines
//! every program-understandable file format and exposes file system bindings,
//! such as reading and writing simulation data.
//!
//! # Features
//!
//! - `vtk`: Integrates VTK-support for reading and writing `.vtu` files. This
//!   feature is enabled by default.
//! - `yaml`: Integrates YAML-support for reading and writing `.yaml` files. This
//!   feature is enabled by default.
//! - `json`: Integrates JSON-support for reading `.json` files. (Disabled by
//!   default)

mod reader;
mod writer;

pub use reader::*;
pub use writer::*;

// re-export of core library, not used in the project
pub use moldyn_core as core;
