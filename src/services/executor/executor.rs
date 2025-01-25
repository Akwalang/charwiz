use crate::services::{keyboard, Clipboard, Platform, PlatformTrait};

use super::enums::Command;
use super::utils;

pub struct Executor {
  platform: Platform,
  clipboard: Clipboard,
}

impl Executor {
  pub fn new() -> Self {
    let platform = Platform::new();
    let clipboard = Clipboard::new();

    println!("");
    platform.log_state();
    println!("");

    Self { platform, clipboard }
  }

  pub async fn apply(&mut self, cmd: Command) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
      Command::SwitchLanguage => self.switch_language().await?,
    }

    Ok(())
  }

  async fn switch_language(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    self.platform.switch_keyboard_layout()?;

    self.clipboard.save();

    keyboard::utils::select_word().await?;
    keyboard::utils::copy().await?;

    let value = Clipboard::get_clipboard_text()?;

    if value.is_none() { return Ok(()); }

    let value = utils::convert_language(value.unwrap());

    Clipboard::set_clipboard_text(&value)?;

    keyboard::utils::paste().await?;

    self.clipboard.restore();

    Ok(())
  }
}
