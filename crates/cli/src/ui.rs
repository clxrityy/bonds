//! The `ui` module provides functions for formatting error messages and other user-facing output in a consistent and visually appealing way. It is used throughout the CLI application to ensure that all messages are presented in a clear and user-friendly manner.

use std::fmt::Display;
use std::io::IsTerminal;

use bonds_core::BondError;
use bonds_core::error::ErrorKind;

const RESET: &str = "\x1b[0m";
const GREEN_BOLD: &str = "\x1b[1;32m";
const YELLOW_BOLD: &str = "\x1b[1;33m";
const RED_BOLD: &str = "\x1b[1;31m";
// const CYAN_BOLD: &str = "\x1b[1;36m";
// const BLUE_BOLD: &str = "\x1b[1;34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const DIM: &str = "\x1b[2m";
const BOLD: &str = "\x1b[1m";
const UNDERLINE: &str = "\x1b[4m";
const BOLD_UNDERLINE: &str = "\x1b[1;4m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const GOLD: &str = "\x1b[38;5;220m";
const LIGHT_BLUE: &str = "\x1b[38;5;117m";
const DIM_BOLD: &str = "\x1b[1;2m";
const NEWLINE: &str = " ";

/// Default landing output shown when running `bond` without subcommands.
/// TODO: This should automatically display the command list and descriptions, but for now it's just a branded message with quick start instructions.
pub fn landing(version: &str) {
    title("bonds");
    info(format!("Version {version}"));
    dim("Organize and manage source-target directory bonds.");
    newline();

    heading("Quick start:");
    normal("  bond add <source> [target] [--name <name>] [--dry-run] [--verbose]");
    normal("  bond list");
    normal("  bond info <id|name>");
    normal("  bond remove <id|name> [--with-target] [--dry-run] [--verbose]");
    normal("  bond update <id|name> [--source <path>] [--target <path>] [--name <name>] [--dry-run] [--verbose]");
    normal("  bond migrate <id|name> [dest] [--dry-run] [--verbose]");
    newline();

    dim("Use `bond --help` for the full command reference.");
}

fn colors_enabled() -> bool {
    // Respect common no-color conventions and avoid ANSI noise when redirected.
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if std::env::var_os("CLICOLOR_FORCE").is_some() {
        return true;
    }

    std::io::stderr().is_terminal() && std::env::var("TERM").map_or(true, |term| term != "dumb")
}

fn paint(text: impl Display, style: &str) -> String {
    let text = text.to_string();
    #[allow(unused_assignments)]
    let mut result = String::with_capacity(style.len() + text.len() + RESET.len());
    if colors_enabled() {
        result = format!("{style}{text}{RESET}");
    } else {
        result = text;
    }
    println!("{}", result);
    result
}

fn style_for(kind: ErrorKind) -> &'static str {
    match kind {
        // These are usually recoverable user-facing situations.
        ErrorKind::NotFound | ErrorKind::Conflict => YELLOW_BOLD,
        // These are hard failures.
        ErrorKind::Input | ErrorKind::Runtime | ErrorKind::Config => RED_BOLD,
    }
}

/// Prints a user-facing message for an error, with a prefix indicating the error type.
pub fn error_prefix(kind: ErrorKind) -> String {
    // Keep the label stable; only the color changes by category.
    paint("Error:", style_for(kind))
}

/// Formats an error message for display to the user, including a colored prefix based on the error kind.
pub fn format_error(err: &BondError) -> String {
    // Used for the final top-level command failure.
    format!("{} {}", error_prefix(err.kind()), err)
}

/// Formats an error message with additional context for display to the user, including a colored prefix based on the error kind. This is useful for providing more information about the error, such as where it occurred or what operation was being attempted when the error happened.
pub fn format_context_error(context: &str, err: &BondError) -> String {
    // Useful for startup/init failures where extra context helps.
    format!("{} {}: {}", error_prefix(err.kind()), context, err)
}

/// Prints a user-facing message for a successful operation.
#[allow(dead_code)]
pub fn success(text: impl Display) -> String {
    paint(text, GREEN)
}

/// Prints a user-facing message for informational purposes.
#[allow(dead_code)]
pub fn info(text: impl Display) -> String {
    paint(text, CYAN)
}

/// Prints a user-facing message for a warning or recoverable issue.
#[allow(dead_code)]
pub fn warning(text: impl Display) -> String {
    paint(text, YELLOW)
}

/// Prints a user-facing message for an error or failure.
#[allow(dead_code)]
pub fn error(text: impl Display) -> String {
    paint(text, RED)
}

/// Prints a user-facing heading or section title.
#[allow(dead_code)]
pub fn title(text: impl Display) -> String {
    paint(format!("\n{}\n", text), BOLD_UNDERLINE)
}

/// Prints a user-facing message for a title or important section header.
#[allow(dead_code)]
pub fn heading(text: impl Display) -> String {
    paint(format!("{}", text), BOLD)
}

/// Prints a user-facing message with underlined text, often used for emphasis or to indicate a sub-section.
#[allow(dead_code)]
pub fn underline(text: impl Display) -> String {
    paint(format!("{}", text), UNDERLINE)
}

/// Prints a user-facing message for a subheading or less prominent section header.
#[allow(dead_code)]
pub fn subheading(text: impl Display) -> String {
    paint(format!("{}", text), DIM_BOLD)
}

/// Prints normal text without any special styling. This can be used for messages that don't fit into the other categories or when you want to reset back to default styling after a heading or label.
#[allow(dead_code)]
pub fn normal(text: impl Display) -> String {
    paint(text, RESET)
}

/// Prints a user-facing label for a key or identifier.
#[allow(dead_code)]
pub fn key(text: impl Display) -> String {
    paint(text, GOLD)
}

/// Prints a user-facing label for an ID or unique identifier.
#[allow(dead_code)]
pub fn id(text: impl Display) -> String {
    paint(text, LIGHT_BLUE)
}

/// Prints a user-facing label for a file path or location.
#[allow(dead_code)]
pub fn path(text: impl Display) -> String {
    paint(text, MAGENTA)
}

/// Prints a user-facing message in a dimmed or less prominent style.
#[allow(dead_code)]
pub fn dim(text: impl Display) -> String {
    paint(text, DIM)
}

/// Prints a user-facing message for a successful status.
#[allow(dead_code)]
pub fn status_ok(text: impl Display) -> String {
    paint(text, GREEN_BOLD)
}

/// Prints a user-facing message for a warning status.
#[allow(dead_code)]
pub fn status_warn(text: impl Display) -> String {
    paint(text, YELLOW_BOLD)
}

/// Prints a user-facing message for an error status.
#[allow(dead_code)]
pub fn status_bad(text: impl Display) -> String {
    paint(text, RED_BOLD)
}

/// Prints a newline to the user-facing output. This can be used to separate sections or add spacing between messages for better readability.
#[allow(dead_code)]
pub fn newline() {
    paint("", NEWLINE);
}

/// Prints a user-facing message for a debug or verbose log, which is only shown when the verbose mode is enabled. This can be used to provide additional information about the internal state or operations of the application that may be helpful for troubleshooting or understanding the behavior of the application, but is not necessary for regular usage.
#[allow(dead_code)]
pub fn debug(text: impl Display) -> String {
    paint(text, DIM)
}
