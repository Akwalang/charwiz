use rdev::Key;
use serde::Deserialize;

use settings_core::structs::KeyboardSnapshot;

use crate::structs::KeyboardModifiersRaw;

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
