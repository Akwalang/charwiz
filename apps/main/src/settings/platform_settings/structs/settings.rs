use std::rc::Rc;

use serde::Deserialize;

use crate::common::structs::KeyboardSnapshot;
use crate::settings::raw::structs::KeyboardSnapshotRaw;

use super::clipboard_hotkeys::{ClipboardHotkeysRaw, ClipboardHotkeys};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsRaw {
  pub clipboard_hotkeys: ClipboardHotkeysRaw,
  pub banned_hotkeys: Vec<KeyboardSnapshotRaw>,
  pub switch_keyboard_layout_hotkeys: Vec<KeyboardSnapshotRaw>,
  pub stack_brake_hotkeys: Vec<KeyboardSnapshotRaw>,
}

#[derive(Debug, Default)]
pub struct Settings {
  pub clipboard_hotkeys: Rc<ClipboardHotkeys>,
  pub banned_hotkeys: Rc<Vec<KeyboardSnapshot>>,
  pub switch_keyboard_layout_hotkeys: Rc<Vec<KeyboardSnapshot>>,
  pub stack_brake_hotkeys: Rc<Vec<KeyboardSnapshot>>,
}

impl From<SettingsRaw> for Settings {
  fn from(raw: SettingsRaw) -> Self {
    Self {
      clipboard_hotkeys: Rc::new(raw.clipboard_hotkeys.into()),
      banned_hotkeys: Rc::new(raw.banned_hotkeys.into_iter().map(Into::into).collect()),
      switch_keyboard_layout_hotkeys: Rc::new(raw.switch_keyboard_layout_hotkeys.into_iter().map(Into::into).collect()),
      stack_brake_hotkeys: Rc::new(raw.stack_brake_hotkeys.into_iter().map(Into::into).collect()),
    }
  }
}
