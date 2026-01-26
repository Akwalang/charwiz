use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InjectMethodEnum {
  Paste,
  Emulate,
  TypeAndSkip,
  TypeAndPaste,
}
