use std::sync::OnceLock;
use rdev::Key;

use super::enums::HotKeyAction;
use super::structs::HotKey;

static HOT_KEYS: OnceLock<Vec<HotKey>> = OnceLock::new();

fn init_get_hot_keys() -> Vec<HotKey> {
  vec![
    HotKey::new(
      vec![Key::ControlLeft, Key::Alt, Key::ShiftLeft],
      HotKeyAction::ToggleLanguage,
    ),
  ]
}

pub fn get_hot_keys() -> &'static Vec<HotKey> {
  HOT_KEYS.get_or_init(init_get_hot_keys)
}
