use rdev::Key;

use crate::common::structs::KeyboardModifiers;

pub struct PrintCharCommand {
  key: Key,
  modifiers: KeyboardModifiers,
}

impl PrintCharCommand {
  pub fn new(key: Key, modifiers: KeyboardModifiers) -> Self {
    PrintCharCommand { key, modifiers }
  }
}
