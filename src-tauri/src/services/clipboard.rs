extern crate copypasta;

use copypasta::{ClipboardContext, ClipboardProvider};
use std::error::Error;

pub struct ClipboardManager {
    previous_content: String,
}

impl ClipboardManager {
    // Create a new ClipboardManager instance
    pub fn new() -> Self {
        ClipboardManager {
            previous_content: String::new(),
        }
    }

    // Get the current clipboard content
    pub fn get_clipboard_content(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut ctx = ClipboardContext::new()?;
        let contents = ctx.get_contents()?;
        Ok(contents)
    }

    // Set the clipboard content
    pub fn set_clipboard_content(&mut self, context_string: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut ctx = ClipboardContext::new()?;
        ctx.set_contents(context_string.to_owned())?;
        Ok(())
    }

    // Check if the clipboard content has changed
    pub fn has_clipboard_changed(&mut self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let current_content = self.get_clipboard_content()?;
        if current_content != self.previous_content {
            self.previous_content = current_content;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_manager() {
        let mut clipboard_manager = ClipboardManager::new();

        let test_string = "Test clipboard content";

        // Test setting clipboard content
        let set_result = clipboard_manager.set_clipboard_content(test_string);
        assert!(set_result.is_ok(), "Failed to set clipboard content");

        // Test getting clipboard content
        let get_result = clipboard_manager.get_clipboard_content();
        assert!(get_result.is_ok(), "Failed to get clipboard content");

        let clipboard_content = get_result.unwrap();
        assert_eq!(clipboard_content, test_string, "Clipboard content mismatch");

        // Test clipboard change detection
        let has_changed = clipboard_manager.has_clipboard_changed().unwrap();
        assert!(has_changed, "Clipboard change was not detected");

        // Test no change in clipboard
        let has_changed_again = clipboard_manager.has_clipboard_changed().unwrap();
        assert!(!has_changed_again, "Clipboard incorrectly detected as changed");
    }
}