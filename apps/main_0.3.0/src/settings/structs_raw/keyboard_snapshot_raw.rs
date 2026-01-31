use rdev::Key;
use serde::Deserialize;

use crate::common::structs::{KeyboardSnapshot, KeyboardModifiers};
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct KeyboardSnapshotRaw {
  pub key: Option<Key>,
  #[serde(default, deserialize_with = "utils::deserialize_modifiers")]
  pub modifiers: KeyboardModifiers,
}

impl Into<KeyboardSnapshot> for KeyboardSnapshotRaw {
  fn into(self) -> KeyboardSnapshot {
    // let mut modifiers = KeyboardModifiers::default();

    // for mod_str in self.modifiers {
    //   if let Some(key) = utils::str_to_modifier(&mod_str) {
    //     modifiers.add_key(&key);
    //   }
    // }

    KeyboardSnapshot { key: self.key, modifiers: self.modifiers }
  }
}
