//! # sqldb utils
//!
//! This module provides utility functions for managing the SQL database.
//!
//! ## Usage
//!
//! The module exposes functions to retrieve program name, version, print the prompt,
//! and print database details.
//!
//! The `print_prompt()` function prints the prompt for user input.
//!
//! The `print_db_details()` function prints the database details, including the database name, version,
//! current time, usage hints, and connection status.
//!
//! ## Example
//!
//! ```rust
//! use sqldb::utils;
//!
//! fn main() -> std::io::Result<()> {
//!     utils::print_db_details();
//!     utils::print_prompt()?;
//!     Ok(())
//! }
//! ```

use cargo_metadata::{semver::Version, Metadata, MetadataCommand};
use chrono::Local;
use lazy_static::lazy_static;
use std::io::{self, Write};

/// Prints the prompt for user input.
///
/// This function prints the prompt for user input, which includes the program name followed by `>`.
///
/// # Errors
///
/// Returns an `io::Result` indicating whether the prompt was printed successfully or if an error
/// occurred during the output operation.
///
/// # Examples
///
/// ```rust
/// use sqldb::utils;
///
/// fn main() -> std::io::Result<()> {
///     utils::print_prompt()?;
///     Ok(())
/// }
/// ```
pub fn print_prompt() -> io::Result<()> {
    print!("{} > ", *PKG_NAME);
    io::stdout().flush()?;
    Ok(())
}

/// Prints the database details.
///
/// This function prints the database details, including the program name, version, current time,
/// usage hints, and connection status.
///
/// # Examples
///
/// ```rust
/// # use sqldb::utils;
///
/// fn main() {
///     utils::print_db_details();
/// }
/// ```
pub fn print_db_details() {
    let current_time_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    println!(
        "{} version {} {}",
        *PKG_NAME, *PKG_VERSION, current_time_string
    );
    println!("Enter \".help\" for usage hints.");
    println!("Connected to a transient in-memory database.");
    println!("Use \".open FILENAME\" to reopen on a persistent database.");
    println!("Enter \".exit\" to exit the database.");
}

/// Retrieves the Cargo metadata.
fn retrieve_cargo_metadata() -> Metadata {
    MetadataCommand::new()
        .no_deps()
        .exec()
        .expect("Failed to retrieve cargo metadata")
}

lazy_static! {
    /// The package name obtained from the Cargo.toml file.
    static ref PKG_NAME: String = retrieve_cargo_metadata().packages[0].name.clone();
    /// The package version obtained from the Cargo.toml file.
    static ref PKG_VERSION: Version = retrieve_cargo_metadata().packages[0].version.clone();
}
