//! Terminal utilities for colored output

use dialoguer::{Confirm, Input, Select};

/// Terminal helper for styled output
pub struct Terminal {
    use_color: bool,
}

impl Terminal {
    /// Create a new terminal helper
    pub fn new() -> Self {
        Self {
            use_color: atty::is(atty::Stream::Stdout),
        }
    }

    /// Print info message
    pub fn info(&self, msg: impl AsRef<str>) {
        println!("{}", msg.as_ref());
    }

    /// Print success message
    pub fn success(&self, msg: impl AsRef<str>) {
        if self.use_color {
            println!("{}", console::style(msg.as_ref()).green());
        } else {
            println!("[OK] {}", msg.as_ref());
        }
    }

    /// Print warning message
    pub fn warn(&self, msg: impl AsRef<str>) {
        if self.use_color {
            println!("{}", console::style(msg.as_ref()).yellow());
        } else {
            println!("[WARN] {}", msg.as_ref());
        }
    }

    /// Print error message
    pub fn error(&self, msg: impl AsRef<str>) {
        if self.use_color {
            eprintln!("{}", console::style(msg.as_ref()).red());
        } else {
            eprintln!("[ERROR] {}", msg.as_ref());
        }
    }

    /// Print a blank line
    #[allow(dead_code)]
    pub fn blank(&self) {
        println!();
    }

    /// Ask for confirmation
    pub fn confirm(&self, msg: &str) -> bool {
        Confirm::new()
            .with_prompt(msg)
            .default(false)
            .interact()
            .unwrap_or(false)
    }

    /// Ask for input
    #[allow(dead_code)]
    pub fn input(&self, prompt: &str) -> String {
        Input::new()
            .with_prompt(prompt)
            .interact_text()
            .unwrap_or_default()
    }

    /// Show a selection menu
    #[allow(dead_code)]
    pub fn select(&self, prompt: &str, items: &[&str]) -> usize {
        Select::new()
            .with_prompt(prompt)
            .items(&items.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .interact()
            .unwrap_or(0)
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}