use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardStateCleanupEnum {
  None,
  Drop,
}

impl Default for KeyboardStateCleanupEnum {
  fn default() -> Self {
    KeyboardStateCleanupEnum::None
  }
}
