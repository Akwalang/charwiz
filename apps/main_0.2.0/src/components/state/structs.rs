use rdev::Key;

use crate::common::structs::KeyboardModifiers;

#[derive(Debug, Clone)]
pub struct KeyboardEventSnapshot {
  pub key: Key,
  pub modifiers: KeyboardModifiers,
}
