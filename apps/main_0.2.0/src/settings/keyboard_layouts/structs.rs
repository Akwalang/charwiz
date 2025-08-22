use std::collections::HashSet;

use rdev::Key;

use serde::de::{Error as DeError, Deserializer};
use serde::Deserialize;

use crate::utils::str_to_key;

#[derive(Debug, Deserialize)]
pub struct KeyItem {
  #[serde(deserialize_with = "deserialize_key")]
  pub key: Key,

  #[serde(default)]
  pub invertible: bool,

  pub insert: Vec<KeyInsert>,
}

#[derive(Debug, Deserialize)]
pub struct KeyInsert {
  pub r#char: char,

  #[serde(default, deserialize_with = "deserialize_modifiers")]
  pub modifiers: HashSet<Key>,
}

fn deserialize_key<'de, D>(deserializer: D) -> Result<Key, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;

  let Some(key) = str_to_key(&s) else {
    return Err(D::Error::custom(format!("unknown key: {}", s)));
  };

  Ok(key)
}

fn deserialize_modifiers<'de, D>(deserializer: D) -> Result<HashSet<Key>, D::Error>
where
  D: Deserializer<'de>,
{
  let maybe_list = Option::<Vec<String>>::deserialize(deserializer)?;
 
  let list = maybe_list.unwrap_or_default();
 
  let mut result = HashSet::with_capacity(list.len());
 
  for s in list {
    let Some(key) = str_to_key(&s) else {
      return Err(D::Error::custom(format!("unknown modifier key: {}", s)));
    };

    result.insert(key);
  }
 
  Ok(result)
}
