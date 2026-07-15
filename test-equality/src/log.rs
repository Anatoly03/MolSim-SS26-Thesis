//! Utility modules for Rust-style logging.

use ansi_term::Colour;

// Console logger
pub enum Log {
    Success,
    Warn,
    Failure,
    Info,
}

impl Log {
    pub fn log(&self, title: &str, message: &str) {
        // ANSI Color codes
        // https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124
        let prefix_color = match self {
            Log::Success => Colour::Green,
            Log::Warn => Colour::Yellow,
            Log::Failure => Colour::Red,
            Log::Info => Colour::Cyan,
        };

        println!("{} {message}", prefix_color.bold().paint(format!("{title:>12}")));
    }

    /// Logs a header with the given title.
    pub fn header(title: String) {
        println!("┌{}┐", "─".repeat(title.len() + 4));
        println!("│  {title}  │");
        println!("└{}┘", "─".repeat(title.len() + 4));
    }
}
