use serde::Deserialize;

use crate::common::enums::KeyboardStateCleanupEnum;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardStateCleanupRawEnum {
  None,
  Drop,
}

impl Default for KeyboardStateCleanupRawEnum {
  fn default() -> Self {
    KeyboardStateCleanupRawEnum::None
  }
}

impl Into<KeyboardStateCleanupEnum> for KeyboardStateCleanupRawEnum {
  fn into(self) -> KeyboardStateCleanupEnum {
    match self {
      KeyboardStateCleanupRawEnum::None => KeyboardStateCleanupEnum::None,
      KeyboardStateCleanupRawEnum::Drop => KeyboardStateCleanupEnum::Drop,
    }
  }
}
