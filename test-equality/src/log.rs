//! Utility modules for Rust-style logging.

use ansi_term::Colour;
use std::{format, fs::File, io::Write};

// Console logger
pub struct Logger {
    file: Option<File>,
}

impl From<Option<File>> for Logger {
    fn from(file: Option<File>) -> Self {
        Self { file }
    }
}

impl Logger {
    /// See ANSI color codes:
    /// https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124
    pub fn success(&mut self, title: &str, message: &str) {
        let colored = format!("{} {message}\n", Colour::Green.bold().paint(format!("{title:>12}")));
        let colorless = format!("{title:>12} {message}\n");

        print!("{colored}");
        if let Some(f) = &mut self.file {
            let _ = f.write(&colorless.into_bytes());
        }
    }
    
    /// See ANSI color codes:
    /// https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124
    pub fn warn(&mut self, title: &str, message: &str) {
        let colored = format!("{} {message}\n", Colour::Yellow.bold().paint(format!("{title:>12}")));
        let colorless = format!("{title:>12} {message}\n");

        print!("{colored}");
        if let Some(f) = &mut self.file {
            let _ = f.write(&colorless.into_bytes());
        }
    }
    
    /// See ANSI color codes:
    /// https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124
    pub fn failure(&mut self, title: &str, message: &str) {
        let colored = format!("{} {message}\n", Colour::Red.bold().paint(format!("{title:>12}")));
        let colorless = format!("{title:>12} {message}\n");

        print!("{colored}");
        if let Some(f) = &mut self.file {
            let _ = f.write(&colorless.into_bytes());
        }
    }
    
    /// See ANSI color codes:
    /// https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124
    pub fn info(&mut self, title: &str, message: &str) {
        let colored = format!("{} {message}\n", Colour::Cyan.bold().paint(format!("{title:>12}")));
        let colorless = format!("{title:>12} {message}\n");

        print!("{colored}");
        if let Some(f) = &mut self.file {
            let _ = f.write(&colorless.into_bytes());
        }
    }

    /// Logs a header with the given title.
    pub fn header(&mut self, title: String) {
        let value = format!("┌{0}┐\n│  {title}  │\n└{0}┘\n", "─".repeat(title.len() + 4));

        print!("{}", value);
        if let Some(f) = &mut self.file {
            let _ = f.write(&value.into_bytes());
        }
    }
}
