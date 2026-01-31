use serde::Deserialize;
use rdev::Key;

use crate::utils;
use crate::common::structs::KeyboardModifiers;

#[derive(Debug, Deserialize)]
pub struct KeyItem {
  pub key: Key,
  pub insert: Vec<KeyInsert>,
}

#[derive(Debug, Deserialize)]
pub struct KeyInsert {
  pub r#char: char,

  #[serde(default, deserialize_with = "utils::deserialize_modifiers")]
  pub modifiers: KeyboardModifiers,
}
