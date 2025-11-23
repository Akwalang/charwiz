use serde::{de::Error, Deserialize, Deserializer};
use rdev::Key;

use crate::common::enums::TransformTarget;
use crate::common::structs::{KeyboardEventSnapshot, KeyboardModifiers};

use crate::settings::main_settings::structs::Executor;

use crate::utils::{str_to_key, str_to_modifier};

#[derive(Debug, Deserialize)]
pub struct HotkeyRaw {
  pub keys: HotkeyKeysRaw,
  pub target: TransformTarget,
  pub executor: Executor,
}

#[derive(Debug, Deserialize)]
pub struct HotkeyKeysRaw {
  pub key: Option<String>,
  pub modifiers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Hotkey {
  pub keys: KeyboardEventSnapshot,
  pub target: TransformTarget,
  pub executor: Executor,
}

impl Into<Hotkey> for HotkeyRaw {
  fn into(self) -> Hotkey {
    let key = self.keys.key.and_then(|key| str_to_key(&key));

    let mut modifiers = crate::common::structs::KeyboardModifiers::new();

    for mod_str in self.keys.modifiers {
      if let Some(key) = str_to_modifier(&mod_str) {
        modifiers.add_key(&key);
      }
    }

    Hotkey {
      keys: KeyboardEventSnapshot{ key, modifiers },
      target: self.target,
      executor: self.executor,
    }
  }
}
