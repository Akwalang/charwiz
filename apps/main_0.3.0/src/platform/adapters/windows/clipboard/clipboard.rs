use clipboard_win::{get_clipboard, set_clipboard, formats};

use rust_logger::*;

pub struct Clipboard {
  buffer: Option<String>,
}

impl Clipboard {
  pub fn new() -> Self {
    Self { buffer: None }
  }

  pub fn backup(&mut self) {
    self.buffer = self.get_clipboard_text().or::<String>(Ok(None)).unwrap();
  }

  pub fn restore(&mut self) {
    let Some(buffer) = &self.buffer else {
      warn!("<$>Clipboard</>: Can't restore buffer: previous value is empty");
      return;
    };

    if let Err(e) = self.set_clipboard_text(buffer) {
      warn!("<$>Clipboard</>: Failed to restore clipboard buffer: {}", e);
    }
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
