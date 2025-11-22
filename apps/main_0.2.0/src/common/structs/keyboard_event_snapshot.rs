use rdev::Key;
use serde::{de::Error, Deserialize, Deserializer};

use crate::{common::structs::KeyboardModifiers, utils::{str_to_modifier, str_to_key}};

#[derive(Debug, Clone, Deserialize)]
pub struct KeyboardEventSnapshot {
  #[serde(deserialize_with = "deserialize_key")]
  pub key: Option<Key>,

  #[serde(deserialize_with = "deserialize_modifiers")]
  pub modifiers: KeyboardModifiers,
}

impl KeyboardEventSnapshot {
  pub fn new(key: Option<Key>, modifiers: KeyboardModifiers) -> Self {
    Self { key, modifiers }
  }
}

fn deserialize_key<'de, D>(deserializer: D) -> Result<Option<Key>, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;

  Ok(str_to_key(&s))
}

fn deserialize_modifiers<'de, D>(deserializer: D) -> Result<KeyboardModifiers, D::Error>
where
  D: Deserializer<'de>,
{
  let maybe_list = Option::<Vec<String>>::deserialize(deserializer)?;
 
  let list = maybe_list.unwrap_or_default();
 
  let mut result = KeyboardModifiers::new();

  for s in list {
    let Some(key) = str_to_modifier(&s) else {
      return Err(D::Error::custom(format!("unknown modifier key: {}", s)));
    };

    result.add_key(&key);
  }
 
  Ok(result)
}
