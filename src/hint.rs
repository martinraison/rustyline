//! Hints (suggestions at the right of the prompt as you type).

/// Hints provider
pub trait Hinter {
    /// Takes the currently edited `line` with the cursor `pos`ition and
    /// returns the string that should be displayed or `None`
    /// if no hint is available for the text the user currently typed.
    fn hint(&mut self, line: &str, pos: usize) -> Option<String>;
}
