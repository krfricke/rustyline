/// Custom prompt updater module for dynamic prompts

/// Custom prompt updater trait for dynamic prompts retrieves from external prints
pub trait PromptUpdater {
    /// Updates the prompt based on the provided line.
    fn update_prompt(&self, line: &str) -> Option<String> {
        let _line = line;
        None
    }
}
