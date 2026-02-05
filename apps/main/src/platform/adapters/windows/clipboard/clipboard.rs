use clipboard_win::{get_clipboard, set_clipboard, formats};

use rust_logger::*;
use zeroize::Zeroize;

pub struct Clipboard {
  buffer: Option<Vec<u8>>,
}

impl Clipboard {
  pub fn new() -> Self {
    Self { buffer: None }
  }

  pub fn backup(&mut self) {
    let Ok(value) = self.get_clipboard_text() else {
      warn!("<$>Clipboard</>: Can't backup clipboard value");
      return;
    };

    let Some(value) = value else {
      warn!("<$>Clipboard</>: No clipboard value to backup");
      return;
    };

    self.buffer = Some(value.into_bytes());
  }

  pub fn restore(&mut self) {
    let Some(buffer) = &self.buffer else {
      warn!("<$>Clipboard</>: Can't restore buffer: previous value is empty");
      return;
    };

    let value = String::from_utf8_lossy(buffer);

    if let Err(e) = self.set_clipboard_text(&value) {
      warn!("<$>Clipboard</>: Failed to restore clipboard buffer: {}", e);
    }

    self.zeroize_buffer();
  }

  fn zeroize_buffer(&mut self) {
    if let Some(ref mut bytes) = self.buffer {
        bytes.zeroize();
        bytes.clear();
        bytes.shrink_to_fit();
    }

    self.buffer = None;
  }

  pub fn get_clipboard_text(&self) -> anyhow::Result<Option<String>> {
    let Ok(buffer) = get_clipboard(formats::Unicode) else {
      anyhow::bail!("Failed to get clipboard text");
    };

    Ok(Some(buffer))
  }

  pub fn set_clipboard_text(&self, text: &str) -> anyhow::Result<()> {
    if let Err(_) = set_clipboard(formats::Unicode, text) {
      anyhow::bail!("Failed to set clipboard text")
    }

    Ok(())
  }
}

impl Drop for Clipboard {
  fn drop(&mut self) {
    self.restore();
  }
}
