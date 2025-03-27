extern crate copypasta;

use copypasta::{ClipboardContext, ClipboardProvider};

pub fn test_clipboard_context() {
  let mut ctx = ClipboardContext::new().unwrap();

  let msg = "Hello, world!";
  ctx.set_contents(msg.to_owned()).unwrap();
  let contents = ctx.get_contents().unwrap();

  println!("Clipboard contents: {}", contents);
}

pub fn get_clipboard_content() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
  let mut ctx = ClipboardContext::new()?;
  let contents = ctx.get_contents()?;
  Ok(contents)
}

pub fn set_clipboard_content(context_string: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let mut ctx = ClipboardContext::new()?;
  ctx.set_contents(context_string.to_owned())?;
  Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_clipboard_content() {
        let test_string = "Test clipboard content";

        // Test setting clipboard content
        let set_result = set_clipboard_content(test_string);
        assert!(set_result.is_ok(), "Failed to set clipboard content");
        // Assert the test_string is the same string as the set_result content
        assert_eq!(set_result.unwrap(), (), "Clipboard content mismatch");
        

        // Test getting clipboard content
        let get_result = get_clipboard_content();
        assert!(get_result.is_ok(), "Failed to get clipboard content");

        let clipboard_content = get_result.unwrap();
        assert_eq!(clipboard_content, test_string, "Clipboard content mismatch");
    }
}