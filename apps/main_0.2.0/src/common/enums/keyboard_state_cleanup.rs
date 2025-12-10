use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
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
