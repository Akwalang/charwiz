use serde::Deserialize;

use crate::common::structs::{KeyboardSnapshot, KeyboardModifiers};
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct KeyboardSnapshotRaw {
  pub key: Option<String>,
  pub modifiers: Vec<String>,
}

impl Into<KeyboardSnapshot> for KeyboardSnapshotRaw {
  fn into(self) -> KeyboardSnapshot {
    let key = self.key.and_then(|key| utils::str_to_key(&key));

    let mut modifiers = KeyboardModifiers::default();

    for mod_str in self.modifiers {
      if let Some(key) = utils::str_to_modifier(&mod_str) {
        modifiers.add_key(&key);
      }
    }

    KeyboardSnapshot { key, modifiers }
  }
}
