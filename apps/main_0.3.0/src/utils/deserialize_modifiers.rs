use serde::de::{Error as DeError, Deserialize, Deserializer};

use rdev::Key;

use crate::common::structs::KeyboardModifiers;

fn str_to_modifier(s: &str) -> Option<Key> {
  match s {
    "Alt" => Some(Key::Alt),
    "AltGr" => Some(Key::AltGr),
    "ControlLeft" => Some(Key::ControlLeft),
    "ControlRight" => Some(Key::ControlRight),
    "ShiftLeft" => Some(Key::ShiftLeft),
    "ShiftRight" => Some(Key::ShiftRight),
    "MetaLeft" => Some(Key::MetaLeft),
    "MetaRight" => Some(Key::MetaRight),
    _ => None,
  }
}

pub fn deserialize_modifiers<'de, D>(deserializer: D) -> Result<KeyboardModifiers, D::Error>
where
  D: Deserializer<'de>,
{
  let maybe_list = Option::<Vec<String>>::deserialize(deserializer)?;
 
  let list = maybe_list.unwrap_or_default();
 
  let mut result = KeyboardModifiers::default();

  for s in list {
    let Some(key) = str_to_modifier(&s) else {
      return Err(D::Error::custom(format!("unknown modifier key: {}", s)));
    };

    result.add_key(&key);
  }
 
  Ok(result)
}
