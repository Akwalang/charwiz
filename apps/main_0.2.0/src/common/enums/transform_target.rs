use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransformTarget {
  Input,
  Clipboard,
  Selection,
  Word,
  Line,
  All,
}
