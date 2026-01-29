use serde::Deserialize;

use crate::settings::structs_raw::KeyboardSnapshotRaw;

use super::clipboard_hotkeys::{ClipboardHotkeysRaw, ClipboardHotkeys};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsRaw {
  pub clipboard_hotkeys: ClipboardHotkeysRaw,
  pub banned_hotkeys: Vec<KeyboardSnapshotRaw>,
  pub switch_keyboard_layout_hotkeys: Vec<KeyboardSnapshotRaw>,
  pub stack_breake_hotkeys: Vec<KeyboardSnapshotRaw>,
}

#[derive(Default)]
pub struct Settings {
  pub clipboard_hotkeys: ClipboardHotkeys,
  pub banned_hotkeys: Vec<KeyboardSnapshotRaw>,
  pub switch_keyboard_layout_hotkeys: Vec<KeyboardSnapshotRaw>,
  pub stack_breake_hotkeys: Vec<KeyboardSnapshotRaw>,
}

impl From<SettingsRaw> for Settings {
  fn from(raw: SettingsRaw) -> Self {
    Self {
      clipboard_hotkeys: raw.clipboard_hotkeys.into(),
      banned_hotkeys: raw.banned_hotkeys,
      switch_keyboard_layout_hotkeys: raw.switch_keyboard_layout_hotkeys,
      stack_breake_hotkeys: raw.stack_breake_hotkeys,
    }
  }
}
