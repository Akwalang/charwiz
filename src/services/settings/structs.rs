use std::collections::HashSet;
use rdev::Key;

use super::enums::HotKeyAction;

#[derive(Debug)]
pub struct HotKey {
  keys: HashSet<Key>,
  pub action: HotKeyAction,
}

impl HotKey {
  pub fn new(keys: Vec<Key>, action: HotKeyAction) -> HotKey {
    let keys: HashSet<Key> = keys.into_iter().collect();

    HotKey { keys, action }
  }

  pub fn is_matching(&self, keys: &HashSet<Key>) -> bool {
    if self.keys.len() != keys.len() {
      return false;
    }

    self.keys.iter().all(|key| keys.contains(key))
  }
}
