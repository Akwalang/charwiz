use serde::Deserialize;
use rdev::Key;

use crate::common::structs::KeyboardModifiers;
use crate::utils::deserialize_modifiers;

#[derive(Debug, Deserialize)]
pub struct KeyItem {
  pub key: Key,
  pub insert: Vec<KeyInsert>,
}

#[derive(Debug, Deserialize)]
pub struct KeyInsert {
  pub r#char: char,

  #[serde(default, deserialize_with = "deserialize_modifiers")]
  pub modifiers: KeyboardModifiers,
}
