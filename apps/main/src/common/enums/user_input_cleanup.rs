#[derive(Debug, Clone, PartialEq)]
pub enum UserInputCleanupEnum {
  None,
  Backspace(u8),
}
