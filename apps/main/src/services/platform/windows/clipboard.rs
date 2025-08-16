use clipboard_win::{get_clipboard, set_clipboard, formats};

use rust_logger::*;

pub struct Clipboard {
  buffer: Option<String>,
}

impl Clipboard {
  pub fn backup() -> Clipboard {
    let buffer = Self::get_clipboard_text().or::<String>(Ok(None)).unwrap();

    Clipboard { buffer }
  }

  pub fn restore(&self) {
    if let Some(buffer) = &self.buffer {
      if let Err(_) = Self::set_clipboard_text(buffer) {
        warn!("Failed to restore clipboard buffer");
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
