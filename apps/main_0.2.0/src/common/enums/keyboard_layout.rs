use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyboardLayoutEnum {
  Previous,
  Current,
  Next,
  Direct(String),
}
