use rdev::Key;
use serde::Deserialize;

use crate::common::structs::KeyboardSnapshot;

use super::keyboard_modifiers_raw::KeyboardModifiersRaw;

#[derive(Deserialize)]
pub struct KeyboardSnapshotRaw {
  pub key: Option<Key>,
  pub modifiers: KeyboardModifiersRaw,
}

impl Into<KeyboardSnapshot> for KeyboardSnapshotRaw {
  fn into(self) -> KeyboardSnapshot {
    KeyboardSnapshot {
      key: self.key,
      modifiers: self.modifiers.into(),
    }
  }
}
