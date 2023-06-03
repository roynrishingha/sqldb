//! # sqldb Tokenizer
//!
//! This module provides functions for tokenizing SQL and Metadata commands.
//!
//! ## Usage
//!
//! The module provides an `InputBuffer` struct, which represents the standard input buffer.
//!
//! The `read_input()` function reads a line of input from the user and updates the `InputBuffer`
//! accordingly.
//!
//! ## Examples
//!
//! ```rust
//! use sqldb::sql::tokenizer::InputBuffer;
//!
//! fn main() -> std::io::Result<()> {
//!     let mut input_buffer = InputBuffer::new();
//!
//!     input_buffer.read_input()?;
//!
//!     Ok(())
//! }
//! ```

use std::io::{self, BufRead};

/// Represent an input buffer.
pub struct InputBuffer {
    pub buffer: Option<String>,
    pub buffer_length: usize,
}

impl Default for InputBuffer {
    /// Returns a default instance of the InputBuffer.
    fn default() -> Self {
        Self {
            buffer: None,
            buffer_length: 0,
        }
    }
}

impl InputBuffer {
    /// Creates a new `InputBuffer` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sqldb::sql::tokenizer::InputBuffer;
    ///
    /// let input_buffer = InputBuffer::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads a line of input from the user and updates the `InputBuffer` accordingly.
    ///
    /// # Errors
    ///
    /// Returns an `io::Result` indicating whether the input was read successfully or if an error
    /// occurred during the input operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sqldb::sql::tokenizer::InputBuffer;
    /// #
    /// fn main() -> std::io::Result<()> {
    ///     let mut input_buffer = InputBuffer::new();
    ///
    ///     input_buffer.read_input()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn read_input(&mut self) -> std::io::Result<()> {
        let stdin = io::stdin();
        let mut buffer = String::new();

        stdin.lock().read_line(&mut buffer)?;

        self.buffer = Some(buffer.trim().to_string());
        self.buffer_length = self.buffer.as_ref().map(|s| s.len()).unwrap_or(0);

        // if self.buffer_length == 0 {
        //     eprintln!("No input provided");
        //     std::process::exit(1);
        // }

        Ok(())
    }
}
