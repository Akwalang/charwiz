use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::common::enums::KeyboardLayoutEnum;

#[derive(Debug, Clone, Serialize)]
pub enum KeyboardLayoutRawEnum {
  Previous,
  Current,
  Next,
  Direct(String),
}

impl Default for KeyboardLayoutRawEnum {
  fn default() -> Self {
    KeyboardLayoutRawEnum::Current
  }
}

impl FromStr for KeyboardLayoutRawEnum {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "previous"  => Ok(Self::Previous),
      "current"   => Ok(Self::Current),
      "next"      => Ok(Self::Next),
      other => Ok(Self::Direct(other.to_string())),
    }
  }
}

impl<'de> Deserialize<'de> for KeyboardLayoutRawEnum {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    let s = String::deserialize(deserializer)?;

    s.parse().map_err(serde::de::Error::custom)
  }
}

impl Into<KeyboardLayoutEnum> for KeyboardLayoutRawEnum {
  fn into(self) -> KeyboardLayoutEnum {
    match self {
      KeyboardLayoutRawEnum::Previous => KeyboardLayoutEnum::Previous,
      KeyboardLayoutRawEnum::Current  => KeyboardLayoutEnum::Current,
      KeyboardLayoutRawEnum::Next     => KeyboardLayoutEnum::Next,
      KeyboardLayoutRawEnum::Direct(value) => KeyboardLayoutEnum::Direct(value),
    }
  }
}