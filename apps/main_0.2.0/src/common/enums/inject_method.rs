use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InjectMethodEnum {
  Type,
  Paste,
}
