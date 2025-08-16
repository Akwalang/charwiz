use std::collections::HashSet;
use std::error::Error;

use rust_logger::*;

use rdev::Key;

use crate::services::transformer::Transformer;
use crate::services::keyboard::KeyboardEmulator;
use crate::services::platform::{Clipboard, KeyboardLayout, Platform, PlatformTrait};

use crate::services::config::settings::{SettingsAction, SettingsActionTarget};
use crate::services::config::Config;

pub struct Executor {
  transformer: Transformer,
}

impl Executor {
  pub fn new() -> Self {
    let transformer = Transformer::new();

    Self { transformer }
  }

  pub fn copy_diagnostic(diagnostic: String) {
    Clipboard::set_clipboard_text(&diagnostic).unwrap();
    log!("Diagnostic data <green>copied to clipboard</>: {}", diagnostic);
  }

  pub fn find_action(keys: HashSet<Key>) -> Option<SettingsAction> {
    let config = Config::get_instance();
    let actions = config.get_actions();

    let action = actions.iter().find(|action| {
      if action.keys.len() != keys.len() { return false; }

      action.keys.iter().all(|key| keys.contains(key))
    });

    if let Some(action) = action {
      Some(action.clone())
    } else {
      None
    }
  }

  pub async fn execute_action(&self, action: &SettingsAction) -> Result<(), Box<dyn Error + 'static>> {
    let backup = Clipboard::backup();

    let emulator = KeyboardEmulator::new();

    Self::prepare_selection(&emulator, action).await?;
    Self::copy_to_clipboard(&emulator, action).await?;
    Self::replace(&emulator, &self.transformer, action).await?;

    backup.restore();

    Ok(())
  }

  async fn prepare_selection(emulator: &KeyboardEmulator, action: &SettingsAction) -> Result<(), Box<dyn Error + 'static>> {
    match action.target {
      SettingsActionTarget::All => emulator.select_all().await?,
      SettingsActionTarget::Line => emulator.select_line().await?,
      SettingsActionTarget::Word => emulator.select_word().await?,
      _ => (),
    }

    Ok(())
  }

  async fn copy_to_clipboard(emulator: &KeyboardEmulator, action: &SettingsAction) -> Result<(), Box<dyn Error>> {
    match action.target {
      SettingsActionTarget::Clipboard => (),
      SettingsActionTarget::None => (),
      _ => emulator.copy().await?,
    }

    Ok(())
  }

  async fn replace(
    emulator: &KeyboardEmulator,
    transformer: &Transformer,
    action: &SettingsAction,
  ) -> Result<(), Box<dyn Error>> {
    let (kbl_before, kbl_after) = Self::switch_keyboard_layout(action).await?;

    let income = Self::get_value(action)?;
    let outcome = transformer.execute(income, action, &kbl_before, &kbl_after)?;

    Clipboard::set_clipboard_text(&outcome)?;
    emulator.paste().await?;

    if action.keep_selection {
      emulator.select_chars(outcome.chars().count()).await?;
    }

    Ok(())
  }

  async fn switch_keyboard_layout(action: &SettingsAction) -> Result<(KeyboardLayout, KeyboardLayout), Box<dyn Error>> {
    let platform = Platform::get_instance();

    let before = platform.get_current_keyboard_layout().clone();

    let after = if action.switch_keyboard_layout {
      let after = platform.switch_keyboard_layout()?;

      after.unwrap_or(before.clone())
    } else {
      before.clone()
    };

    Ok((before, after))
  }

  fn get_value(action: &SettingsAction) -> Result<String, Box<dyn Error>> {
    match action.target {
      SettingsActionTarget::None => { return Ok("".to_string()); },
      _ => (),
    }

    let value = Clipboard::get_clipboard_text()?;

    if value.is_none() { return Ok("".to_string()); }

    Ok(value.unwrap())
  }
}
