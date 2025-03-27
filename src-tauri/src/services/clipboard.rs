extern crate copypasta;

use copypasta::{ClipboardContext, ClipboardProvider};

pub fn test_clipboard_context() {
  let mut ctx = ClipboardContext::new().unwrap();

  let msg = "Hello, world!";
  ctx.set_contents(msg.to_owned()).unwrap();
  let contents = ctx.get_contents().unwrap();

  println!("Clipboard contents: {}", contents);
}