extern crate copypasta;

use copypasta::{ClipboardContext, ClipboardProvider as CopypastaProvider};
use std::error::Error;
use std::sync::{Arc, Mutex};

// Trait for clipboard operations to allow mocking in tests
pub trait ClipboardProvider: Send + Sync {
    fn get_contents(&mut self) -> Result<String, Box<dyn Error + Send + Sync>>;
    fn set_contents(&mut self, content: String) -> Result<(), Box<dyn Error + Send + Sync>>;
}

// Real clipboard implementation using copypasta
pub struct SystemClipboard;

impl ClipboardProvider for SystemClipboard {
    fn get_contents(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut ctx = ClipboardContext::new()?;
        Ok(ctx.get_contents()?)
    }

    fn set_contents(&mut self, content: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut ctx = ClipboardContext::new()?;
        ctx.set_contents(content)?;
        Ok(())
    }
}

pub struct ClipboardManager<T: ClipboardProvider> {
    current_content: String,
    original_content: Option<String>,
    consecutive_errors: u32,
    provider: Arc<Mutex<T>>,
}

impl ClipboardManager<SystemClipboard> {
    // Create a new ClipboardManager instance with real clipboard
    pub fn new() -> Self {
        ClipboardManager {
            current_content: String::new(),
            original_content: None,
            consecutive_errors: 0,
            provider: Arc::new(Mutex::new(SystemClipboard)),
        }
    }
}

impl<T: ClipboardProvider> ClipboardManager<T> {
    // Create a new ClipboardManager instance with custom provider (for testing)
    pub fn with_provider(provider: T) -> Self {
        ClipboardManager {
            current_content: String::new(),
            original_content: None,
            consecutive_errors: 0,
            provider: Arc::new(Mutex::new(provider)),
        }
    }

