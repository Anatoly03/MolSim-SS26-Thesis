//! TODO document

mod txt;
mod xyz;

use crate::writer::xyz::XyzWriter;
use crate::{Simulation, writer::txt::TxtWriter};
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
    fn write_frame(
        &mut self,
        writer: &mut BufWriter<File>,
        state: &Box<dyn Simulation>,
    ) -> Result<()>;

    fn extension(&self) -> &str;

    /// Writes the simulation results.
    fn write(&mut self, path: &Path, state: &Box<dyn Simulation>) -> Result<()> {
        let full_file_path = path.parent().unwrap_or(Path::new(".")).join(format!(
            // https://stackoverflow.com/questions/50458144/what-is-the-easiest-way-to-pad-a-string-with-0-to-the-left
            "{}_{:0>8}.{}",
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
    /// Creates a new output writer from a file extension. Supported extensions are:
    /// `txt`, `xyz`
    pub fn from_extension(extension: &str) -> Result<Box<dyn OutputWriter>> {
        match extension.to_ascii_lowercase().as_str() {
            "txt" | "text" => Ok(Box::new(TxtWriter::default())),
            "xyz" => Ok(Box::new(XyzWriter::default())),
            f => Err(Error::new(
                InvalidInput,
                format!("Unsupported file extension: `{f}`"),
            )),
        }
    }
}
