use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransformTargetEnum {
  Command,
  Input,
  Clipboard,
  Selection,
  Word,
  Line,
  All,
}
