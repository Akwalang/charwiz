use std::cmp::Ordering;

use rdev::Key;

use crate::common::structs::KeyboardModifiers;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct KeyboardEventSnapshot {
  pub key: Option<Key>,
  pub modifiers: KeyboardModifiers,
}

impl KeyboardEventSnapshot {
  pub const CAPACITY: usize = 9;

  pub fn new(key: Option<Key>, modifiers: KeyboardModifiers) -> Self {
    Self { key, modifiers }
  }

  pub fn len(&self) -> u8 {
    let mut length = self.modifiers.0.count_ones() as u8;

    if self.key.is_some() {
      length += 1;
    }

    length
  }
}

impl PartialOrd for KeyboardEventSnapshot {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for KeyboardEventSnapshot {
  fn cmp(&self, other: &Self) -> Ordering {
    self.len().cmp(&other.len())
  }
}
