use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransformTarget {
  Input,
  Clipboard,
  Selection,
  Word,
  Line,
  All,
}
