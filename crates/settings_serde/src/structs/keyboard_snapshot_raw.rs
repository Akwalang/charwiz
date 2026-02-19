use rdev::Key;
use serde::{Deserialize, Serialize};

use settings_core::structs::KeyboardSnapshot;

use crate::structs::KeyboardModifiersRaw;

#[derive(Clone, Deserialize, Serialize)]
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
