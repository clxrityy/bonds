use std::fmt::Display;
use std::io::IsTerminal;

use bonds_core::BondError;
use bonds_core::error::ErrorKind;

const RESET: &str = "\x1b[0m";
const GREEN_BOLD: &str = "\x1b[1;32m";
const YELLOW_BOLD: &str = "\x1b[1;33m";
const RED_BOLD: &str = "\x1b[1;31m";
const CYAN_BOLD: &str = "\x1b[1;36m";
const BLUE_BOLD: &str = "\x1b[1;34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const DIM: &str = "\x1b[2m";

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

pub fn error_prefix(kind: ErrorKind) -> String {
    // Keep the label stable; only the color changes by category.
    paint("Error:", style_for(kind))
}

pub fn format_error(err: &BondError) -> String {
    // Used for the final top-level command failure.
    format!("{} {}", error_prefix(err.kind()), err)
}

pub fn format_context_error(context: &str, err: &BondError) -> String {
    // Useful for startup/init failures where extra context helps.
    format!("{} {}: {}", error_prefix(err.kind()), context, err)
}

/// Prints a user-facing message for a successful operation.
#[allow(dead_code)]
pub fn success(text: impl Display) -> String {
    paint(text, GREEN_BOLD)
}

/// Prints a user-facing message for informational purposes.
#[allow(dead_code)]
pub fn info(text: impl Display) -> String {
    paint(text, CYAN_BOLD)
}

/// Prints a user-facing message for a warning or recoverable issue.
#[allow(dead_code)]
pub fn warning(text: impl Display) -> String {
    paint(text, YELLOW_BOLD)
}

/// Prints a user-facing message for an error or failure.
#[allow(dead_code)]
pub fn error(text: impl Display) -> String {
    paint(text, RED_BOLD)
}

/// Prints a user-facing heading or section title.
#[allow(dead_code)]
pub fn heading(text: impl Display) -> String {
    paint(text, CYAN_BOLD)
}

/// Prints a user-facing label for a key or identifier.
#[allow(dead_code)]
pub fn key(text: impl Display) -> String {
    paint(text, BLUE_BOLD)
}

/// Prints a user-facing label for an ID or unique identifier.
#[allow(dead_code)]
pub fn id(text: impl Display) -> String {
    paint(text, MAGENTA)
}

/// Prints a user-facing label for a file path or location.
#[allow(dead_code)]
pub fn path(text: impl Display) -> String {
    paint(text, CYAN)
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
