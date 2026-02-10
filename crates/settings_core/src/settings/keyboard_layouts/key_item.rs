use rdev::Key;

use super::KeyInsert;

#[derive(Debug)]
pub struct KeyItem {
  pub key: Key,
  pub insert: Vec<KeyInsert>,
}
