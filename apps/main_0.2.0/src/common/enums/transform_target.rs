use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
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
