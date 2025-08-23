use rdev::Key;

use crate::common::structs::KeyboardModifiers;

#[derive(Debug, Clone)]
pub struct KeyboardEventSnapshot {
  pub key: Key,
  pub modifiers: KeyboardModifiers,
}

impl KeyboardEventSnapshot {
  pub fn new(key: Key, modifiers: KeyboardModifiers) -> Self {
    Self { key, modifiers }
  }
}
