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
//! ## Examples
//!
//! ```rust
//! use sqldb::utils::{print_db_details, print_prompt};
//!
//! fn main() -> std::io::Result<()> {
//!     print_db_details();
//!     print_prompt()?;
//!     Ok(())
//! }
//! ```

use chrono::Local;
use once_cell::sync::Lazy;
use std::fs;
use std::io::{self, Write};
use toml::Value;

// Metadata struct holding the program name and version.
struct Metadata {
    name: String,
    version: String,
}

static METADATA: Lazy<Metadata> = Lazy::new(retrieve_metadata);

/// Prints the prompt for user input.
///
/// This function prints the prompt for user input, which includes the program name followed by `>`.
///
/// # Errors
///
/// Returns an `std::io::Result` indicating whether the prompt was printed successfully or if an error
/// occurred during the output operation.
///
/// # Examples
///
/// ```rust
/// use sqldb::utils::print_prompt;
///
/// fn main() -> std::io::Result<()> {
///     print_prompt()?;
///     Ok(())
/// }
/// ```
pub fn print_prompt() -> io::Result<()> {
    print!("{} > ", METADATA.name);
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
/// # use sqldb::utils::print_db_details;
///
/// fn main() {
///     print_db_details();
/// }
/// ```
pub fn print_db_details() {
    let current_time_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    println!(
        "{} version {} {}",
        METADATA.name, METADATA.version, current_time_string
    );
    println!("Enter \".help\" for usage hints.");
    println!("Connected to a transient in-memory database.");
    println!("Use \".open FILENAME\" to reopen on a persistent database.");
    println!("Enter \".exit\" to exit the database.");
}

fn retrieve_metadata() -> Metadata {
    let toml_content = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml file");

    let toml: Value = toml::from_str(&toml_content).expect("Failed to parse Cargo.toml content");

    let package = toml
        .get("package")
        .and_then(|value| value.as_table())
        .expect("Failed to find 'package' section in Cargo.toml");

    let name = package
        .get("name")
        .and_then(|value| value.as_str())
        .expect("Failed to find 'name' field in 'package' section")
        .to_owned();

    let version = package
        .get("version")
        .and_then(|value| value.as_str())
        .expect("Failed to find 'version' field in 'package' section")
        .to_owned();

    Metadata { name, version }
}
