use clipboard_win::{get_clipboard, set_clipboard, formats};

pub struct Clipboard {
  buffer: Option<String>,
}

impl Clipboard {
  pub fn new() -> Clipboard {
    Clipboard { buffer: None }
  }

  pub fn save(&mut self) {
    self.buffer = Self::get_clipboard_text();
  }

  pub fn restore(&self) {
    if let Some(buffer) = &self.buffer {
      Self::set_clipboard_text(buffer);
    }
  }

  pub fn get_clipboard_text() -> Option<String> {
    match get_clipboard(formats::Unicode) {
      Ok(buffer) => Some(buffer),
      Err(_) => None,
    }
  }

  pub fn set_clipboard_text(text: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    match set_clipboard(formats::Unicode, text) {
      Ok(_) => Ok(()),
      Err(_) => Err(Box::new(std::io::Error::last_os_error())),
    }
  }
}
