use std::fmt::Display;
use std::io::IsTerminal;

const RESET: &str = "\x1b[0m";
const GREEN_BOLD: &str = "\x1b[1;32m";
const YELLOW_BOLD: &str = "\x1b[1;33m";
const RED_BOLD: &str = "\x1b[1;31m";
const CYAN_BOLD: &str = "\x1b[1;36m";
const BLUE_BOLD: &str = "\x1b[1;34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const DIM: &str = "\x1b[2m";

fn color_enabled(is_tty: bool) -> bool {
    // Respect the common "don't color" signal.
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }

    // If a user explicitly forces color, honor it even when output is piped.
    if std::env::var_os("CLICOLOR_FORCE").is_some() {
        return true;
    }

    // Avoid ANSI noise when output is redirected.
    is_tty && std::env::var("TERM").map_or(true, |term| term != "dumb")
}

fn paint(text: impl Display, code: &str, enabled: bool) -> String {
    let text = text.to_string();

    if enabled {
        format!("{code}{text}{RESET}")
    } else {
        text
    }
}

pub fn success(text: impl Display) -> String {
    paint(text, GREEN_BOLD, color_enabled(std::io::stdout().is_terminal()))
}

pub fn warning(text: impl Display) -> String {
    paint(text, YELLOW_BOLD, color_enabled(std::io::stdout().is_terminal()))
}

pub fn error(text: impl Display) -> String {
    paint(text, RED_BOLD, color_enabled(std::io::stderr().is_terminal()))
}

pub fn heading(text: impl Display) -> String {
    paint(text, CYAN_BOLD, color_enabled(std::io::stdout().is_terminal()))
}

pub fn key(text: impl Display) -> String {
    paint(text, BLUE_BOLD, color_enabled(std::io::stdout().is_terminal()))
}

pub fn id(text: impl Display) -> String {
    paint(text, MAGENTA, color_enabled(std::io::stdout().is_terminal()))
}

pub fn path(text: impl Display) -> String {
    paint(text, CYAN, color_enabled(std::io::stdout().is_terminal()))
}

pub fn dim(text: impl Display) -> String {
    paint(text, DIM, color_enabled(std::io::stdout().is_terminal()))
}

pub fn status_ok(text: impl Display) -> String {
    paint(text, GREEN_BOLD, color_enabled(std::io::stdout().is_terminal()))
}

pub fn status_warn(text: impl Display) -> String {
    paint(text, YELLOW_BOLD, color_enabled(std::io::stdout().is_terminal()))
}

pub fn status_bad(text: impl Display) -> String {
    paint(text, RED_BOLD, color_enabled(std::io::stdout().is_terminal()))
}
