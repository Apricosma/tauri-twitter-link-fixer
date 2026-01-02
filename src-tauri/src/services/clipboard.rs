extern crate copypasta;

use copypasta::{ClipboardContext, ClipboardProvider};
use std::error::Error;

pub struct ClipboardManager {
    current_content: String,
    original_content: Option<String>,
    consecutive_errors: u32,
}

impl ClipboardManager {
    // Create a new ClipboardManager instance
    pub fn new() -> Self {
        ClipboardManager {
            current_content: String::new(),
            original_content: None,
            consecutive_errors: 0,
        }
    }

    pub fn has_text_content(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        match ClipboardContext::new() {
            Ok(mut ctx) => {
                match ctx.get_contents() {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false), 
                }
            }
            Err(e) => Err(e.into())
        }
    }

    // Get the current clipboard content
    pub fn get_clipboard_content(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        match ClipboardContext::new()?.get_contents() {
            Ok(contents) => {
                self.consecutive_errors = 0;
                Ok(contents)
            }
            Err(e) => {
                self.consecutive_errors += 1;
                // If we've had too many errors, reset state
                if self.consecutive_errors > 5 {
                    self.current_content = String::new();
                    self.original_content = None;
                    self.consecutive_errors = 0;
                }
                Err(e.into())
            }
        }
    }

    // Set the clipboard content and store the original
    pub fn set_clipboard_content(&mut self, context_string: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut ctx = ClipboardContext::new()?;
        ctx.set_contents(context_string.to_owned())?;
        // Store the current content as original before updating
        self.original_content = Some(self.current_content.clone());
        self.current_content = context_string.to_owned();
        self.consecutive_errors = 0;
        Ok(())
    }

    // Get the original content that triggered the conversion
    pub fn get_original_content(&self) -> Option<String> {
        self.original_content.clone()
    }

    // Check if the clipboard content has changed
    pub fn has_clipboard_changed(&mut self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let clipboard_content = self.get_clipboard_content()?;
        
        if clipboard_content.trim() != self.current_content.trim() {
            self.current_content = clipboard_content;
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