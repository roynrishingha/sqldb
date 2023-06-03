//! # sqldb parser
//!
//! The `parser` module provides functions and types for parsing and executing SQL & Meta commands.
//!
//! ## Usage
//!
//! The `run_command` function is the main entry point for executing SQL and Meta commands. It takes an input buffer and a mutable `Command` struct as parameters. The input buffer contains the command to be parsed and executed, while the `Command` struct holds the parsed command for execution.
//!
//! ## Examples
//!
//! ```rust
//! use sqldb::sql::tokenizer::InputBuffer;
//! use sqldb::sql::parser::{run_command, Command};
//!
//! let input_buffer = InputBuffer::new();
//! let mut command = Command::new();
//!
//! run_command(&input_buffer, &mut command);
//! ```
//!
//! ## Structs
//!
//! ### `Command`
//!
//! A struct representing a parsed command.
//!
//! #### Fields
//!
//! - `variant`: An `Option` that holds the type of the command (`CommandType`).
//!
//! #### Methods
//!
//! - `new()`: Creates a new `Command` instance with the default values.
//!
//! ## Enums
//!
//! ### `CommandType`
//!
//! An enum representing the type of a command.
//!
//! #### Variants
//!
//! - `MetaCommand(MetaCommand)`: Represents a meta command.
//! - `Query(Query)`: Represents a SQL query command.
//!
//! ### `MetaCommand`
//!
//! An enum representing the different types of meta commands.
//!
//! #### Variants
//!
//! - `Exit`: Represents the exit meta command.
//! - `Help`: Represents the help meta command.
//!
//! ### `Query`
//!
//! An enum representing the different types of SQL queries.
//!
//! #### Variants
//!
//! - `Insert`: Represents an INSERT query.
//! - `Select`: Represents a SELECT query.
//!
//! ### `CommandError`
//!
//! An enum representing the possible errors that can occur when processing a command.
//!
//! #### Variants
//!
//! - `EmptyBuffer`: The input buffer is empty.
//! - `InvalidBuffer`: The input buffer is invalid.
//! - `UnrecognizedMetaCommand(String)`: The meta command is not recognized.
//! - `UnrecognizedQuery(String)`: The SQL query is not recognized.
//!
//! ## Functions
//!
//! ### `run_command`
//!
//! Executes a command by parsing and executing it.
//!
//! ```rust
//! use sqldb::sql::tokenizer::InputBuffer;
//! use sqldb::sql::parser::{run_command, Command};
//!
//! let input_buffer = InputBuffer::new();
//! let mut command = Command::new();
//!
//! run_command(&input_buffer, &mut command);
//! ```
//!
//! ### `parse_command`
//!
//! Parses a command and updates the `Command` struct with the parsed command.
//!
//! ### `execute_command`
//!
//! Executes a parsed command.
//!
//! ### `parse_meta_command`
//!
//! Parses a meta command and returns the corresponding `MetaCommand` enum variant.
//!
//! ## Error Handling
//!
//! The `CommandError` enum represents the possible errors that can occur when processing a command. It provides descriptive error messages for each error variant.
//!

use crate::sql::tokenizer::InputBuffer;
use std::process;

pub fn run_command(input_buffer: &InputBuffer, command: &mut Command) {
    match parse_command(input_buffer, command) {
        Ok(()) => match execute_command(command) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Error executing command: {}", err);
            }
        },
        Err(err) => match err {
            CommandError::EmptyBuffer => {
                eprintln!("Input buffer is empty.");
            }
            CommandError::InvalidBuffer => {
                eprintln!("Invalid input buffer.");
            }
            CommandError::UnrecognizedMetaCommand(cmd) => {
                eprintln!("Unrecognized command: '{}'.", cmd);
            }
            CommandError::UnrecognizedQuery(query) => {
                eprintln!("Unrecognized query: '{}'.", query);
            }
        },
    }
}

type CommandResult<T> = Result<T, CommandError>;

/// Represents the possible errors that can occur when processing a SQL command.
enum CommandError {
    /// The input buffer is empty.
    EmptyBuffer,
    /// The input buffer is invalid.
    InvalidBuffer,
    /// The meta command is not recognized.
    UnrecognizedMetaCommand(String),
    /// The SQL query is not recognized.
    UnrecognizedQuery(String),
}

/// Represents the different types of meta commands.
enum MetaCommand {
    /// Exit the SQL shell.
    Exit,
    /// Display help information.
    Help,
    // Add more meta commands as needed
}

enum Query {
    Insert,
    Select,
    // Add more SQL queries as needed
}

enum CommandType {
    MetaCommand(MetaCommand),
    Query(Query),
}

#[derive(Default)]
pub struct Command {
    variant: Option<CommandType>,
}

fn parse_command(input_buffer: &InputBuffer, command: &mut Command) -> CommandResult<()> {
    let buffer_content = input_buffer
        .buffer
        .as_ref()
        .ok_or(CommandError::EmptyBuffer)?;

    if buffer_content.is_empty() {
        return Err(CommandError::InvalidBuffer);
    }

    if buffer_content.starts_with('.') {
        if let Some(meta_command) = parse_meta_command(buffer_content) {
            command.variant = Some(CommandType::MetaCommand(meta_command));
            Ok(())
        } else {
            Err(CommandError::UnrecognizedMetaCommand(
                buffer_content.to_string(),
            ))
        }
    } else if buffer_content.starts_with("select") {
        command.variant = Some(CommandType::Query(Query::Select));
        Ok(())
    } else if buffer_content.starts_with("insert") {
        command.variant = Some(CommandType::Query(Query::Insert));
        Ok(())
    } else {
        Err(CommandError::UnrecognizedQuery(buffer_content.to_string()))
    }
}

fn execute_command(command: &Command) -> CommandResult<()> {
    match &command.variant {
        Some(CommandType::MetaCommand(meta_command)) => match meta_command {
            MetaCommand::Exit => process::exit(0),
            MetaCommand::Help => {
                println!(".help command executed.");
                Ok(())
            } // Handle more meta commands as needed
        },
        Some(CommandType::Query(query)) => match query {
            Query::Insert => {
                println!("This is where we would do an insert.");
                Ok(())
            }
            Query::Select => {
                println!("This is where we would do a select.");
                Ok(())
            } // Handle more SQL queries as needed
        },
        None => Err(CommandError::UnrecognizedQuery(
            "Unrecognized command".to_string(),
        )),
    }
}

fn parse_meta_command(buffer_content: &str) -> Option<MetaCommand> {
    match buffer_content {
        ".exit" => Some(MetaCommand::Exit),
        ".help" => Some(MetaCommand::Help),
        // Handle more meta commands here
        _ => None,
    }
}

impl Command {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::EmptyBuffer => write!(f, "Input buffer is empty."),
            CommandError::InvalidBuffer => write!(f, "Invalid input buffer."),
            CommandError::UnrecognizedMetaCommand(cmd) => {
                write!(f, "Unrecognized meta command: '{}'.", cmd)
            }
            CommandError::UnrecognizedQuery(query) => write!(f, "Unrecognized query: '{}'.", query),
            // Add custom error messages for more error variants
        }
    }
}
