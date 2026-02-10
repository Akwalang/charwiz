use serde::Deserialize;

use rdev::Key;

use settings_core::settings::keyboard_layouts::KeyItem;

use super::KeyInsertRaw;

#[derive(Deserialize)]
pub struct KeyItemRaw {
  pub key: Key,
  pub insert: Vec<KeyInsertRaw>,
}

impl Into<KeyItem> for KeyItemRaw {
  fn into(self) -> KeyItem {
    KeyItem {
      key: self.key,
      insert: self.insert.into_iter().map(Into::into).collect(),
    }
  }
}
