pub mod sql {
    pub mod parser;
    pub mod tokenizer;
}
pub mod utils;

use crate::{
    sql::{
        parser::{run_command, Command},
        tokenizer::InputBuffer,
    },
    utils::{print_db_details, print_prompt},
};

/// Runs the SQLDB application.
///
/// This function initializes the necessary components, such as the input buffer and command objects,
/// and starts a loop to read and process user input until the user enters the `.exit` command.
/// During each iteration, the user's input is read, and the corresponding SQL command is executed.
///
/// # Errors
///
/// Returns an `std::io::Result` indicating whether the operation was successful or encountered an I/O error.
pub fn run() -> std::io::Result<()> {
    let mut input_buffer = InputBuffer::new();
    let mut command = Command::new();

    print_db_details();

    loop {
        print_prompt()?;
        input_buffer.read_input()?;

        if input_buffer.buffer == Some(".exit".to_string()) {
            break;
        }

        run_command(&input_buffer, &mut command);
    }

    Ok(())
}
