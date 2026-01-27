use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize)]
pub enum KeyboardLayoutEnum {
  Previous,
  Current,
  Next,
  Direct(String),
}

impl Default for KeyboardLayoutEnum {
  fn default() -> Self {
    KeyboardLayoutEnum::Current
  }
}

impl FromStr for KeyboardLayoutEnum {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "previous" => Ok(Self::Previous),
      "current" => Ok(Self::Current),
      "next" => Ok(Self::Next),
      other => Ok(Self::Direct(other.to_string())),
    }
  }
}

impl<'de> Deserialize<'de> for KeyboardLayoutEnum {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
  }
}
