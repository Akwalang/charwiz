use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum KeyboardLayoutEnum {
  Current,
  Next,
  Custom(String),
}
