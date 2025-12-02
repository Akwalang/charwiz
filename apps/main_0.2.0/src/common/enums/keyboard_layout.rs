use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum KeyboardLayoutEnum {
  Direct(String),
  Previous,
  Current,
  Next,
}
