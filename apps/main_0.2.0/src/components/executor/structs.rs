use std::collections::HashSet;

use rdev::Key;

pub struct PrintCharCommand {
  key: Key,
  modifiers: HashSet<Key>,
}

impl PrintCharCommand {
  pub fn new(key: Key, modifiers: HashSet<Key>) -> Self {
    PrintCharCommand { key, modifiers }
  }
}
