use std::sync::OnceLock;
use rdev::Key;

use super::enums::HotKeyAction;
use super::structs::HotKey;

static HOT_KEYS: OnceLock<Vec<HotKey>> = OnceLock::new();

fn init_get_hotkeys() -> Vec<HotKey> {
  vec![
    HotKey::new(
      vec![Key::ControlLeft, Key::Alt, Key::ShiftLeft],
      HotKeyAction::SwitchLanguage,
    ),
  ]
}

pub fn get_hotkeys() -> &'static Vec<HotKey> {
  HOT_KEYS.get_or_init(init_get_hotkeys)
}
