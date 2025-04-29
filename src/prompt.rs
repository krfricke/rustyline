use std::sync::Arc;

use arc_swap::ArcSwapAny;

/// Prompt struct that contains the current prompt
#[derive(Debug, Clone)]
pub struct Prompt(Arc<ArcSwapAny<Arc<String>>>);

impl Prompt {
    /// Create a new prompt
    pub fn new(prompt: String) -> Self {
        Self(Arc::new(ArcSwapAny::new(Arc::new(prompt))))
    }

    /// Set the prompt
    pub fn set_prompt(&self, prompt: String) {
        self.0.store(Arc::new(prompt.into()));
    }

    /// Swap the current prompt with a new one and return the old one
    pub fn swap(&self, prompt: String) -> Arc<String> {
        self.0.swap(Arc::new(prompt))
    }

    /// Get the current prompt
    pub fn get_prompt(&self) -> Arc<String> {
        self.0.load().clone()
    }
}

/// Update prompt with last unfinished line of the input
pub fn update_prompt(prompt: &Prompt, line: String) -> String {
    let mut lines = line.lines();
    let last_line = match lines.next_back() {
        Some(line) => line,
        None => {
            prompt.set_prompt("".to_string());
            return String::new();
        }
    }
    .to_string();

    // get first lines
    let mut first_lines = String::new();

    let previous_prompt = prompt.swap(last_line.clone());
    first_lines.push_str(previous_prompt.as_ref());

    for line in lines {
        first_lines.push_str(line);
        first_lines.push('\n');
    }

    first_lines
}
