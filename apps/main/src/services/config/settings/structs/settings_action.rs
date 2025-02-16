use rdev::Key;

use crate::services::config::file::TomlAction;
use crate::services::config::settings::SettingsActionTarget;
use crate::services::config::utils::str_to_key;

#[derive(Debug, Clone)]
pub struct SettingsAction {
  pub keys: Vec<Key>,
  pub target: SettingsActionTarget,
  pub switch_keyboard_layout: bool,
  pub keep_selection: bool,
  pub handler: String,
}

impl Into<SettingsAction> for TomlAction {
  fn into(self) -> SettingsAction {
    let keys = self.keys.into_iter()
      .map(|key| { str_to_key(&key).unwrap() })
      .collect();

    let target: SettingsActionTarget = self.target.into();

    SettingsAction {
      keys,
      target,
      switch_keyboard_layout: self.switch_keyboard_layout,
      keep_selection: self.keep_selection,
      handler: self.handler,
    }
  }
}
