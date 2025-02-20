use std::collections::HashMap;

use crate::services::config::file::TomlFile;

use crate::services::config::settings::{
  SettingsDelay,
  SettingsAction,
  SettingsKeyboardLayout,
};

#[derive(Debug)]
pub struct Settings {
  pub delay: SettingsDelay,
  pub keyboard_layouts: HashMap<String, SettingsKeyboardLayout>,
  pub actions: Vec<SettingsAction>,
}

impl From<TomlFile> for Settings {
  fn from(file: TomlFile) -> Self {
    let delay = file.delay.into();

    let keyboard_layouts = file.keyboard_layouts.into_iter()
      .map(|(key, value)| { (key, value.into()) })
      .collect();

    let actions = file.actions.into_iter()
      .map(|action| { action.into() })
      .collect();

    Self { delay, keyboard_layouts, actions }
  }
}
