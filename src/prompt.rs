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