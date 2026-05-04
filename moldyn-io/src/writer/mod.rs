//! This module manages the simulation writer. It provides the [OutputWriter]
//! trait which is capable of selecting the correct writing method based on
//! the given output path.
//!
//! This module also re-exports the individual writers for the supported file
//! formats locked behind the respective features.

mod txt;
#[cfg(feature = "vtk")]
mod vtk;
mod xyz;
#[cfg(feature = "yaml")]
mod yml;

pub use txt::TxtWriter;
#[cfg(feature = "vtk")]
pub use vtk::VtkWriter;
pub use xyz::XyzWriter;
#[cfg(feature = "yaml")]
pub use yml::YamlWriter;

use moldyn_core::SimulationTrait;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::{Error, ErrorKind::InvalidInput, Result};
use std::{ffi::OsStr, path::Path};

/// A trait for writing simulation results to an output file.
pub trait OutputWriter {
    /// The next frame number to write. This is used primarily in file writing to
    /// generate ascending file names.
    fn next_frame_number(&mut self) -> usize;

    /// Writes a single frame of the simulation state to the provided writer.
    fn write_frame(&mut self, writer: &mut BufWriter<File>, state: &dyn SimulationTrait) -> Result<()>;

    /// The file extension used in file writing.
    fn extension(&self) -> &str;

    /// Writes the simulation results.
    fn write(&mut self, path: &Path, state: &dyn SimulationTrait) -> Result<()> {
        let full_file_path = path.parent().unwrap_or(Path::new(".")).join(format!(
            // https://stackoverflow.com/questions/50458144/what-is-the-easiest-way-to-pad-a-string-with-0-to-the-left
            "{}_{:0>4}.{}",
            // map to UTF sequences: https://doc.rust-lang.org/std/ffi/struct.OsStr.html#method.to_string_lossy
            // this theoretically makes using tbis program on Windows possible but no clue tbf
            // something to do with endians and UTF-16
            path.with_extension("")
                .file_name()
                .unwrap_or(OsStr::new("out"))
                .to_string_lossy(),
            self.next_frame_number(),
            self.extension()
        ));

        let file = File::create(&full_file_path)?;
        let mut writer = BufWriter::new(file);

        self.write_frame(&mut writer, state)?;
        writer.flush()?;

        Ok(())
    }
}

impl dyn OutputWriter {
    /// Creates a new output writer from a file extension. The extension passed
    /// as argument is case-insensitive and does not include the leading dot.
    ///
    /// # Supported File Formats
    ///
    /// | File Format | Extensions | URL |
    /// | --- | --- | --- |
    /// | Text      | `.txt`, `.text` |
    /// | XYZ       | `.xyz`          | <http://openbabel.org/wiki/XYZ_(format)>
    #[cfg_attr(
        feature = "vtk",
        doc = " | VTK       | `.vtk`, `.vtu`  | <https://en.wikipedia.org/wiki/VTK>"
    )]
    #[cfg_attr(
        feature = "yaml",
        doc = " | YAML      | `.yml`, `.yaml` | <https://yaml.org>/"
    )]
    pub fn from_extension(extension: &str) -> Result<Box<dyn OutputWriter>> {
        let ext = extension.to_ascii_lowercase();

        if matches!(ext.as_ref(), "txt" | "text") {
            return Ok(Box::new(TxtWriter::default()));
        }

        if matches!(ext.as_ref(), "xyz") {
            return Ok(Box::new(XyzWriter::default()));
        }

        #[cfg(feature = "vtk")]
        if matches!(ext.as_ref(), "vtk" | "vtu") {
            return Ok(Box::new(VtkWriter::default()));
        }

        #[cfg(feature = "yaml")]
        if matches!(ext.as_ref(), "yml" | "yaml") {
            return Ok(Box::new(YamlWriter::default()));
        }

        Err(Error::new(
            InvalidInput,
            format!("Unsupported file extension: `{ext}`"),
        ))
    }
}
