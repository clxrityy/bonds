//! This crate implements the command line interface (CLI) for the bond management application. It provides a user-friendly way to interact with the core functionality exposed by the `bonds-core` crate, allowing users to create, list, update, and delete bonds through terminal commands. The CLI is built using the `clap` crate for argument parsing and structuring of commands and subcommands.
//! The main components of this crate include:
//! - `args`: Defines the command line argument structures and parsing logic using the `clap` crate.
//! - `commands`: Contains the handler functions for each CLI subcommand, which interact with the BondManager to perform the requested operations.
//! - `ui`: Provides helper functions for rendering output to the user, such as success messages, error messages, and informational logs, with consistent formatting and coloring.
//! The CLI relies on the core crate for all business logic and data management, ensuring a clear separation of concerns between the user interface and the underlying functionality.

/// Command line argument parsing using `clap`.
pub mod args;
/// UI rendering helpers for consistent output formatting.
pub mod ui;
