use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserInputCleanupEnum {
  None,
  Backspace(u8),
}

impl Default for UserInputCleanupEnum {
  fn default() -> Self {
    UserInputCleanupEnum::None
  }
}
