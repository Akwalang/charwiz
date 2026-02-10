use std::rc::Rc;

use serde::Deserialize;

use settings_core::structs::KeyboardSnapshot;
use settings_core::settings::platform_settings::Settings;

use super::ClipboardHotkeysRaw;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsRaw {
  pub clipboard_hotkeys: ClipboardHotkeysRaw,
  pub banned_hotkeys: Vec<KeyboardSnapshotRaw>,
  pub switch_keyboard_layout_hotkeys: Vec<KeyboardSnapshotRaw>,
  pub stack_brake_hotkeys: Vec<KeyboardSnapshotRaw>,
}

impl Into<Settings> for SettingsRaw {
  fn into(self) -> Settings {
    Settings {
      clipboard_hotkeys: Rc::new(self.clipboard_hotkeys.into()),
      banned_hotkeys: Rc::new(self.banned_hotkeys.into_iter().map(Into::into).collect()),
      switch_keyboard_layout_hotkeys: Rc::new(self.switch_keyboard_layout_hotkeys.into_iter().map(Into::into).collect()),
      stack_brake_hotkeys: Rc::new(self.stack_brake_hotkeys.into_iter().map(Into::into).collect()),
    }
  }
}
