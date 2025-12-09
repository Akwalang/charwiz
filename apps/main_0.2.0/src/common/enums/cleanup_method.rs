use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupMethodEnum {
  None,
  Backspace(u8),
}

impl Default for CleanupMethodEnum {
  fn default() -> Self {
    CleanupMethodEnum::None
  }
}
