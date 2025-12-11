use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransformTargetEnum {
  None,
  Input,
  Events,
  Command,
  Clipboard,
  Selection,
  Word,
  Line,
  All,
}
