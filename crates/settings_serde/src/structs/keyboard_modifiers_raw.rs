use serde::{Deserialize, Serialize};

use rdev::Key;

use settings_core::structs::KeyboardModifiers;

#[derive(Clone, Deserialize, Serialize)]
pub struct KeyboardModifiersRaw(Vec<String>);

impl Default for KeyboardModifiersRaw {
  fn default() -> Self {
    KeyboardModifiersRaw(vec![])
  }
}

fn str_to_modifier(s: &str) -> Option<Key> {
  match s {
    "Alt"          => Some(Key::Alt),
    "AltGr"        => Some(Key::AltGr),
    "ControlLeft"  => Some(Key::ControlLeft),
    "ControlRight" => Some(Key::ControlRight),
    "ShiftLeft"    => Some(Key::ShiftLeft),
    "ShiftRight"   => Some(Key::ShiftRight),
    "MetaLeft"     => Some(Key::MetaLeft),
    "MetaRight"    => Some(Key::MetaRight),
    _              => None,
  }
}

impl Into<KeyboardModifiers> for KeyboardModifiersRaw {
  fn into(self) -> KeyboardModifiers {
    let mut result = KeyboardModifiers::default();

    for str in self.0 {
      let Some(key) = str_to_modifier(&str) else {
        panic!("Unknown modifier key: {}", str);
      };

      result.add_key(key);
    }

    result
  }
}
