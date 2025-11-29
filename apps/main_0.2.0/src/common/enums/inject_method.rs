use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InjectMethodEnum {
  Type,
  Paste,
  TypeAndPaste,
}
