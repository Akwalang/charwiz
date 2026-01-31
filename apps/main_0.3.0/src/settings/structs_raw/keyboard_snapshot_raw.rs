use rdev::Key;
use serde::Deserialize;

use crate::common::structs::{KeyboardSnapshot, KeyboardModifiers};
use crate::utils::deserialize_modifiers;

#[derive(Debug, Deserialize)]
pub struct KeyboardSnapshotRaw {
  pub key: Option<Key>,
  #[serde(default, deserialize_with = "deserialize_modifiers")]
  pub modifiers: KeyboardModifiers,
}

impl Into<KeyboardSnapshot> for KeyboardSnapshotRaw {
  fn into(self) -> KeyboardSnapshot {
    KeyboardSnapshot { key: self.key, modifiers: self.modifiers }
  }
}
