use serde::Deserialize;

use settings_core::enums::UserInputCleanupEnum;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserInputCleanupRawEnum {
  None,
  Backspace(u8),
}

impl Default for UserInputCleanupRawEnum {
  fn default() -> Self {
    UserInputCleanupRawEnum::None
  }
}

impl Into<UserInputCleanupEnum> for UserInputCleanupRawEnum {
  fn into(self) -> UserInputCleanupEnum {
    match self {
      UserInputCleanupRawEnum::None          => UserInputCleanupEnum::None,
      UserInputCleanupRawEnum::Backspace(n)  => UserInputCleanupEnum::Backspace(n),
    }
  }
}
