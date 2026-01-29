use x11_clipboard::Clipboard as X11Clipboard;

use rust_logger::*;

pub struct Clipboard {
  clipboard: X11Clipboard,
  buffer: Option<String>,
}

impl Clipboard {
  pub fn new() -> Self {
    let clipboard = X11Clipboard::new().expect("Failed to initialize X11 clipboard");

    Self {
      clipboard,
      buffer: None,
    }
  }

  pub fn backup(&mut self) {
    self.buffer = self.get_clipboard_text().or::<Option<String>>(Ok(None)).unwrap();
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
    let Ok(bytes) = self.clipboard.load(
      self.clipboard.getter.atoms.clipboard,
      self.clipboard.getter.atoms.utf8_string,
      self.clipboard.getter.atoms.property,
      std::time::Duration::from_millis(200),
    ) else {
      return Ok(None);
    };

    Ok(Some(String::from_utf8_lossy(&bytes).to_string()))
  }

  pub fn set_clipboard_text(&self, text: &str) -> anyhow::Result<()> {
    self.clipboard.store(
      self.clipboard.getter.atoms.clipboard,
      self.clipboard.getter.atoms.utf8_string,
      text.as_bytes().to_vec(),
    );

    Ok(())
  }
}