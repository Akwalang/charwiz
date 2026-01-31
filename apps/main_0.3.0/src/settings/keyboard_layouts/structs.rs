use serde::Deserialize;
use rdev::Key;

use crate::common::structs::KeyboardModifiers;

use super::super::raw::structs::KeyboardModifiersRaw;

#[derive(Deserialize)]
pub struct KeyItemRaw {
  pub key: Key,
  pub insert: Vec<KeyInsertRaw>,
}

#[derive(Debug)]
pub struct KeyItem {
  pub key: Key,
  pub insert: Vec<KeyInsert>,
}

impl Into<KeyItem> for KeyItemRaw {
  fn into(self) -> KeyItem {
    KeyItem {
      key: self.key,
      insert: self.insert.into_iter().map(Into::into).collect(),
    }
  }
}


#[derive(Deserialize)]
pub struct KeyInsertRaw {
  pub r#char: char,
  #[serde(default)]
  pub modifiers: KeyboardModifiersRaw,
}

#[derive(Debug)]
pub struct KeyInsert {
  pub r#char: char,
  pub modifiers: KeyboardModifiers,
}

impl Into<KeyInsert> for KeyInsertRaw {
  fn into(self) -> KeyInsert {
    KeyInsert {
      r#char: self.r#char,
      modifiers: self.modifiers.into(),
    }
  }
}