    pub fn has_text_content(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        match self.provider.lock() {
            Ok(mut provider) => {
                match provider.get_contents() {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false), 
                }
            }
            Err(e) => Err(format!("Failed to lock clipboard provider: {}", e).into())
        }
    }

    // Get the current clipboard content
    pub fn get_clipboard_content(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut provider = self.provider.lock()
            .map_err(|e| format!("Failed to lock clipboard provider: {}", e))?;
        
        match provider.get_contents() {
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
                Err(e)
            }
        }
    }

    // Set the clipboard content and store the original
    pub fn set_clipboard_content(&mut self, context_string: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut provider = self.provider.lock()
            .map_err(|e| format!("Failed to lock clipboard provider: {}", e))?;
        
        provider.set_contents(context_string.to_owned())?;
        
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

    // Mock clipboard implementation for testing
    // Uses Mutex<String> instead of the real system clipboard to:
    // 1. Avoid modifying the user's actual clipboard during tests
    // 2. Enable deterministic, repeatable tests
    // 3. Allow tests to run in parallel without interference
    struct MockClipboard {
        content: Mutex<String>,
    }

    impl MockClipboard {
        fn new() -> Self {
            MockClipboard {
                content: Mutex::new(String::new()),
            }
        }

        fn with_content(content: &str) -> Self {
            MockClipboard {
                content: Mutex::new(content.to_string()),
            }
        }
    }

    impl ClipboardProvider for MockClipboard {
        fn get_contents(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
            Ok(self.content.lock().unwrap().clone())
        }

        fn set_contents(&mut self, content: String) -> Result<(), Box<dyn Error + Send + Sync>> {
            *self.content.lock().unwrap() = content;
            Ok(())
        }
    }

    #[test]
    fn test_new_clipboard_manager() {
        let manager = ClipboardManager::with_provider(MockClipboard::new());
        
        assert_eq!(manager.current_content, "");
        assert_eq!(manager.original_content, None);
        assert_eq!(manager.consecutive_errors, 0);
    }

    #[test]
    fn test_set_and_get_clipboard_content() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        let test_string = "Test clipboard content";

        // Test setting clipboard content
        let set_result = manager.set_clipboard_content(test_string);
        assert!(set_result.is_ok(), "Failed to set clipboard content");

        // Test getting clipboard content
        let get_result = manager.get_clipboard_content();
        assert!(get_result.is_ok(), "Failed to get clipboard content");

        let clipboard_content = get_result.unwrap();
        assert_eq!(clipboard_content, test_string, "Clipboard content mismatch");
    }

    #[test]
    fn test_original_content_tracking() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        
        // Initially, there should be no original content
        assert_eq!(manager.get_original_content(), None);

        // Set first content
        let first_content = "First clipboard content";
        manager.set_clipboard_content(first_content).unwrap();
        
        // Still no original content after first set
        assert_eq!(manager.get_original_content(), Some(String::new()));

        // Set second content
        let second_content = "Second clipboard content";
        manager.set_clipboard_content(second_content).unwrap();
        
        // Now original content should be the first content
        assert_eq!(manager.get_original_content(), Some(first_content.to_string()));
    }

    #[test]
    fn test_clipboard_change_detection() {
        let mock = MockClipboard::with_content("First clipboard content");
        let mut manager = ClipboardManager::with_provider(mock);
        let first_string = "First clipboard content";
        let second_string = "Second clipboard content";

        // Set initial content
        manager.set_clipboard_content(first_string).unwrap();
        
        // Manually change the mock clipboard content
        {
            let mut provider = manager.provider.lock().unwrap();
            provider.set_contents(second_string.to_string()).unwrap();
        }
        
        // Clipboard change detection should detect the change
        let has_changed = manager.has_clipboard_changed().unwrap();
        assert!(has_changed, "Clipboard change was not detected");

        // Test no change in clipboard
        let has_changed_again = manager.has_clipboard_changed().unwrap();
        assert!(!has_changed_again, "Clipboard incorrectly detected as changed");
    }

    #[test]
    fn test_clipboard_change_with_whitespace() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        
        // Set content with trailing whitespace
        manager.set_clipboard_content("test content  ").unwrap();
        
        // Manually set clipboard to same content with different whitespace
        {
            let mut provider = manager.provider.lock().unwrap();
            provider.set_contents("test content".to_string()).unwrap();
        }
        
        // Should not detect change because trimmed content is the same
        let has_changed = manager.has_clipboard_changed().unwrap();
        assert!(!has_changed, "Trimmed content should be considered equal");
    }

    #[test]
    fn test_minimal_string_clipboard() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        
        // Test with a minimal string
        let minimal_string = " ";
        
        // Test setting minimal string
        let result = manager.set_clipboard_content(minimal_string);
        assert!(result.is_ok(), "Should handle minimal string");
        
        // Get and verify minimal content
        let content = manager.get_clipboard_content().unwrap();
        assert_eq!(content, minimal_string);
    }

    #[test]
    fn test_empty_string_clipboard() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        
        // Test setting empty string (mock supports it)
        let result = manager.set_clipboard_content("");
        assert!(result.is_ok(), "Should handle empty string");
        
        // Get and verify empty content
        let content = manager.get_clipboard_content().unwrap();
        assert_eq!(content, "");
    }

    #[test]
    fn test_unicode_content() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        let unicode_string = "Hello 世界 🌍 مرحبا Привет";

        // Test setting unicode content
        let set_result = manager.set_clipboard_content(unicode_string);
        assert!(set_result.is_ok(), "Failed to set unicode clipboard content");

        // Test getting unicode content
        let get_result = manager.get_clipboard_content();
        assert!(get_result.is_ok(), "Failed to get unicode clipboard content");

        let clipboard_content = get_result.unwrap();
        assert_eq!(clipboard_content, unicode_string, "Unicode content mismatch");
    }

    #[test]
    fn test_multiline_content() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        let multiline_string = "Line 1\nLine 2\nLine 3\n\nLine 5";

        // Test setting multiline content
        let set_result = manager.set_clipboard_content(multiline_string);
        assert!(set_result.is_ok(), "Failed to set multiline clipboard content");

        // Test getting multiline content
        let content = manager.get_clipboard_content().unwrap();
        assert_eq!(content, multiline_string, "Multiline content mismatch");
    }

    #[test]
    fn test_long_content() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        let long_string = "a".repeat(10000);

        // Test setting long content
        let set_result = manager.set_clipboard_content(&long_string);
        assert!(set_result.is_ok(), "Failed to set long clipboard content");

        // Test getting long content
        let content = manager.get_clipboard_content().unwrap();
        assert_eq!(content, long_string, "Long content mismatch");
    }

    #[test]
    fn test_has_text_content() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        
        // Set some content
        manager.set_clipboard_content("test").unwrap();
        
        // Check if clipboard has text content
        let has_text = manager.has_text_content();
        assert!(has_text.is_ok(), "Failed to check for text content");
        assert!(has_text.unwrap(), "Should have text content");
    }

    #[test]
    fn test_consecutive_content_updates() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        
        // Set multiple contents in sequence
        let contents = vec!["First", "Second", "Third", "Fourth"];
        
        for (i, content) in contents.iter().enumerate() {
            manager.set_clipboard_content(content).unwrap();
            let retrieved = manager.get_clipboard_content().unwrap();
            assert_eq!(&retrieved, content, "Content mismatch at iteration {}", i);
        }
        
        // Original content should be "Third" (the one before "Fourth")
        assert_eq!(manager.get_original_content(), Some("Third".to_string()));
    }

    #[test]
    fn test_special_characters() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        let special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?`~\t\r\n";

        let result = manager.set_clipboard_content(special_chars);
        assert!(result.is_ok(), "Failed to set special characters");

        let content = manager.get_clipboard_content().unwrap();
        assert_eq!(content, special_chars, "Special characters mismatch");
    }

    #[test]
    fn test_url_content() {
        let mut manager = ClipboardManager::with_provider(MockClipboard::new());
        let url = "https://x.com/user/status/1234567890?t=abc&s=xyz";

        let result = manager.set_clipboard_content(url);
        assert!(result.is_ok(), "Failed to set URL content");

        let content = manager.get_clipboard_content().unwrap();
        assert_eq!(content, url, "URL content mismatch");
    }
}