use serde::Deserialize;

use crate::common::structs::{KeyboardEventSnapshot, KeyboardModifiers};

use crate::settings::main_settings::structs::{Executor, Injector};

use crate::utils::{str_to_key, str_to_modifier};

#[derive(Debug, Deserialize)]
pub struct HotkeyRaw {
  pub keys: HotkeyKeysRaw,
  pub executor: Executor,
  pub injector: Injector,
}

#[derive(Debug, Deserialize)]
pub struct HotkeyKeysRaw {
  pub key: Option<String>,
  pub modifiers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HotKey {
  pub keys: KeyboardEventSnapshot,
  pub executor: Executor,
  pub injector: Injector,
}

impl Into<HotKey> for HotkeyRaw {
  fn into(self) -> HotKey {
    let key = self.keys.key.and_then(|key| str_to_key(&key));

    let mut modifiers = KeyboardModifiers::default();

    for mod_str in self.keys.modifiers {
      if let Some(key) = str_to_modifier(&mod_str) {
        modifiers.add_key(&key);
      }
    }

    HotKey {
      keys: KeyboardEventSnapshot{ key, modifiers },
      executor: self.executor,
      injector: self.injector,
    }
  }
}
