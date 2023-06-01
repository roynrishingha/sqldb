//! # Input Buffer
//!
//! This module provides an input buffer for reading user input from a stream.
//!
//! ## Usage
//!
//! To use the `InputBuffer` struct, create an instance using the `new()` method,
//! and then call the `read_input()` method to read user input from the standard input stream.
//!
//! ```rust
//! # use sqldb::input::InputBuffer;
//!
//! fn main() {
//!     let mut input_buffer = InputBuffer::new();
//!     input_buffer.read_input();
//!     // Use the input_buffer.buffer to access the user input
//! }
//! ```

use std::io::{self, BufRead, BufReader};

/// A struct representing an input buffer.
pub struct InputBuffer {
    /// The user input buffer.
    pub buffer: Option<String>,
    buffer_length: usize,
    input_lenght: usize,
}

// Associated funtions
impl InputBuffer {
    fn read_line_from_stream(stream: &mut dyn io::Read, buffer: &mut String) -> io::Result<usize> {
        // clear the buffer before reading a new line
        buffer.clear();

        // Create a BufReader to efficiently read from the stream
        let mut reader = BufReader::new(stream);

        // Read a line into the buffer
        reader.read_line(buffer)
    }
}

// Methods
impl InputBuffer {
    /// Creates a new instance of the InputBuffer.
    pub fn new() -> Self {
        Self {
            buffer: None,
            buffer_length: 0,
            input_lenght: 0,
        }
    }

    /// Reads user input from the standard input stream.
    pub fn read_input(&mut self) {
        let stdin = io::stdin();
        let mut buffer = String::new();

        let bytes_read = match Self::read_line_from_stream(&mut stdin.lock(), &mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                std::process::exit(1);
            }
        };

        if bytes_read == 0 {
            eprintln!("Error reading input");
            std::process::exit(1);
        }

        // Ignore trailing newline
        self.input_lenght = bytes_read - 1;
        self.buffer = Some(buffer[..bytes_read - 1].to_owned());
    }
}

impl Default for InputBuffer {
    /// Returns a default instance of the InputBuffer.
    fn default() -> Self {
        Self::new()
    }
}
