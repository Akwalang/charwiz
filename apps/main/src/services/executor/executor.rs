use std::collections::HashSet;

use rdev::Key;

use crate::services::{keyboard, KeyboardLayout, Config, ActionConfig, ActionTarget, Clipboard, Plugins, Platform, PlatformTrait};

use super::utils;

pub struct Executor {
  platform: Platform,
  clipboard: Clipboard,
  plugins: Plugins,
}

impl Executor {
  pub fn new() -> Self {
    let platform = Platform::new();
    let clipboard = Clipboard::new();
    let plugins = Plugins::new();

    println!("");
    platform.log_state();
    println!("");

    Self { platform, clipboard, plugins }
  }

  pub fn find_action(keys: HashSet<Key>) -> Option<&'static ActionConfig> {
    let actions = Config::get_actions();

    actions.iter().find(|action| {
      if action.keys.len() != keys.len() { return false; }

      action.keys.iter().all(|key| keys.contains(key))
    })
  }

  pub async fn apply(&mut self, action: &ActionConfig) -> Result<(), Box<dyn std::error::Error>> {
    self.clipboard.save();

    self.prepare_selection(action).await?;
    self.copy_to_clipboard(action).await?;
    self.use_transformation(action).await?;

    self.clipboard.restore();

    Ok(())
  }

  async fn prepare_selection(&self, action: &ActionConfig) -> Result<(), Box<dyn std::error::Error>> {
    match action.target {
      ActionTarget::All => keyboard::utils::select_all().await?,
      ActionTarget::Line => keyboard::utils::select_line().await?,
      ActionTarget::Word => keyboard::utils::select_word().await?,
      _ => (),
    }

    Ok(())
  }

  async fn copy_to_clipboard(&self, action: &ActionConfig) -> Result<(), Box<dyn std::error::Error>> {
    match action.target {
      ActionTarget::Clipboard => (),
      _ => keyboard::utils::copy().await?,
    }

    Ok(())
  }

  async fn use_transformation(&self, action: &ActionConfig) -> Result<(), Box<dyn std::error::Error>> {
    let (kbl_before, kbl_after) = self.switch_keyboard_layout().await?;

    let value = self.transform_clipboard(action, kbl_before, kbl_after)?;

    keyboard::utils::paste().await?;

    if action.keep_selection { keyboard::utils::select_chars(value.chars().count()).await?; }

    Ok(())
  }

  async fn switch_keyboard_layout(&self) -> Result<(&KeyboardLayout, &KeyboardLayout), Box<dyn std::error::Error>> {
    let before = self.platform.get_current_keyboard_layout();

    self.platform.switch_keyboard_layout()?;

    let after = self.platform.get_current_keyboard_layout();

    Ok((before, after))
  }

  fn transform_clipboard(
    &self,
    action: &ActionConfig,
    kbl_before: &KeyboardLayout,
    kbl_after: &KeyboardLayout,
  ) -> Result<String, Box<dyn std::error::Error>> {
    let value = Clipboard::get_clipboard_text()?;

    if value.is_none() { return Ok("".to_string()); }

    let value = self.transform_value(value.unwrap(), action, kbl_before, kbl_after)?;

    Clipboard::set_clipboard_text(&value)?;

    Ok(value)
  }

  fn transform_value(
    &self,
    value: String,
    action: &ActionConfig,
    kbl_before: &KeyboardLayout,
    kbl_after: &KeyboardLayout,
  ) -> Result<String, Box<dyn std::error::Error>> {
    let result = self.apply_handler(&value, action, kbl_before, kbl_after)?;

    if result.is_some() { return Ok(result.unwrap()); }

    let result = self.apply_plugin(&value, action, kbl_before, kbl_after)?;

    if result.is_some() { return Ok(result.unwrap()); }

    Ok(value)
  }

  fn apply_handler(
    &self,
    value: &str,
    action: &ActionConfig,
    kbl_before: &KeyboardLayout,
    kbl_after: &KeyboardLayout,
  ) -> Result<Option<String>, Box<dyn std::error::Error>> {
    match action.handler.as_str() {
      "convert_char_layout" => Ok(Some(utils::convert_char_layout(value, kbl_before, kbl_after))),
      "invert_case" => Ok(Some(utils::invert_case(value, kbl_before, kbl_after))),
      _ => Ok(None),
    }
  }

  fn apply_plugin(
    &self,
    value: &str,
    action: &ActionConfig,
    kbl_before: &KeyboardLayout,
    kbl_after: &KeyboardLayout,
  ) -> Result<Option<String>, Box<dyn std::error::Error>> {
    self.plugins
      .run(value, action, kbl_before, kbl_after)
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
  }
}
