# sqldb

A Sqlite clone in Rust for learning purpose.

## High-level Architecture

```
Tokenizer -> Parser -> Code Generator -> Virtual Machine -> B-Tree -> Pager -> Os Interface
```

## Current Project Structure

- `lib.rs`: This file serves as the main entry point for sqldb library crate. It provides an overview of the library's functionality, including modules for query parsing and tokenizing, as well as general database management utilities.

- `main.rs`: This file serves as the entry point for sqldb binary crate. It imports the necessary modules from sqldb library and contains the main function that sets up the command-line interface for interacting with the database.

- `sql`: This directory contains the modules related to SQL parsing and tokenizing.
      
    - `parser.rs`: This file contains functions for parsing SQL and meta commands. It defines structs and enums representing parsed commands, command types, and command results.
      
    - `tokenizer.rs`: This file contains functions for tokenizing SQL and metadata commands. It defines a struct representing the input buffer and provides a function for reading user input.

- `utils.rs`: This file contains utility functions for managing the SQL database, such as printing prompts and database details.
