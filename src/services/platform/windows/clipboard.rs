use clipboard_win::{get_clipboard, set_clipboard, formats};

pub struct Clipboard {
  buffer: Option<String>,
}

impl Clipboard {
  pub fn new() -> Clipboard {
    Clipboard { buffer: None }
  }

  pub fn save(&mut self) {
    let result = Self::get_clipboard_text();

    match result {
      Ok(buffer) => self.buffer = buffer,
      Err(_) => println!("Failed to save clipboard buffer"),
    }
  }

  pub fn restore(&self) {
    if let Some(buffer) = &self.buffer {
      if let Err(_) = Self::set_clipboard_text(buffer) {
        println!("Failed to restore clipboard buffer");
      }
    }
  }

  pub fn get_clipboard_text() -> Result<Option<String>, Box<dyn std::error::Error + 'static>> {
    match get_clipboard(formats::Unicode) {
      Ok(buffer) => Ok(Some(buffer)),
      Err(_) => Err(Box::new(std::io::Error::last_os_error())),
    }
  }

  pub fn set_clipboard_text(text: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    match set_clipboard(formats::Unicode, text) {
      Ok(_) => Ok(()),
      Err(_) => Err(Box::new(std::io::Error::last_os_error())),
    }
  }
}
