use serde::Deserialize;

use settings_core::structs::KeyboardModifiers;
use settings_core::settings::KeyInsert;

use crate::structs::KeyboardModifiersRaw;

#[derive(Deserialize)]
pub struct KeyInsertRaw {
  pub r#char: char,
  #[serde(default)]
  pub modifiers: KeyboardModifiersRaw,
}

impl Into<KeyInsert> for KeyInsertRaw {
  fn into(self) -> KeyInsert {
    KeyInsert {
      r#char: self.r#char,
      modifiers: self.modifiers.into(),
    }
  }
}
