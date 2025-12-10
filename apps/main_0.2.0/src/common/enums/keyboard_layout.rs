use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum KeyboardLayoutEnum {
  Previous,
  Current,
  Next,
  Direct(String),
}
