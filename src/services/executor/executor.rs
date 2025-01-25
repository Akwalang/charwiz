use crate::services::{keyboard, Clipboard, Platform};

use super::enums::Command;
use super::utils;

pub struct Executor {
  clipboard: Clipboard,
}

impl Executor {
  pub fn new() -> Self {
    let clipboard = Clipboard::new();

    Self { clipboard }
  }

  pub async fn apply(&mut self, cmd: Command) {
    match cmd {
      Command::SwitchLanguage => self.switch_language().await,
    }
  }

  async fn switch_language(&mut self) {
    self.clipboard.save();

    keyboard::select_word().await.unwrap();
    keyboard::copy().await.unwrap();

    let value = Clipboard::get_clipboard_text().unwrap();
    let value = utils::convert_language(value);

    Clipboard::set_clipboard_text(&value).unwrap();

    keyboard::paste().await.unwrap();
    keyboard::deselect().await.unwrap();

    self.clipboard.restore();
  }
}
